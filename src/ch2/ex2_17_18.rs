use std::{
    fmt::{Debug, Display},
    mem::replace,
};

use crate::{list, utils::cons::*};

fn last_pair<T: Default + Debug>(mut list: List<T>) -> List<T> {
    while let List::Cons(cons) = list.cdr_ref() {
        list = list.cdr();
    }
    list
}

#[test]
fn test_last_pair() {
    let l = list!(1, 2, 3, 4);
    assert_eq!(4, last_pair(l).car().unwrap());
}

fn reverse<T: Copy + Default + Debug>(mut list: List<T>) -> List<T> {
    let mut last: List<T> = List::Nil;
    while let List::Cons(cons) = &list {
        // change link, list cdr to last, hold current list cdr
        let next = replace(list.cdr_mut(), last);
        // set head to last
        last = list;
        // set list current cdr
        list = next;
    }
    last
}

fn reverse_rec<T: Copy + Default + Debug>(mut list: List<T>) -> List<T> {
    // if end, then new head
    if let List::Nil = list.cdr_ref() {
        return list;
    }

    // break link, list to Nil, hold next
    let mut next = replace(list.cdr_mut(), List::Nil);
    let mut next_cdr: *mut List<T> = &mut next;
    let mut head = reverse_rec(next);
    unsafe {
        // set next.next to list
        replace((*next_cdr).cdr_mut(), list);
    }
    head
}

#[test]
fn test_reverse() {
    let l = list!(1, 2, 3, 4);

    let test_list = [4, 3, 2, 1];
    let mut test_idx = 0;
    for val in &reverse(l) {
        assert_eq!(&test_list[test_idx], val);
        test_idx += 1;
    }
}

#[test]
fn test_reverse_rec() {
    let mut l = list!(1, 2, 3, 4);

    let test_list = [4, 3, 2, 1];
    let mut test_idx = 0;
    for val in &reverse_rec(l) {
        assert_eq!(&test_list[test_idx], val);
        test_idx += 1;
    }
}
