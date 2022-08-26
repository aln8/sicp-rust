use num::traits::{Signed, Zero};
use std::ops::{Add, BitAnd, Div, Mul};

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

pub fn power<T>(x: T, mut n: u32) -> T
where
    T: Mul<Output = T> + From<i8> + Copy,
{
    let mut res = x;
    if n == 0 {
        return T::from(1);
    }

    while n > 1 {
        res = res * x;
        n = n - 1;
    }
    res
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

pub fn average<T>(a: T, b: T) -> T
where
    T: Add<Output = T> + Div<Output = T> + From<i8>,
{
    (a + b) / T::from(2)
}
