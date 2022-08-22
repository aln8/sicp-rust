// recursive
fn rec(n: i32) -> i32 {
    if n < 3 {
        return n;
    }

    return rec(n - 1) + 2 * rec(n - 2) + 3 * rec(n - 3);
}

// iter
fn iter(n: i32) -> i32 {
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
