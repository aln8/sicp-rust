use crate::utils::*;

pub fn ex1_03(a: i64, b: i64, c: i64) -> i64 {
    let (l1, l2) = larger_two_of_there(a, b, c);
    sum_of_square(l1, l2)
}

// procedure: sum of square
fn sum_of_square(a: i64, b: i64) -> i64 {
    square(a) + square(b)
}

// procedure: larger two of there
fn larger_two_of_there(a: i64, b: i64, c: i64) -> (i64, i64) {
    if a > b {
        if b > c {
            return (a, b);
        }
        return (a, c);
    }

    if a > c {
        return (a, b);
    }
    return (b, c);
}

#[test]
fn test_ex1_3() {
    assert_eq!(41, ex1_03(3, 4, 5));
}
