use num::Signed;

use crate::utils::ops::*;
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};
use std::process::Output;

fn double<'a, T>(procedure: Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T + 'a>
where
    T: 'a,
{
    Box::new(move |x| procedure(procedure(x)))
}

// ex1_41
fn inc(a: i32) -> i32 {
    a + 1
}

#[test]
fn test_inc() {
    assert_eq!(3, double(Box::new(inc))(1));
    assert_eq!(17, double(double(Box::new(double)))(Box::new(inc))(1));
}

// ex1_42
// f(x), g(x) -> f(g(x))
fn compose<'a, T>(f: Box<dyn Fn(T) -> T>, g: Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T + 'a>
where
    T: 'a,
{
    Box::new(move |x| f(g(x)))
}

#[test]
fn test_compose() {
    assert_eq!(4, compose(Box::new(square), Box::new(inc))(1));
}

// ex1_43
fn repeat<'a, T>(f: Box<dyn Fn(T) -> T>, n: u32) -> Box<dyn Fn(T) -> T + 'a>
where
    T: 'a,
{
    Box::new(move |mut x| {
        let mut max = n;
        while max > 0 {
            x = f(x);
            max = max - 1;
        }
        x
    })
}

#[test]
fn test_repeat() {
    assert_eq!(625, repeat(Box::new(square), 2)(5));
}

// ex1_44
fn smooth<'a, T>(f: Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T + 'a>
where
    T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<f32> + Copy + 'a,
{
    Box::new(move |x| {
        let dx: T = T::from(0.00001);
        (f(x - dx) + f(x) + f(x + dx)) / T::from(3.0)
    })
}

fn repeat_smooth<T>(f: Box<dyn Fn(T) -> T>, n: u32) -> Box<dyn Fn(T) -> T>
where
    T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<f32> + Copy + 'static,
{
    repeat(Box::new(smooth), n)(f)
}

// return a closure and used by next item
fn avg_damping<'a, T>(f: Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T + 'a>
where
    T: Add<Output = T> + Div<Output = T> + From<i8> + Copy + 'a,
{
    Box::new(move |x: T| -> T { average(x, f(x)) })
}

fn fixed_point<T, FT>(f: FT, mut now: T) -> T
where
    T: Signed + Copy + Sub<Output = T> + PartialOrd + From<f32>,
    FT: Fn(T) -> T,
{
    let tolerance: f32 = 0.00001;
    let close = |a: T, b: T| -> bool { abs(a - b) < T::from(tolerance) };
    let mut next = f(now);
    while !close(now, next) {
        now = next;
        next = f(now);
    }
    now
}

// ex1_45
fn nth_root(a: i32, n: u32) -> f32 {
    fixed_point(
        repeat(Box::new(avg_damping), n - 1)(Box::new(move |x| a as f32 / power(x, n - 1))),
        1.0,
    )
}

#[test]
fn test_nth_root() {
    assert_eq!(5.000016, nth_root(625, 4));
    assert_eq!(7.0002623, nth_root(40353607, 9));
}
