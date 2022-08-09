use num::{
    traits::{Signed, Zero},
    Integer,
};
use std::ops::{Mul, Rem};

pub fn square<T>(a: T) -> T
where
    T: Mul<Output = T> + Copy,
{
    a * a
}

pub fn cube<T>(x: T) -> T
where
    T: Mul<Output = T> + Copy,
{
    x * x * x
}

pub fn abs<T>(x: T) -> T
where
    T: Signed + Zero + PartialOrd<T>,
{
    if x > T::zero() {
        return x;
    }
    return -x;
}

pub fn is_even<T>(n: T) -> bool
where
    T: Rem<Output = T> + Integer + From<u8>,
{
    n % T::from(2u8) == T::from(0u8)
}
