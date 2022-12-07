use std::{
    fmt::{Debug, Display},
    mem::{replace, ManuallyDrop},
    pin::Pin,
};

use crate::{list, utils::cons::*};

fn last_pair<T: Default + Debug>(mut list: List) -> List {
    while let Some(next) = list.cdr() {
        list = next;
    }
    list
}

#[test]
fn test_last_pair() {
    let l = list!(1, 2, 3, 4);
    assert_eq!(4, last_pair::<i32>(l).car().unwrap());
}

fn reverse<T: Copy + Default + Debug>(mut list: List) -> List {
    // keep last element, for each iteration:
    // 1. next = cur.next
    // 2. cur.next = last
    // iter next
    let mut last: Option<List> = None;
    while let Some(next) = list.cdr_mut() {
        // change link, list cdr to last, next
        let next = list.set_cdr(last);
        // set head to last
        last = Some(list);
        // set list current cdr
        list = next.unwrap();
    }
    list
}

fn reverse_rec(mut list: &mut List) -> List {
    // if end, then new head
    if list.cdr_ref().is_none() {
        return list.take();
    }

    // break link, list to Nil, hold next
    let mut next = list.cdr().unwrap();
    let mut head = reverse_rec(&mut next);
    next.set_cdr(Some(list.take()));
    head
}

#[test]
fn test_reverse() {
    let l = list!(1, 2, 3, 4);

    let test_list = [4, 3, 2, 1];
    let mut test_idx = 0;
    for val in &reverse::<i32>(l) {
        assert_eq!(*val, test_list[test_idx]);
        test_idx += 1;
    }
}

#[test]
fn test_reverse_rec() {
    let mut l = list!(1, 2, 3, 4, 5, 6, 7);

    let test_list = [7, 6, 5, 4, 3, 2, 1];
    let mut test_idx = 0;
    for val in &reverse_rec(&mut l) {
        assert_eq!(*val, test_list[test_idx]);
        test_idx += 1;
    }
}
