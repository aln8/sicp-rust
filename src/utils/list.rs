use std::{any::Any, fmt::Debug, marker::PhantomData, mem};

use super::cons::{Cons, ConsAny, Link};

#[derive(PartialEq, Debug)]
pub struct List {
    head: Cons,
}

impl List {
    pub fn new<T: ConsAny>(mut car: T) -> Self {
        // here need to handle if T is Box<dyn ConsAny>
        // because if that, the result will be Box<Box<dyn ConsAny>>
        // which will cause weird behavior, such as dyn_eq fail for T & Box<T>
        if car
            .as_ref_any()
            .downcast_ref::<Box<dyn ConsAny>>()
            .is_some()
        {
            let a = (Box::new(car) as Box<dyn Any>)
                .downcast::<Box<dyn ConsAny>>()
                .unwrap();
            return Self {
                head: Cons::new(Some(*a), None),
            };
        }
        Self {
            head: Cons::new(Some(Box::new(car)), None),
        }
    }

    pub fn len(&self) -> usize {
        let mut len = 1;
        let mut list = self;
        while list.cdr_ref().is_some() {
            len += 1;
            list = list.cdr_ref().unwrap()
        }
        len
    }

    pub fn take(&mut self) -> Self {
        mem::replace(self, Self::default())
    }

    pub fn car<T: ConsAny + 'static>(&mut self) -> Option<T> {
        self.head.car_downcast().map(|car| *car)
    }

    pub fn car_ref<T: ConsAny + 'static>(&self) -> Option<&T> {
        self.head.car_downcast_ref()
    }

    pub fn car_mut<T: ConsAny + 'static>(&mut self) -> Option<&mut T> {
        self.head.car_downcast_mut()
    }

    pub fn set_car<T: ConsAny + 'static>(&mut self, car: T) {
        self.head.set_car(Some(car));
    }

    pub fn cdr(&mut self) -> Option<Self> {
        self.head.cdr_downcast().map(|cdr| *cdr)
    }

    pub fn cdr_ref(&self) -> Option<&Self> {
        self.head.cdr_downcast_ref()
    }

    pub fn cdr_mut(&mut self) -> Option<&mut Self> {
        self.head.cdr_downcast_mut()
    }

    pub fn tail(&mut self) -> &mut Self {
        let mut list = self;
        while list.cdr_mut().is_some() {
            list = list.cdr_mut().unwrap()
        }
        list
    }

    pub fn reverse(mut self) -> Self {
        if self.cdr_ref().is_none() {
            return self;
        }
        let next = self.cdr().unwrap();
        let mut head = next.reverse();
        head.tail().set_cdr(Some(self));
        head
    }

    pub fn set_cdr(&mut self, cdr: Option<Self>) -> Option<Self> {
        let a = self.head.set_cdr(cdr);
        match a {
            Some(list) => match (list as Box<dyn Any>).downcast::<Self>() {
                Ok(list) => Some(*list),
                Err(_) => None,
            },
            None => None,
        }
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter {
            car: self.head.car,
            cdr: self.head.cdr,
            marker: PhantomData,
        }
    }

    pub fn iter_downcast<T>(&self) -> IterDowncast<'_, T> {
        IterDowncast {
            car: self.head.car,
            cdr: self.head.cdr,
            marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut {
            car: self.head.car,
            cdr: self.head.cdr,
            marker: PhantomData,
        }
    }

    pub fn into_iter_downcast<T>(self) -> IntoIterDowncast<T> {
        IntoIterDowncast {
            next: self,
            marker: PhantomData,
        }
    }
}

impl Default for List {
    fn default() -> Self {
        Self {
            head: Cons::default(),
        }
    }
}

impl IntoIterator for List {
    type IntoIter = IntoIter;
    type Item = Box<dyn ConsAny>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { next: self }
    }
}

impl<'a> IntoIterator for &'a List {
    type IntoIter = Iter<'a>;
    type Item = &'a dyn ConsAny;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut List {
    type IntoIter = IterMut<'a>;
    type Item = &'a mut dyn ConsAny;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T: ConsAny> FromIterator<T> for List {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut dummy = Self::new(0);
        dummy.extend(iter);
        dummy.cdr().unwrap()
    }
}

impl<T: ConsAny> Extend<T> for List {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let mut tail = self.tail();
        let mut new_tail: List;
        for mut item in iter {
            new_tail = Self::new(item);
            tail.set_cdr(Some(new_tail));
            tail = tail.cdr_mut().unwrap();
        }
    }
}

#[derive(Debug)]

pub struct Iter<'a> {
    car: Link,
    cdr: Link,
    marker: PhantomData<&'a dyn ConsAny>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a dyn ConsAny;
    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.car.map(|car| unsafe { &*car.as_ptr() });
        match self.cdr {
            Some(cdr) => unsafe {
                match (&*cdr.as_ptr()).as_ref_any().downcast_ref::<List>() {
                    Some(next) => {
                        self.car = next.head.car;
                        self.cdr = next.head.cdr;
                        return cur;
                    }
                    None => {}
                }
            },
            None => {}
        };
        self.car = None;
        self.cdr = None;
        cur
    }
}

#[derive(Debug)]

pub struct IterDowncast<'a, T> {
    car: Link,
    cdr: Link,
    marker: PhantomData<&'a T>,
}

impl<'a, T: 'static> Iterator for IterDowncast<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let cur = match self.car {
            Some(car) => unsafe { &*car.as_ptr() }.as_ref_any().downcast_ref::<T>(),
            None => None,
        };

        match self.cdr {
            Some(cdr) => unsafe {
                match (&*cdr.as_ptr()).as_ref_any().downcast_ref::<List>() {
                    Some(next) => {
                        self.car = next.head.car;
                        self.cdr = next.head.cdr;
                        return cur;
                    }
                    None => {}
                }
            },
            None => {}
        };
        self.car = None;
        self.cdr = None;
        cur
    }
}

#[derive(Debug)]
pub struct IterMut<'a> {
    car: Link,
    cdr: Link,
    marker: PhantomData<&'a mut List>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut dyn ConsAny;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.car.map(|car| unsafe { &mut *car.as_ptr() });
        match self.cdr {
            Some(cdr) => unsafe {
                match (&*cdr.as_ptr()).as_ref_any().downcast_ref::<List>() {
                    Some(next) => {
                        self.car = next.head.car;
                        self.cdr = next.head.cdr;
                        return cur;
                    }
                    None => {}
                }
            },
            None => {}
        };
        self.car = None;
        self.cdr = None;
        cur
    }
}

#[derive(Debug)]
pub struct IntoIter {
    next: List,
}

impl Iterator for IntoIter {
    type Item = Box<dyn ConsAny>;
    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.next.head.car();
        match self.next.cdr() {
            Some(list) => {
                self.next = list;
            }
            _ => {}
        }
        cur
    }
}

#[derive(Debug)]
pub struct IntoIterDowncast<T> {
    next: List,
    marker: PhantomData<T>,
}

impl<T: 'static> Iterator for IntoIterDowncast<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let cur = match self.next.head.car() {
            Some(car) => match (car as Box<dyn Any>).downcast::<T>() {
                Ok(car) => Some(*car),
                Err(_) => None,
            },
            None => None,
        };
        // let cur = self.next.head.car();
        match self.next.cdr() {
            Some(list) => {
                self.next = list;
            }
            _ => {}
        }
        cur
    }
}

#[macro_export]
macro_rules! list {
    ( $first:expr ) => (
        List::new($first)
    );

    ( $first:expr, $( $a:expr ),* ) => {
        {
            let mut head = List::new($first);
            let mut cur = &mut head;
            $(
                let next = List::new($a);
                cur.set_cdr(Some(next));
                cur = cur.cdr_mut().unwrap();
            )*
            head
        }
    };
}

mod test {
    use super::*;

    #[test]
    fn basic_list() {
        let mut list = list!(1, 2, 3, 4);
        let mut iter = list.iter();
        assert_eq!(iter.next().unwrap(), &1);
        assert_eq!(iter.next().unwrap(), &2);
        assert_eq!(iter.next().unwrap(), &3);
        assert_eq!(iter.next().unwrap(), &4);
        assert_eq!(iter.next(), None);

        let mut iter_mut = list.iter_mut();
        assert_eq!(iter_mut.next().unwrap(), &1);
        assert_eq!(iter_mut.next().unwrap(), &2);
        assert_eq!(iter_mut.next().unwrap(), &3);
        assert_eq!(iter_mut.next().unwrap(), &4);
        assert_eq!(iter_mut.next(), None);

        let mut into = list.into_iter();
        assert_eq!(*into.next().unwrap(), 1);
        assert_eq!(*into.next().unwrap(), 2);
        assert_eq!(*into.next().unwrap(), 3);
        assert_eq!(*into.next().unwrap(), 4);
        assert_eq!(into.next(), None);
    }
}
