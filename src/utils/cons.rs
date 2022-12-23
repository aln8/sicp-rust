use std::{any::Any, fmt::Debug, marker::PhantomData, ptr::NonNull};

pub trait ConsAny: Any + Debug {
    fn dyn_clone(&self) -> Box<dyn ConsAny>;
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
    T: Clone + Debug + PartialEq + 'static,
{
    fn eq(&self, other: &T) -> bool {
        self.dyn_eq(other)
    }
}

impl Eq for dyn ConsAny {}

impl Clone for Box<dyn ConsAny> {
    fn clone(&self) -> Self {
        (&**self).dyn_clone()
    }
}

impl<T> ConsAny for T
where
    T: Debug + Clone + PartialEq + 'static,
{
    fn dyn_clone(&self) -> Box<dyn ConsAny> {
        Box::new(self.clone())
    }

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
    pub fn cast_ref<T: 'static>(&self) -> Option<&T> {
        self.as_ref_any().downcast_ref()
    }

    pub fn cast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.as_mut_any().downcast_mut()
    }
}

pub type Link = Option<NonNull<dyn ConsAny>>;

#[derive(Debug, Clone)]
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
            if a.is_none() && b.is_none() {
                return true;
            }
            if a.is_none() || b.is_none() {
                return false;
            }
            a.unwrap().dyn_eq(b.unwrap())
        }
        _eq(self.car_ref(), other.car_ref()) && _eq(self.cdr_ref(), other.cdr_ref())
    }
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
}
