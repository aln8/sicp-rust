use std::mem::replace;

use crate::{list, utils::cons::*, utils::list::*};

// since rust won't support any kind of dotted-tail notation
// use rust macro for that achieve same grammar
macro_rules! same_parity {
    ( $base:expr, $( $a:expr ),*) => {
        {
            let base_odd = $base % 2 == 0;
            // using dummy to hold head
            // using last to hold internal tail for add
            let mut dummy = list!(0);
            let mut last = &mut dummy;

            let mut list = list!($( $a),*);
            while list.cdr_ref().is_some() {
                let cons_odd = list.car_ref::<i32>().unwrap() % 2 == 0;
                let mut next = list.set_cdr(None);
                // if both odd or both not odd
                if !(base_odd ^ cons_odd) {
                    // get next and break current list link
                    // set last next to list
                    last.set_cdr(Some(list));
                    // set last to list
                    last = last.cdr_mut().unwrap();
                }
                list = next.unwrap();
            }
            dummy.cdr().unwrap()
        }
    };
}

#[test]
fn test_same_parity() {
    let expect = list!(3, 5, 7);
    let result = same_parity!(1, 2, 3, 4, 5, 6, 7, 8);
    assert!(result.iter().eq(expect.iter()));
}
