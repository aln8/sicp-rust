// continue fraction
use std::ops::{Add, Div, Mul, Sub};

use num::Zero;

fn cont_frac<NT, DT, T>(f_n: NT, f_d: DT, k: i32) -> T
where
    T: Add<Output = T> + Div<Output = T> + Zero,
    NT: Fn(i32) -> T,
    DT: Fn(i32) -> T,
{
    if k < 1 {
        return T::zero();
    }

    let mut i = k - 1;
    let mut cur = f_n(k) / f_d(k);
    while i > 0 {
        cur = f_n(i) / (f_d(i) + cur);
        i -= 1;
    }
    cur
}

fn cont_frac_rec<NT, DT, T>(f_n: NT, f_d: DT, k: i32) -> T
where
    T: Add<Output = T> + Div<Output = T> + Zero,
    NT: Fn(i32) -> T,
    DT: Fn(i32) -> T,
{
    cont_frac_rec_impl(f_n, f_d, 1, k)
}

fn cont_frac_rec_impl<NT, DT, T>(f_n: NT, f_d: DT, i: i32, k: i32) -> T
where
    T: Add<Output = T> + Div<Output = T> + Zero,
    NT: Fn(i32) -> T,
    DT: Fn(i32) -> T,
{
    if i == k {
        return f_n(k) / f_d(k);
    }

    f_n(i) / (f_d(i) + cont_frac_rec_impl(f_n, f_d, i + 1, k))
}

// golden ration: f_n: || return 1.0, f_d: || return 1.0
fn golden_ratio() -> f32 {
    cont_frac(|n: i32| 1.0, |n: i32| 1.0, 1000)
}

fn golden_ratio_rec() -> f32 {
    cont_frac_rec(|n: i32| 1.0, |n: i32| 1.0, 1000)
}

#[test]
fn test_golden_ratio() {
    assert_eq!(0.618034, golden_ratio());
}

#[test]
fn test_golden_ratio_rec() {
    assert_eq!(0.618034, golden_ratio_rec());
}

// e sub 2
// ni = 1.0
// di = 1, 2, 1, 1, 4, 1, 1, 6, 1, 1, 8
fn e_sub_2() -> f32 {
    cont_frac(
        |n| 1.0,
        |n| {
            if n % 3 == 2 {
                return (2 * (n / 3 + 1)) as f32;
            }
            1.0
        },
        1000,
    )
}

#[test]
fn test_e_sub_2() {
    assert_eq!(0.7182818, e_sub_2());
}

// tangent
// ni = x, -x^2, -x^2, ...
// di = 1, 3, 5, 7, 9
fn tan_cf(x: f32, k: i32) -> f32 {
    cont_frac(
        |n| {
            if n == 1 {
                return 1.0 * x;
            }
            -1.0 * x * x
        },
        |n| 1.0 + (n - 1) as f32 * 2.0,
        k,
    )
}

#[test]
fn test_tan_cf() {
    assert_eq!(-0.14254652, tan_cf(3.0, 1000));
}
