use std::mem::replace;

use crate::{list, utils::cons::*};

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
            while let List::Cons(cons) = &list {
                let cons_odd = cons.car_ref() % 2 == 0;
                let mut next = replace(list.cdr_mut(), List::Nil);
                // if both odd or both not odd
                if !(base_odd ^ cons_odd) {
                    // get next and break current list link
                    // set last next to list
                    last.set_cdr(list);
                    // set last to list
                    last = last.cdr_mut();
                }
                list = next;
            }
            dummy.cdr()
        }
    };
}

#[test]
fn test_same_parity() {
    let mut odd_result = list!(3, 5, 7);
    for val in &same_parity!(1, 2, 3, 4, 5, 6, 7, 8) {
        assert_eq!(odd_result.car_ref().unwrap(), val);
        odd_result = odd_result.cdr();
    }
}
