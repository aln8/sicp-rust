use num::Signed;

use crate::utils::*;
use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

// return a closure and used by next item
fn avg_damping<T>(f: impl Fn(T) -> T) -> impl Fn(T) -> T
where
    T: Add<Output = T> + Div<Output = T> + From<i8> + Copy,
{
    move |x: T| -> T { average(x, f(x)) }
}

#[test]
fn test_avg_damping() {
    assert_eq!(55, avg_damping(square)(10));
    assert_eq!(15, avg_damping(square)(5));
}

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

// sqrt x = n/x, since this equation guess will only n/x, or x
// do it with average damping HOC
fn sqrt(n: i32) -> f32 {
    fixed_point(avg_damping(|x: f32| n as f32 / x), 1.0)
}

// cube root: x = n/x^2
fn cube_rt(n: i32) -> f32 {
    fixed_point(avg_damping(|x| n as f32 / square(x)), 1.0)
}

#[test]
fn test_sqrt() {
    assert_eq!(1.4142135, sqrt(2));
    assert_eq!(1.7320509, sqrt(3));
}

#[test]
fn test_cube_rt() {
    assert_eq!(1.2599217, cube_rt(2));
    assert_eq!(1.44225, cube_rt(3));
}

fn deriv<'a, T>(g: &'a impl Fn(T) -> T) -> impl Fn(T) -> T + 'a
where
    T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<f32> + Copy,
{
    move |x: T| {
        let dx = T::from(0.00001);
        (g(x + dx) - g(x)) / dx
    }
}

fn newton_transform<T>(g: impl Fn(T) -> T) -> impl Fn(T) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<f32> + Copy,
{
    move |x: T| x - g(x) / deriv(&g)(x)
}

fn newton_method<T>(g: impl Fn(T) -> T, guess: T) -> T
where
    T: Signed + PartialOrd + From<f32> + Copy,
{
    fixed_point(newton_transform(&g), guess)
}

// newton method: g(x) = 0
// so sqrt n = x^2
//      -> g(x) = x^2 - n = 0
fn sqrt_nm(n: i32) -> f32 {
    newton_method(|x| square(x) - n as f32, 1.0)
}

#[test]
fn test_deriv_cube() {
    assert_eq!(75.00014999983857, deriv(&cube)(5.0));
    assert_eq!(27.000090000187356, deriv(&cube)(3.0));
}

#[test]
fn test_sqrt_nm() {
    assert_eq!(1.4142135, sqrt_nm(2));
    assert_eq!(1.7320514, sqrt_nm(3));
}
