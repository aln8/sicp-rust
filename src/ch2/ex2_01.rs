use crate::utils::*;
use num::traits::NumOps;
use std::ops::{Add, Div, Mul, Sub};

// rational data
struct Rat {
    numer: i64,
    denom: u64, // keep denominator always positive
}

impl Rat {
    pub fn new(mut num: i64, mut den: i64) -> Rat {
        let div = gcd(num, den);
        (num, den) = (num / div, den / div);
        if den < 0 {
            (num, den) = (-num, -den);
        }

        Rat {
            numer: num,
            denom: den as u64,
        }
    }
}

impl Add for Rat {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(
            self.numer * other.denom as i64 + other.numer * self.denom as i64,
            (self.denom * other.denom) as i64,
        )
    }
}

impl Sub for Rat {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(
            self.numer * other.denom as i64 - other.numer * self.denom as i64,
            (self.denom * other.denom) as i64,
        )
    }
}

impl Mul for Rat {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.numer * other.numer, (self.denom * other.denom) as i64)
    }
}

impl Div for Rat {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self::new(
            self.numer * other.denom as i64,
            self.denom as i64 * other.numer,
        )
    }
}

impl PartialEq for Rat {
    fn eq(&self, other: &Self) -> bool {
        self.numer * other.denom as i64 == self.denom as i64 * other.numer
    }
}

#[test]
fn test_rat_new() {
    let r1 = Rat::new(10, -20);
    assert_eq!(-1, r1.numer);
    assert_eq!(2, r1.denom);
}

#[test]
fn test_rat_add() {
    let r1 = Rat::new(16, -20);
    let r2 = Rat::new(9, -12);
    let r3 = r1 + r2;
    assert_eq!(-31, r3.numer);
    assert_eq!(20, r3.denom);
}
