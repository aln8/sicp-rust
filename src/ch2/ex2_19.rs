use crate::{list, utils::cons::*};

// section 1.2.2 counting change
fn cc(amount: f64, coin: &List<f64>) -> i32 {
    // recursive end
    if amount == 0.0 {
        return 1;
    }

    if amount < 0.0 {
        return 0;
    }

    if let List::Nil = coin {
        return 0;
    }

    // recursion, tree
    // (0 current coin recursion) + (1 current coin recursion)
    cc(amount, coin.cdr_ref()) + cc(amount - coin.car_ref().unwrap(), coin)
}

#[test]
fn test_count_amount() {
    let us_coins = list!(50.0, 25.0, 10.0, 5.0, 1.0);
    assert_eq!(292, cc(100.0, &us_coins));

    let uk_coins = list!(100.0, 50.0, 20.0, 10.0, 5.0, 2.0, 1.0, 0.5);
    assert_eq!(104561, cc(100.0, &uk_coins));
}
