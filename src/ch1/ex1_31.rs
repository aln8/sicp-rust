use crate::utils::*;
use num::traits::One;
use std::fmt::Debug;
use std::ops::Mul;

fn product<T, FT, FN>(term: FT, mut now: T, next: FN, end: T) -> T
where
    T: Mul<Output = T> + PartialOrd + One + Copy + Debug,
    FT: Fn(T) -> T,
    FN: Fn(T) -> T,
{
    if now > end {
        return T::one();
    }
    term(now) * product(term, next(now), next, end)
}

fn prod_iter<T, FT, FN>(term: FT, mut now: T, next: FN, end: T) -> T
where
    T: Mul + PartialOrd + One + Copy + Debug,
    FT: Fn(T) -> T,
    FN: Fn(T) -> T,
{
    let mut result = T::one();
    while now <= end {
        result = result * term(now);
        now = next(now);
    }
    result
}

// pi/4  =  2/3 * 4/3 * 4/5 * 6/5 * 6/7 * 8/7 * .....
// so, pi/4
// term: n/(n+1) * (n+2)/(n+1)
// now: 2
// end: if even end, else end + 1
// next: n + 2
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
