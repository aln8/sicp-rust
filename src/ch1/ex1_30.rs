use num::traits::Zero;
use std::fmt::Debug;
use std::ops::Add;

fn sum_iter<T, FT, FN>(term: FT, mut now: T, next: FN, end: T) -> T
where
    T: Add + PartialOrd + Zero + Copy + Debug,
    FT: Fn(T) -> T,
    FN: Fn(T) -> T,
{
    let mut result = T::zero();
    while now <= end {
        result = result + term(now);
        now = next(now);
    }
    result
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
fn test_frac_pi_8_iter() {
    assert_eq!(0.39244908, frac_pi_8_iter(1000.0));
}
