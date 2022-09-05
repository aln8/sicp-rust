use std::{ops::Mul, usize};

use num::{pow, Integer, One};

// 2^a*3^b
// means count divider 2, 3
fn cons(x: usize, y: usize) -> usize {
    pow(2, x) * pow(3, y)
}

fn car(con: usize) -> usize {
    count_divider(con, 2)
}

fn cdr(con: usize) -> usize {
    count_divider(con, 3)
}

fn count_divider(mut n: usize, divisor: usize) -> usize {
    let mut count = 0;
    while n % divisor == 0 {
        count += 1;
        n = n / divisor;
    }
    count
}

#[test]
fn test_car() {
    let a = cons(3, 5);
    assert_eq!(3, car(a));
    assert_eq!(5, cdr(a));
}
