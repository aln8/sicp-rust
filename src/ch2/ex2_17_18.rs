use std::{
    fmt::{Debug, Display},
    mem::{replace, ManuallyDrop},
    pin::Pin,
};

use crate::{
    list,
    utils::{cons::*, list::List},
};

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

fn reverse(list: List) -> List {
    // keep last element, for each iteration:
    // 1. next = cur.next
    // 2. cur.next = last
    // iter next
    let mut last: Option<List> = None;
    let mut cur = Some(list);
    while let Some(mut cur_list) = cur {
        // change link, list cdr to last, next
        let next = cur_list.set_cdr(last);
        // set head to last
        last = Some(cur_list);
        // set list current cdr
        cur = next;
    }
    last.unwrap()
}

fn reverse_rec(mut list: List) -> List {
    if list.cdr_ref().is_none() {
        return list;
    }

    // break link, list to Nil, hold next
    let mut head = reverse_rec(list.set_cdr(None).unwrap());
    head.tail().set_cdr(Some(list));
    head
}

#[test]
fn test_reverse() {
    let l = list!(1, 2, 3, 4);
    let expect = [4, 3, 2, 1];
    assert!(reverse(l).iter().eq(expect.iter()));
}

#[test]
fn test_reverse_rec() {
    let mut l = list!(1, 2, 3, 4, 5, 6, 7);
    let expect = [7, 6, 5, 4, 3, 2, 1];
    assert!(reverse_rec(l).iter().eq(expect.iter()));
}
