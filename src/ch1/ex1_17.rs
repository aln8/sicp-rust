use crate::utils::*;

/// a + x * n = a + n of x sum
/// if even(n) =   a + (n/2) of ((x) + (x))
///     a_next = a, x_next = x + x, n_next = n / 2
/// else       =  (a + x) + (n-1) of (a+x)
///     a_next = a + x, x_next = x, n_next = n - 1
fn mul(x: i32, n: i32) -> i32 {
    mul_rec(0, x, n)
}

fn mul_rec(a: i32, x: i32, n: i32) -> i32 {
    if n == 0 || x == 0 {
        return 0;
    }

    if n == 1 {
        return a + x;
    }

    if is_even(n) {
        return mul_rec(a, x + x, n / 2);
    }

    mul_rec(a + x, x, n - 1)
}

#[test]
fn test_ex1_17() {
    assert_eq!(0, mul(5, 0));
    assert_eq!(5, mul(5, 1));
    assert_eq!(20, mul(5, 4));
}
