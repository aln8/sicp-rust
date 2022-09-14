use num::Signed;

use crate::utils::ops::*;
use std::ops::{Add, Mul, Sub};

fn fixed_point<T, FT>(f: FT, mut now: T) -> T
where
    T: Signed + Copy + Sub<Output = T> + PartialOrd + From<f32>,
    FT: Fn(T) -> T,
{
    let tolerance: f32 = 0.000001;
    let close = |a: T, b: T| -> bool { abs(a - b) < T::from(tolerance) };
    let mut next = f(now);
    while !close(now, next) {
        now = next;
        next = f(now);
    }
    now
}

// golden ration (x+1)/x = x = phi
// so we can get x -> 1/x + 1
fn golden_ratio() -> f32 {
    fixed_point(|x| 1.0 + 1.0 / x, 1.0)
}

// sqrt x = n/x, since this equation guess will only n/x, or x
// average damping it to, x = 1/2 * (x + n/x)
fn sqrt(n: i32) -> f32 {
    fixed_point(|x: f32| average(x, n as f32 / x), 1.0)
}

// x^x = n. so x = log(n) / log(x)
fn x_to_x(n: i32) -> f32 {
    fixed_point(|x: f32| (n as f32).log10() / x.log10(), 1.1)
}

// x = log(n) / log(x)
// x = 1/2(log(n) / log(x) + x)
fn x_to_x_avg_damping(n: i32) -> f32 {
    fixed_point(|x: f32| average(x, (n as f32).log10() / x.log10()), 1.1)
}

#[test]
fn test_golden_ratio() {
    assert_eq!(1.6180344, golden_ratio());
}

#[test]
fn test_sqrt() {
    assert_eq!(1.4142135, sqrt(2));
    assert_eq!(1.7320509, sqrt(3));
}

#[test]
fn test_x_to_x() {
    assert_eq!(4.5555363, x_to_x(1000));
}

#[test]
fn test_x_to_x_avg_damping() {
    assert_eq!(4.5555363, x_to_x_avg_damping(1000));
}
