use std::{fmt::Debug, iter::FromIterator, mem::replace};

// simple cons rust implementation
#[derive(Debug)]
pub struct Cons<T, F> {
    cdr: F,
    car: T,
}

impl<T: Default, F: Default> Cons<T, F> {
    pub fn new(car: T, cdr: F) -> Cons<T, F> {
        Cons { car, cdr }
    }

    pub fn car(&mut self) -> T {
        replace(&mut self.car, T::default())
    }

    pub fn cdr(&mut self) -> F {
        replace(&mut self.cdr, F::default())
    }

    // pub fn cdr_swap(&mut self, cdr: F) -> F {
    //     replace(&mut self.cdr, cdr)
    // }

    pub fn cdr_ref(&self) -> &F {
        &self.cdr
    }

    pub fn cdr_mut(&mut self) -> &mut F {
        &mut self.cdr
    }

    pub fn set_cdr(&mut self, cdr: F) {
        self.cdr = cdr;
    }

    pub fn car_ref(&self) -> &T {
        &self.car
    }

    pub fn car_mut(&mut self) -> &mut T {
        &mut self.car
    }

    pub fn set_car(&mut self, car: T) {
        self.car = car
    }
}

#[test]
fn test_cons() {
    let mut pair = Cons::new(1, 2);
    assert_eq!(1, pair.car());
}

#[derive(Debug)]
pub enum List<T: Default> {
    Cons(Cons<T, Box<List<T>>>),
    Nil,
}

impl<T: Default> Default for List<T> {
    fn default() -> Self {
        List::Nil
    }
}

impl<T: Default> List<T> {
    pub fn new(car: T) -> Self {
        Self::Cons(Cons::new(car, Box::new(Self::Nil)))
    }

    pub fn car(self) -> Option<T> {
        if let List::Cons(mut cons) = self {
            Some(cons.car())
        } else {
            None
        }
    }

    pub fn car_ref(&self) -> Option<&T> {
        if let List::Cons(cons) = self {
            Some(cons.car_ref())
        } else {
            None
        }
    }

    pub fn car_mut(&mut self) -> Option<&mut T> {
        if let List::Cons(cons) = self {
            Some(cons.car_mut())
        } else {
            None
        }
    }

    pub fn set_car(&mut self, car: T) {
        if let List::Cons(cons) = self {
            cons.set_car(car)
        }
    }

    pub fn cdr(&mut self) -> List<T> {
        if let List::Cons(cons) = self {
            *cons.cdr()
        } else {
            List::Nil
        }
    }

    // pub fn cdr_swap(&mut self, cdr: List<T>) -> List<T> {
    //     if let List::Cons(cons) = self {
    //         *cons.cdr_swap(Box::new(cdr))
    //     } else {
    //         List::Nil
    //     }
    // }

    pub fn cdr_ref(&self) -> &List<T> {
        if let List::Cons(cons) = self {
            &**cons.cdr_ref()
        } else {
            &List::Nil
        }
    }

    pub fn cdr_mut(&mut self) -> &mut List<T> {
        if let List::Cons(cons) = self {
            &mut **cons.cdr_mut()
        } else {
            self
        }
    }

    pub fn set_cdr(&mut self, cdr: List<T>) {
        if let List::Cons(cons) = self {
            cons.set_cdr(Box::new(cdr));
        }
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
                cur.set_cdr(next);
                cur = cur.cdr_mut();
            )*
            head
        }
    };
}

impl<'a, T: 'a + Copy + Default> Iterator for &'a List<T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            List::Cons(cons) => {
                *self = cons.cdr_ref();
                return Some(cons.car_ref());
            }
            List::Nil => None,
        }
    }
}

impl<T: Copy + Default> Iterator for List<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            List::Cons(cons) => {
                let car = *cons.car_ref();
                *self = *cons.cdr();
                return Some(car);
            }
            List::Nil => None,
        }
    }
}

impl<T: Default> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let mut dummy = List::new(T::default());
        let mut last = &mut dummy;
        while let Some(car) = iter.next() {
            last.set_cdr(Self::new(car));
            last = last.cdr_mut();
        }
        dummy.cdr()
    }
}

#[test]
fn test_list_macro() {
    let a = list!(1, 2, 3, 4);
    assert_eq!(&1, a.car_ref().unwrap());
    assert_eq!(&2, a.cdr_ref().car_ref().unwrap());
    assert_eq!(&3, a.cdr_ref().cdr_ref().car_ref().unwrap());
    assert_eq!(&4, a.cdr_ref().cdr_ref().cdr_ref().car_ref().unwrap());
    assert!(a
        .cdr_ref()
        .cdr_ref()
        .cdr_ref()
        .cdr_ref()
        .car_ref()
        .is_none());
}

#[test]
fn test_list() {
    let test_list = [1, 3, 2, 4];
    let mut test_idx = 0;
    let mut head = list!(1, 2, 3);

    // direct loop
    while let List::Cons(mut cons) = head {
        assert_eq!(&test_list[test_idx], cons.car_ref());
        test_idx += 1;
        head = *cons.cdr();
    }

    // iteration method
    let mut test_idx = 0;
    for val in &head {
        assert_eq!(&test_list[test_idx], val);
        test_idx += 1;
    }
}
