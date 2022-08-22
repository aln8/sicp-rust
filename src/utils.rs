use num::traits::{Signed, Zero};
use std::ops::{BitAnd, Mul};

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
    T: BitAnd<Output = T> + From<u8> + PartialEq,
{
    n & T::from(1) != T::from(1)
}
