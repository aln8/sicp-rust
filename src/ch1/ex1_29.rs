use crate::utils::*;
use num::traits::Zero;
use std::fmt::Debug;
use std::ops::Add;

fn sum<T, FT, FN>(term: FT, now: T, next: FN, end: T) -> T
where
    T: Add + PartialOrd + Zero + Copy + Debug,
    FT: Fn(T) -> T,
    FN: Fn(T) -> T,
{
    if now > end {
        return T::zero();
    }
    term(now) + sum(term, next(now), next, end)
}

fn integral_simpson_f32<FT>(f: FT, a: f32, b: f32, n: i32) -> f32
where
    FT: Fn(f32) -> f32,
{
    let h = (b - a) / n as f32;
    // sum from 0 to n
    (h / 3.0)
        * sum(
            |x: f32| -> f32 {
                let fx = f(a + (x as f32 * h));
                if is_even(x as i32) {
                    return 2.0 * fx;
                }
                4.0 * fx
            },
            0.0,
            |x: f32| -> f32 { x + 1.0 },
            n as f32,
        )
}

fn integral_f32<FT>(f: FT, a: f32, b: f32, dx: f32) -> f32
where
    FT: Fn(f32) -> f32,
{
    sum(f, a + dx / 2.0, |x: f32| -> f32 { x + dx }, b) * dx
}

fn frac_pi_4(end: f32) -> f32 {
    sum(
        |a: f32| -> f32 { 1.0 / (a * (a + 2.0)) },
        1.0,
        |a: f32| -> f32 { a + 4.0 },
        end,
    )
}

#[test]
fn test_frac_pi_4() {
    assert_eq!(0.39244908, frac_pi_4(1000.0));
}

#[test]
fn test_integral_simpson_cube() {
    assert_eq!(
        0.2533333,
        integral_simpson_f32(|x: f32| { x * x * x }, 0.0, 1.0, 100)
    );
    assert_eq!(
        0.25033355,
        integral_simpson_f32(|x: f32| { x * x * x }, 0.0, 1.0, 1000)
    );
}

#[test]
fn test_integral_cube() {
    assert_eq!(
        0.24998708,
        integral_f32(|x: f32| { x * x * x }, 0.0, 1.0, 0.01)
    );
    assert_eq!(
        0.24999388,
        integral_f32(|x: f32| { x * x * x }, 0.0, 1.0, 0.001)
    );
}
