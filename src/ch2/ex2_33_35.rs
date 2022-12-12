use std::{
    any::Any,
    collections::HashMap,
    fmt::Debug,
    hash, mem,
    ops::{Mul, Rem},
};

use crate::{
    list,
    utils::{cons::ConsAny, list::List},
};

fn square<T>(x: Box<dyn ConsAny>) -> T
where
    T: Mul<Output = T> + Copy + ConsAny,
{
    let x = (*x).as_ref_any().downcast_ref::<T>().unwrap();
    x.clone() * x.clone()
}

#[test]
fn test_map_square() {
    let l = list!(1, 2, 3, 4, 5);
    let expect = list!(1, 4, 9, 16, 25);

    let result: List = l.into_iter().map(square::<i32>).collect();
    assert!(result.iter().eq(expect.iter()))
}

fn is_odd<T>(x: &Box<dyn ConsAny>) -> bool
where
    T: Rem<Output = T> + From<i32> + PartialEq + Copy + ConsAny,
{
    let x = (**x).as_ref_any().downcast_ref::<T>().unwrap();
    x.clone() % T::from(2) != T::from(0)
}

#[test]
fn test_filter_odd() {
    let l = list!(1, 2, 3, 4, 5);
    let expect = vec![1, 3, 5];
    let mut idx = 0;
    let result: List = l.into_iter().filter(is_odd::<i32>).collect();
    assert!(result.iter().eq(expect.iter()))
}

fn plus(acc: i32, car: Box<dyn ConsAny>) -> i32 {
    if let Ok(cur) = (car as Box<dyn Any>).downcast::<i32>() {
        return acc + *cur;
    }
    acc
}

fn mul(acc: i32, car: Box<dyn ConsAny>) -> i32 {
    if let Ok(cur) = (car as Box<dyn Any>).downcast::<i32>() {
        return acc * *cur;
    }
    acc
}

fn cons(acc: Option<List>, car: Box<dyn ConsAny>) -> Option<List> {
    if let Ok(cur) = (car as Box<dyn Any>).downcast::<i32>() {
        match acc {
            None => return Some(List::new(*cur)),
            Some(mut acc) => {
                acc.tail().set_cdr(Some(List::new(*cur)));
                return Some(acc);
            }
        };
    }
    acc
}

#[test]
fn test_accumulate() {
    let l = list!(1, 2, 3, 4, 5);
    let result = l.into_iter().fold(0, plus);
    assert_eq!(result, 15);

    let l = list!(1, 2, 3, 4, 5);
    let result = l.into_iter().fold(1, mul);
    assert_eq!(result, 120);

    let l = list!(1, 2, 3, 4, 5);
    let expect = vec![1, 2, 3, 4, 5];
    assert!(l
        .into_iter()
        .fold(None, cons)
        .unwrap()
        .iter()
        .eq(expect.iter()));
}

fn enumerate_interval(low: i32, high: i32) -> Option<List> {
    if low > high {
        return None;
    }
    let mut now = List::new(low);
    now.set_cdr(enumerate_interval(low + 1, high));
    Some(now)
}

#[test]
fn test_enumerate_interval() {
    let expect = vec![2, 3, 4, 5, 6, 7];
    assert!(enumerate_interval(2, 7).unwrap().iter().eq(expect.iter()))
}

fn enumerate_tree(mut l: Option<List>) -> Option<List> {
    l.map(|mut list| {
        let mut dummy = List::default();
        if list.car_ref::<i32>().is_some() {
            let cdr = enumerate_tree(list.cdr());
            dummy.tail().set_cdr(Some(list));
            dummy.tail().set_cdr(cdr);
        } else if list.car_ref::<List>().is_some() {
            dummy.tail().set_cdr(enumerate_tree(list.car()));
            dummy.tail().set_cdr(enumerate_tree(list.cdr()));
        }
        dummy.cdr().unwrap()
    })
}

#[test]
fn test_enumerate_tree() {
    let tree = list!(1, list!(2, list!(3, 4)), 5);
    let expect = vec![1, 2, 3, 4, 5];
    assert!(enumerate_tree(Some(tree)).unwrap().iter().eq(expect.iter()));
}

#[test]
fn test_sum_odd_squares() {
    let tree = list!(1, list!(2, list!(3, 4)), 5);
    assert_eq!(
        enumerate_tree(Some(tree))
            .unwrap()
            .into_iter()
            .filter(is_odd::<i32>)
            .collect::<List>() // filter
            .into_iter()
            .map(square::<i32>)
            .collect::<List>() // map square
            .into_iter()
            .fold(0, plus), // accumulate plus
        35
    )
}

fn is_even<T>(x: &Box<dyn ConsAny>) -> bool
where
    T: Rem<Output = T> + From<i32> + PartialEq + Copy + ConsAny,
{
    !is_odd::<T>(x)
}

fn map_fib() -> impl FnMut(Box<dyn ConsAny>) -> i32 {
    let mut hashmap: HashMap<i32, i32> = HashMap::new();
    hashmap.insert(0, 0);
    hashmap.insert(1, 1);
    return move |x: Box<dyn ConsAny>| {
        let k = *(x as Box<dyn Any>).downcast::<i32>().unwrap();
        if let Some(v) = hashmap.get(&k) {
            return *v;
        }

        let v = match k {
            0 | 1 => 1,
            _ => {
                let prev2 = hashmap.get(&(k - 2)).unwrap();
                let prev1 = hashmap.get(&(k - 1)).unwrap();
                prev2 + prev1
            }
        };

        hashmap.insert(k, v);
        v
    };
}

fn even_fibs(n: i32) -> List {
    enumerate_interval(0, n)
        .unwrap()
        .into_iter()
        .map(map_fib())
        .collect::<List>()
        .into_iter()
        .filter(is_even::<i32>)
        .collect::<List>()
        .into_iter()
        .fold(None, cons)
        .unwrap()
}

#[test]
fn test_even_fibs() {
    let expect = vec![0, 2, 8, 34];
    let odd_fib = even_fibs(10);
    assert!(odd_fib.iter().eq(expect.iter()));
}

fn lib_fib_squares(n: i32) -> List {
    enumerate_interval(0, n)
        .unwrap()
        .into_iter()
        .map(map_fib())
        .collect::<List>()
        .into_iter()
        .map(square::<i32>)
        .collect::<List>()
        .into_iter()
        .fold(None, cons)
        .unwrap()
}

#[test]
fn test_lib_fib_squares() {
    let fib_sq = lib_fib_squares(10);
    let expect = vec![0, 1, 1, 4, 9, 25, 64, 169, 441, 1156, 3025];
    assert!(fib_sq.iter().eq(expect.iter()));
}

fn acc_map<T: ConsAny>(mut p: impl FnMut(Box<dyn ConsAny>) -> T, l: List) -> List {
    l.into_iter()
        .fold(None, |acc, cur| match acc {
            None => Some(List::new(p(cur))),
            Some(mut acc) => {
                acc.tail().set_cdr(Some(List::new(p(cur))));
                Some(acc)
            }
        })
        .unwrap()
}

fn append(l1: List, l2: List) -> List {
    l2.into_iter().fold(Some(l1), cons).unwrap()
}

fn length(l: List) -> i32 {
    l.into_iter().fold(0, |mut acc, cur| {
        acc += 1;
        acc
    })
}

#[test]
fn test_2_33() {
    let l1 = list!(1, 2, 3, 4, 5);
    let expect = vec![1, 4, 9, 16, 25];
    assert!(acc_map(square::<i32>, l1).iter().eq(expect.iter()));

    let l1 = list!(1, 2);
    let l2 = list!(3, 4, 5);
    let expect = list!(1, 2, 3, 4, 5);
    assert!(append(l1, l2).iter().eq(expect.iter()));

    let l1 = list!(1, 2, 3, 4, 5);
    assert_eq!(length(l1), 5);
}

fn horner_eval(x: i32, coef: List) -> i32 {
    coef.reverse().into_iter().fold(0, |mut acc, cur| {
        acc *= x;
        if let Ok(cur) = (cur as Box<dyn Any>).downcast::<i32>() {
            acc += *cur;
        }
        acc
    })
}

#[test]
fn test_2_34() {
    let coef = list!(1, 3, 0, 5, 0, 1);
    assert_eq!(79, horner_eval(2, coef));
}

fn count_leaves(l: List) -> i32 {
    l.into_iter()
        .map(|cur| {
            if let Ok(ll) = (cur as Box<dyn Any>).downcast::<List>() {
                return count_leaves(*ll);
            }
            1
        })
        .collect::<List>()
        .into_iter()
        .fold(0, |mut acc, cur| {
            acc += *(cur as Box<dyn Any>).downcast::<i32>().unwrap();
            acc
        })
}

#[test]
fn test_2_35() {
    let a = list!(list!(1, 2), 3, 4);
    assert_eq!(a.len(), 3);
    assert_eq!(count_leaves(a), 4);

    let aa = list!(list!(1, 2), 3, 4);
    let ab = list!(list!(1, 2), 3, 4);
    let b = list!(aa, ab);
    assert_eq!(count_leaves(b), 8);
}
