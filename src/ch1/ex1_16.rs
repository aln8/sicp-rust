use crate::utils::ops::*;

fn exp(b: i32, n: i32) -> i32 {
    exp_rec(1, b, n)
}

fn exp_rec(a: i32, b: i32, n: i32) -> i32 {
    if n == 0 {
        return a;
    }

    if is_even(n) {
        return exp_rec(a, square(b), n / 2);
    }

    exp_rec(a * b, b, n - 1)
}

#[test]
fn test_ex1_16() {
    assert_eq!(1, exp(5, 0));
    assert_eq!(5, exp(5, 1));
    assert_eq!(125, exp(5, 3));
}
