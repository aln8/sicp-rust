use num::Signed;

use crate::utils::ops::*;
use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

fn fixed_point<T, FT>(f: FT, mut now: T) -> T
where
    T: Signed + Copy + Sub<Output = T> + PartialOrd + From<f32> + Debug,
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

fn fixed_point_of_transform<T>(
    g: Box<dyn Fn(T) -> T>,
    transform: impl Fn(Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T>,
    guess: T,
) -> T
where
    T: Signed + PartialOrd + From<f32> + Copy + Debug,
{
    fixed_point(transform(g), guess)
}

// return a closure and used by next item
fn avg_damping<T>(f: Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T>
where
    T: Add<Output = T> + Div<Output = T> + From<i8> + Copy + 'static,
{
    Box::new(move |x: T| -> T { average(x, f(x)) })
}

fn deriv<'a, T>(g: &'a Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T + 'a>
where
    T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<f32> + Copy + 'a,
{
    Box::new(move |x: T| {
        let dx = T::from(0.00001);
        (g(x + dx) - g(x)) / dx
    })
}

fn newton_transform<'a, T>(g: Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T>
where
    T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<f32> + Copy + 'static,
    &'a T: Div<T>,
{
    Box::new(move |x: T| {
        let dg = deriv(&g);
        x - g(x) / dg(x)
    })
}

// x -> n / x
fn sqrt(n: i32) -> f32 {
    fixed_point_of_transform(Box::new(move |x| n as f32 / x), avg_damping, 1.0)
}

#[test]
fn test_sqrt() {
    assert_eq!(1.4142135, sqrt(2));
    assert_eq!(1.7320509, sqrt(3));
}

// x -> x^2 - n
fn sqrt_nm(n: i32) -> f32 {
    fixed_point_of_transform(
        Box::new(move |x| square(x) - n as f32),
        newton_transform,
        1.0,
    )
}

#[test]
fn test_sqrt_nm() {
    assert_eq!(1.4142135, sqrt_nm(2));
    assert_eq!(1.7320514, sqrt_nm(3));
}
