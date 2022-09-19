use crate::{
    list,
    utils::{cons::*, ops::*},
};

fn square_list(l: List<i32>) -> List<i32> {
    l.map(|x| square(x)).collect()
}

#[test]
fn test_square_list() {
    let mut test_result = list!(1, 4, 9);
    for val in &square_list(list!(1, 2, 3)) {
        assert_eq!(test_result.car_ref().unwrap(), val);
        test_result = test_result.cdr();
    }
}

fn for_each(l: List<i32>, apply: impl FnMut(i32)) {
    l.for_each(apply)
}

#[test]
fn test_for_each() {
    let mut v = vec![];
    let out = |a: i32| v.push(a);

    let result = vec![1, 2, 3, 4];
    for_each(list!(1, 2, 3, 4), Box::new(out));
    assert_eq!(v, result);
}
