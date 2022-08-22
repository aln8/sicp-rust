use crate::utils::*;

fn russian_peasant_mul(mut a: i32, mut b: i32) -> i32 {
    let mut mul = 0;
    loop {
        if b == 0 {
            return mul;
        }

        mul = mul + a * (b & 1);
        a = a << 1;
        b = b >> 1;
    }
}

#[test]
fn test_russian_peasant_mul() {
    assert_eq!(25, russian_peasant_mul(5, 5))
}

fn russian_peasant_exp(mut a: i32, mut b: i32) -> i32 {
    let mut exp = 1;
    loop {
        if b == 0 {
            return exp;
        }

        if (b & 1) > 0 {
            exp = exp * a
        }

        a = a * a;
        b = b >> 1;
    }
}

#[test]
fn test_russian_peasant_exp() {
    assert_eq!(25, russian_peasant_exp(5, 2));
    assert_eq!(625, russian_peasant_exp(5, 4));
}

// a(1) = q(a + b) + pa = (p+q)a + qb
// b(1) = qa + pb
// a(2) = (p+q)a(1) + qb(1) = (p+q)((p+q)a + qb) + q(qa + pb) = (p^2 + 2pq + 2q^2)a + (2pq + q^2)b
// b(2) = qa(1) + pb(1) = q((p+q)a + qb) + p(qa+pb) = (2pq + q^2)a + (p^2 + q^2)b
// so, twice
// q(1) = (q^2 + 2pq)
// p(1) = p^2 + q^2
// a(2) = q(1)(a+b) + p(1)b
// b(2) = q(1)a + p(1)b
fn fib(n: i32) -> i32 {
    fib_iter(1, 0, 0, 1, n)
}

fn fib_iter(mut a: i32, mut b: i32, mut p: i32, mut q: i32, n: i32) -> i32 {
    if n == 0 {
        return b;
    }

    if is_even(n) {
        (p, q) = (square(p) + square(q), square(q) + 2 * p * q);
        return fib_iter(a, b, p, q, n / 2);
    }

    // odd a(1), b(1)
    (a, b) = ((p + q) * a + q * b, q * a + p * b);
    fib_iter(a, b, p, q, n - 1)
}

#[test]
fn test_fib() {
    assert_eq!(5, fib(5));
    assert_eq!(55, fib(10));
}
