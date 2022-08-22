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
