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

// section 1.2.2 counting change
fn count_change(amount: i32) -> i32 {
    cc(amount, 5)
}

fn cc(amount: i32, coin: i32) -> i32 {
    // recursive end
    if amount == 0 {
        return 1;
    }

    if amount < 0 || coin <= 0 {
        return 0;
    }

    // recursion, tree
    // (0 current coin recursion) + (1 current coin recursion)
    cc(amount, coin - 1) + cc(amount - coins_value(coin), coin)
}

// available coins 1, 5, 10, 15, 20
fn coins_value(i: i32) -> i32 {
    match i {
        1 => 1,
        2 => 5,
        3 => 10,
        4 => 25,
        5 => 50,
        _ => 0,
    }
}

#[test]
fn test_count_amount() {
    assert_eq!(4, count_change(10));
    assert_eq!(292, count_change(100));
}
