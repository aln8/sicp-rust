use crate::utils::*;
use num::traits::{One, Zero};
use std::fmt::Debug;
use std::ops::{Add, Mul};

fn accumulate<T, FT, FN>(
    mut now: T,
    end: T,
    term: FT,
    next: FN,
    combiner: fn(T, T) -> T,
    null_value: T,
) -> T
where
    T: PartialOrd + Copy + Debug,
    FT: Fn(T) -> T,
    FN: Fn(T) -> T,
{
    if now > end {
        return null_value;
    }

    combiner(
        term(now),
        accumulate(next(now), end, term, next, combiner, null_value),
    )
}

fn accumulate_iter<T, FT, FN>(
    mut now: T,
    end: T,
    term: FT,
    next: FN,
    combiner: fn(T, T) -> T,
    null_value: T,
) -> T
where
    T: PartialOrd + Copy + Debug,
    FT: Fn(T) -> T,
    FN: Fn(T) -> T,
{
    let mut result = null_value;
    while now <= end {
        result = combiner(term(now), result);
        now = next(now);
    }
    result
}

fn sum<T, FT, FN>(term: FT, mut now: T, next: FN, end: T) -> T
where
    T: Add<Output = T> + PartialOrd + Zero + Copy + Debug,
    FT: Fn(T) -> T,
    FN: Fn(T) -> T,
{
    accumulate(now, end, term, next, |a: T, b: T| -> T { a + b }, T::zero())
}

fn sum_iter<T, FT, FN>(term: FT, mut now: T, next: FN, end: T) -> T
where
    T: Add<Output = T> + PartialOrd + Zero + Copy + Debug,
    FT: Fn(T) -> T,
    FN: Fn(T) -> T,
{
    accumulate_iter(now, end, term, next, |a, b| a + b, T::zero())
}

fn product<T, FT, FN>(term: FT, mut now: T, next: FN, end: T) -> T
where
    T: Mul<Output = T> + PartialOrd + One + Copy + Debug,
    FT: Fn(T) -> T,
    FN: Fn(T) -> T,
{
    accumulate(now, end, term, next, |a, b| a * b, T::one())
}

fn prod_iter<T, FT, FN>(term: FT, mut now: T, next: FN, end: T) -> T
where
    T: Mul<Output = T> + PartialOrd + One + Copy + Debug,
    FT: Fn(T) -> T,
    FN: Fn(T) -> T,
{
    accumulate_iter(now, end, term, next, |a, b| a * b, T::one())
}

fn frac_pi_8(end: f32) -> f32 {
    sum(
        |a: f32| -> f32 { 1.0 / (a * (a + 2.0)) },
        1.0,
        |a: f32| -> f32 { a + 4.0 },
        end,
    )
}

fn frac_pi_8_iter(end: f32) -> f32 {
    sum_iter(
        |a: f32| -> f32 { 1.0 / (a * (a + 2.0)) },
        1.0,
        |a: f32| -> f32 { a + 4.0 },
        end,
    )
}

#[test]
fn test_frac_pi_8() {
    assert_eq!(0.39244908, frac_pi_8(1000.0));
}

#[test]
fn test_frac_pi_8_iter() {
    assert_eq!(0.39244908, frac_pi_8_iter(1000.0));
}

fn frac_pi_4(mut end: i32) -> f32 {
    if !is_even(end) {
        end = end + 1;
    }

    product(
        |now: f32| -> f32 { now * (now + 2.0) / square(now + 1.0) },
        2.0,
        |now: f32| -> f32 { now + 2.0 },
        end as f32,
    )
}

fn frac_pi_4_iter(mut end: i32) -> f32 {
    if !is_even(end) {
        end = end + 1;
    }

    prod_iter(
        |now: f32| -> f32 { now * (now + 2.0) / square(now + 1.0) },
        2.0,
        |now: f32| -> f32 { now + 2.0 },
        end as f32,
    )
}

#[test]
fn test_frac_pi_4() {
    assert_eq!(0.78549474, frac_pi_4(10000));
}

#[test]
fn test_frac_pi_4_iter() {
    assert_eq!(0.7854873, frac_pi_4_iter(10000));
}
