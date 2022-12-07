use std::{
    any::Any,
    fmt::Debug,
    marker::{Destruct, PhantomData},
    mem,
    ptr::{self, NonNull},
};

pub trait ConsAny: Any + Debug {
    fn as_ref_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
    fn dyn_eq(&self, other: &dyn ConsAny) -> bool;
}

impl PartialEq for dyn ConsAny {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other)
    }
}

impl<T> PartialEq<T> for dyn ConsAny
where
    T: Debug + PartialEq + 'static,
{
    fn eq(&self, other: &T) -> bool {
        self.dyn_eq(other)
    }
}

impl Eq for dyn ConsAny {}

impl<T> ConsAny for T
where
    T: Debug + PartialEq + 'static,
{
    fn as_ref_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn dyn_eq(&self, other: &dyn ConsAny) -> bool {
        if let Some(other) = other.as_ref_any().downcast_ref::<Self>() {
            return other.eq(self);
        }
        false
    }
}

impl dyn ConsAny {
    fn cast_ref<T: 'static>(&self) -> Option<&T> {
        self.as_ref_any().downcast_ref()
    }

    fn cast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.as_mut_any().downcast_mut::<T>()
    }
}

type Link = Option<NonNull<dyn ConsAny>>;

#[derive(Debug)]
pub struct Cons {
    pub car: Link,
    pub cdr: Link,
    marker: PhantomData<Box<dyn ConsAny>>,
}

impl Cons {
    pub fn new(car: Option<Box<dyn ConsAny>>, cdr: Option<Box<dyn ConsAny>>) -> Cons {
        Cons {
            car: car.map(|car| Box::leak(car).into()),
            cdr: cdr.map(|cdr| Box::leak(cdr).into()),
            marker: PhantomData,
        }
    }

    // since all cons holds Box<dyn ConsAny>, should cast as box first
    pub fn car_ref(&self) -> Option<&dyn ConsAny> {
        self.car.map(|car| unsafe { &*car.as_ptr() })
    }

    pub fn car_mut(&mut self) -> Option<&mut dyn ConsAny> {
        self.car.map(|car| unsafe { &mut *car.as_ptr() })
    }

    pub fn set_car<T: ConsAny>(&mut self, car: Option<T>) -> Option<Box<dyn ConsAny>> {
        // clean self.car
        let re = self.car();
        self.car = car.map(|car| Box::leak(Box::new(car) as Box<dyn ConsAny>).into());
        re
    }

    pub fn car(&mut self) -> Option<Box<dyn ConsAny>> {
        self.car.map(|car| unsafe {
            self.car = None;
            Box::from_raw(car.as_ptr())
        })
    }

    pub fn car_downcast<T: Any>(&mut self) -> Option<Box<T>> {
        match self.car() {
            Some(car) => match (car as Box<dyn Any>).downcast() {
                Ok(car) => Some(car),
                Err(_) => None,
            },
            None => None,
        }
    }

    pub fn car_downcast_ref<T: Any>(&self) -> Option<&T> {
        match self.car {
            Some(car) => unsafe { (*car.as_ptr()).cast_ref() },
            None => None,
        }
    }

    pub fn car_downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        match self.car {
            Some(car) => unsafe { (*car.as_ptr()).cast_mut() },
            None => None,
        }
    }

    pub fn cdr_ref(&self) -> Option<&dyn ConsAny> {
        self.cdr.map(|cdr| unsafe { &*cdr.as_ptr() })
    }

    pub fn cdr_mut(&mut self) -> Option<&mut dyn ConsAny> {
        self.cdr.map(|cdr| unsafe { &mut *cdr.as_ptr() })
    }

    pub fn set_cdr<T: ConsAny>(&mut self, cdr: Option<T>) -> Option<Box<dyn ConsAny>> {
        // clean self cdr
        let re = self.cdr();
        self.cdr = cdr.map(|cdr| Box::leak(Box::new(cdr) as Box<dyn ConsAny>).into());
        re
    }

    pub fn cdr(&mut self) -> Option<Box<dyn ConsAny>> {
        self.cdr.map(|cdr| unsafe {
            self.cdr = None;
            Box::from_raw(cdr.as_ptr())
        })
    }

    pub fn cdr_downcast<T: Any>(&mut self) -> Option<Box<T>> {
        match self.cdr() {
            Some(cdr) => match (cdr as Box<dyn Any>).downcast() {
                Ok(cdr) => Some(cdr),
                Err(_) => None,
            },
            None => None,
        }
    }

    pub fn cdr_downcast_ref<T: Any>(&self) -> Option<&T> {
        match self.cdr {
            Some(cdr) => unsafe { (*cdr.as_ptr()).cast_ref() },
            None => None,
        }
    }

    pub fn cdr_downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        match self.cdr {
            Some(cdr) => unsafe { (*cdr.as_ptr()).cast_mut() },
            None => None,
        }
    }
}

impl Default for Cons {
    fn default() -> Self {
        Self::new(None, None)
    }
}

impl Drop for Cons {
    fn drop(&mut self) {
        if let Some(car) = self.car() {
            drop(car)
        }

        if let Some(cdr) = self.cdr() {
            drop(cdr)
        }
    }
}

impl PartialEq<Cons> for Cons {
    fn eq(&self, other: &Cons) -> bool {
        fn _eq(a: Option<&dyn ConsAny>, b: Option<&dyn ConsAny>) -> bool {
            match a {
                Some(a) => {
                    if let Some(b) = b {
                        return a.dyn_eq(b);
                    }
                    false
                }
                None => {
                    if b.is_none() {
                        return true;
                    }
                    false
                }
            }
        }
        _eq(self.car_ref(), other.car_ref()) && _eq(self.cdr_ref(), other.cdr_ref())
    }
}

#[derive(PartialEq, Debug)]
pub struct List {
    pub head: Cons,
}

impl List {
    pub fn new<T: ConsAny>(car: T) -> Self {
        Self {
            head: Cons::new(Some(Box::new(car)), None),
        }
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
        for item in iter {
            let mut new_tail = Self::new(item);
            tail.set_cdr(Some(new_tail));
            tail = tail.cdr_mut().unwrap();
        }
    }
}

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
    fn basic_cons() {
        let list = Cons::new(Some(Box::new(1)), Some(Box::new(2)));
        assert_eq!(list.car_ref().unwrap(), &1);
        assert_eq!(list.cdr_ref().unwrap(), &2);

        let list2 = Cons::new(Some(Box::new(list)), Some(Box::new(3)));
        let l1 = list2.car_ref().unwrap().cast_ref::<Cons>().unwrap();
        assert_eq!(l1.car_ref().unwrap(), &1);
        assert_eq!(list2.cdr_ref().unwrap(), &3);
    }

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
