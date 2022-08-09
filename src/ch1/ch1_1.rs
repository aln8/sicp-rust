mod ex1_03 {
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
}

mod ex1_04 {
    pub fn a_plus_abs_b(a: i64, b: i64) -> i64 {
        abs_operator(b)(a, b)
    }

    // (define (a-plus-abs-b a b)
    //   ((if (> b 0) + -) a b))
    // why we implement return a function is because the procedure
    // shows it return a operator rather than operands
    fn abs_operator(x: i64) -> fn(i64, i64) -> i64 {
        if x > 0 {
            return |a: i64, b: i64| -> i64 {
                return a + b;
            };
        }

        return |a: i64, b: i64| -> i64 {
            return a - b;
        };
    }

    #[test]
    fn test_ex1_04() {
        assert_eq!(9, a_plus_abs_b(4, 5));
        assert_eq!(9, a_plus_abs_b(4, -5));
    }
}

mod ex1_08 {
    use crate::utils::*;

    // if (valid)
    //     guess
    //     then next approximate
    //
    //                    curt
    //                     |
    //                  curt_iter
    //                 /        \
    //             valid     approximate
    //             /   \
    //         cubed   abs
    //
    pub fn curt(x: i64) -> f64 {
        curt_iter(x as f64, 1.0)
    }

    fn curt_iter(x: f64, now: f64) -> f64 {
        if valid(x, now) {
            return now;
        }

        let next = approximate(x, now);
        curt_iter(x, next)
    }

    fn approximate(x: f64, now: f64) -> f64 {
        ((x / (now * now)) + 2.0 * now) / 3.0
    }

    fn valid(x: f64, now: f64) -> bool {
        abs(cube(now) - x) < 0.001
    }

    #[test]
    fn test_ex1_08() {
        fn good(expect: f64, real: f64) -> bool {
            let mind = real - expect;
            if mind > 0.0 {
                return mind < 0.001;
            }
            return mind > -0.001;
        }

        assert_eq!(true, good(2.0, curt(8)));
        assert_eq!(true, good(-2.0, curt(-8)));
    }
}

mod ex1_11 {
    // recursive
    fn rec(n: i32) -> i32 {
        if n < 3 {
            return n;
        }

        return rec(n - 1) + 2 * rec(n - 2) + 3 * rec(n - 3);
    }

    // iter
    fn iter(mut n: i32) -> i32 {
        if n < 3 {
            return n;
        }

        let (mut a, mut b, mut c, mut next) = (2, 1, 0, 0);
        for _ in 3..n + 1 {
            next = a + 2 * b + 3 * c;
            (a, b, c) = (next, a, b)
        }
        return next;
    }

    #[test]
    fn test_ex1_11() {
        assert_eq!(2, rec(2));
        assert_eq!(59, rec(6));
        assert_eq!(2, iter(2));
        assert_eq!(59, iter(6));
    }
}

mod ex1_12 {
    fn yanghui_rec(row: i32, col: i32) -> i32 {
        if col == 1 || row == col {
            return 1;
        }
        return yanghui_rec(row - 1, col - 1) + yanghui_rec(row - 1, col);
    }

    #[test]
    fn test_ex1_12() {
        assert_eq!(1, yanghui_rec(1, 1));
        assert_eq!(6, yanghui_rec(5, 3));
    }
}

mod ex1_15 {
    fn sin(mut f: f32) -> f32 {
        if f < 0.1 {
            return f;
        }
        p(sin(f / 3.0))
    }

    fn cube(f: f32) -> f32 {
        return f * f * f;
    }

    fn p(f: f32) -> f32 {
        return 3.0 * f - 4.0 * cube(f);
    }

    #[test]
    fn test_ex1_15() {
        assert_eq!(0.09, sin(0.09));
        assert_eq!(0.14044023, sin(3.0));
    }
}

mod ex1_16 {
    use crate::utils::*;

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
}

mod ex1_17 {
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
}

mod ex1_18 {
    // same to ex1_17
}

mod ex1_19 {
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

    fn fib_iter(mut a: i32, mut b: i32, mut p: i32, mut q: i32, mut n: i32) -> i32 {
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
}
