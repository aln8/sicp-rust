use std::ops::{Add, Div, Mul, Sub};

use num::Signed;

use crate::utils::ops::*;

#[derive(Debug, Clone, Copy)]
struct Interval {
    lower: f64,
    upper: f64,
}

impl Interval {
    // ex 2.7
    fn new(a: f64, b: f64) -> Interval {
        if a > b {
            return Interval { lower: b, upper: a };
        }
        Interval { lower: a, upper: b }
    }

    // ex 2.12
    fn with_center_percent(center: f64, percent: f64) -> Interval {
        let delta = abs(center * percent);
        Interval {
            lower: center - delta,
            upper: center + delta,
        }
    }

    fn center(&self) -> f64 {
        (self.lower + self.upper) / 2.0
    }

    fn width(&self) -> f64 {
        (self.upper - self.lower) / 2.0
    }

    // ex 2.12
    fn percent(&self) -> f64 {
        self.width() / self.center()
    }
}

impl Add for Interval {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.lower + other.lower, self.upper + other.upper)
    }
}

// ex 2.11 mul
impl Mul for Interval {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        // original implementation
        // let v = [
        //     self.lower * rhs.lower,
        //     self.lower * rhs.upper,
        //     self.upper * rhs.lower,
        //     self.upper * rhs.upper,
        // ];

        // Self::new(
        //     v.iter().copied().reduce(f64::min).unwrap(),
        //     v.into_iter().reduce(f64::max).unwrap(),
        // )

        // multiply reduction implementation
        if self.lower > 0.0 {
            // lower > 0 and lower < 0, upper > 0, same
            if rhs.lower > 0.0 {
                return Self::new(self.lower * rhs.lower, self.upper * rhs.upper);
            } else {
                return Self::new(self.lower * rhs.upper, self.upper * rhs.lower);
            }
        } else if self.upper > 0.0 {
            if rhs.lower > 0.0 {
                return Self::new(self.lower * rhs.upper, self.upper * rhs.lower);
            } else if rhs.upper > 0.0 {
                return Self::new(
                    min(self.lower * rhs.upper, self.upper * rhs.lower),
                    max(self.lower * rhs.lower, self.upper * self.upper),
                );
            } else {
                return Self::new(self.lower * rhs.upper, self.upper * rhs.lower);
            }
        } else {
            if rhs.lower > 0.0 {
                return Self::new(self.upper * rhs.lower, self.lower * rhs.upper);
            }
            // self l < 0, u < 0, rhs u < 0 && u > 0 combine
            else {
                return Self::new(self.upper * rhs.upper, self.lower * rhs.lower);
            }
        }
    }
}

// ex 2.10, spans 0 means cross 0
impl Div for Interval {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        if rhs.lower * rhs.upper <= 0.0 {
            panic!("invalid 0 bound divide");
        }
        self * Self::new(1.0 / rhs.lower, 1.0 / rhs.upper)
    }
}

// ex 2.8
impl Sub for Interval {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.lower - rhs.lower, self.upper - rhs.upper)
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.upper == other.upper && self.lower == other.lower
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

// ex 2.13
fn par1(r1: Interval, r2: Interval) -> Interval {
    (r1 * r2) / (r1 + r2)
}

fn par2(r1: Interval, r2: Interval) -> Interval {
    let one = Interval::new(1.0, 1.0);
    one / (one / r1 + one / r2)
}

// ex2.14
#[test]
fn test_par() {
    let p1 = Interval::new(1.0, 5.0);
    let p2 = Interval::new(2.0, 10.0);
    assert_ne!(par1(p1, p2), par2(p1, p2));
}

// ex2.15, ex2.16
// the reason is when bound multiple divide result (A/B)*B != A
#[test]
fn test_expression() {
    let a = Interval::with_center_percent(2.0, 0.2);
    let b = Interval::with_center_percent(2.0, 0.0);
    assert_eq!(Interval::with_center_percent(1.0, 0.0), b / b);
    assert_ne!(Interval::with_center_percent(1.0, 0.0), a / a);
}
