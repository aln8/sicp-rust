use crate::{list, utils::cons::*};

// section 1.2.2 counting change
fn cc(amount: f64, coin: &List) -> i32 {
    fn cc_rec(amount: f64, coin: Option<&List>) -> i32 {
        // recursive end
        if amount == 0.0 {
            return 1;
        }

        if amount < 0.0 {
            return 0;
        }

        if coin.is_none() {
            return 0;
        }

        cc_rec(amount, coin.unwrap().cdr_ref())
            + cc_rec(amount - coin.unwrap().car_ref::<f64>().unwrap(), coin)
    }
    cc_rec(amount, Some(coin))
}

#[test]
fn test_count_amount() {
    let us_coins = list!(50.0, 25.0, 10.0, 5.0, 1.0);
    assert_eq!(292, cc(100.0, &us_coins));

    let uk_coins = list!(100.0, 50.0, 20.0, 10.0, 5.0, 2.0, 1.0, 0.5);
    assert_eq!(104561, cc(100.0, &uk_coins));
}
