use crate::{
    list,
    utils::{cons::*, list::List, ops::*},
};

fn square_list(l: List) -> List {
    l.into_iter_downcast::<i32>().map(|x| square(x)).collect()
}

#[test]
fn test_square_list() {
    let mut test_result = vec![1, 4, 9];
    let mut idx = 0;
    assert!(&square_list(list!(1, 2, 3)).iter().eq(test_result.iter()));
}

fn for_each(l: List, apply: impl FnMut(i32)) {
    l.into_iter_downcast::<i32>().for_each(apply)
}

#[test]
fn test_for_each() {
    let mut v = vec![];
    let out = |a: i32| v.push(a);

    let result = vec![1, 2, 3, 4];
    for_each(list!(1, 2, 3, 4), Box::new(out));
    assert_eq!(v, result);
}
