use num::Signed;

use crate::utils::*;
use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

// ex_1_46
fn iter_improve<'a, T>(
    is_good: Box<dyn Fn(T, T) -> bool>,
    improve: Box<dyn Fn(T) -> T>,
) -> Box<dyn Fn(T) -> T + 'a>
where
    T: Signed + Copy + 'a,
{
    Box::new(move |mut x| {
        // random num
        let mut next = improve(x);
        while !is_good(x, next) {
            x = next;
            next = improve(x);
        }
        next
    })
}

fn fixed_point<T>(f: Box<dyn Fn(T) -> T>, guess: T) -> T
where
    T: Signed + Sub<Output = T> + Copy + PartialOrd + From<f32>,
{
    iter_improve(Box::new(|a, b| abs(a - b) < T::from(0.000001)), f)(guess)
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
fn avg_damping<'a, T>(f: Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T + 'a>
where
    T: Add<Output = T> + Div<Output = T> + From<i8> + Copy + 'a,
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

fn newton_transform<'a, T>(g: Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T + 'a>
where
    T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<f32> + Copy + 'a,
{
    Box::new(move |x: T| {
        let dg = deriv(&g);
        x - g(x) / dg(x)
    })
}

fn newton_method<T>(g: Box<dyn Fn(T) -> T>, guess: T) -> T
where
    T: Signed + PartialOrd + From<f32> + Copy + 'static,
{
    fixed_point(newton_transform(g), guess)
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

// ex1_40
fn cubic(a: f32, b: f32, c: f32) -> f32 {
    fixed_point_of_transform(
        Box::new(move |x| cube(x) + a * square(x) + b * x + c),
        newton_transform,
        1.0,
    )
}

#[test]
fn test_cubic() {
    assert_eq!(-1.6506292, cubic(2.0, 3.0, 4.0));
    assert_eq!(-4.9609547, cubic(5.0, 1.0, 4.0));
}
