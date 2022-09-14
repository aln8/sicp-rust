use crate::utils::ops::*;
use num::traits::{One, Zero};
use std::collections::HashSet;
use std::fmt::Debug;
use std::ops::{Add, Mul};

fn accumulate_filter<T, FT, FN, FF>(
    mut now: T,
    end: T,
    term: FT,
    next: FN,
    combiner: fn(T, T) -> T,
    filter: FF, // true for filter out
    null_value: T,
) -> T
where
    T: PartialOrd + Copy + Debug,
    FT: Fn(T) -> T,
    FN: Fn(T) -> T,
    FF: Fn(T) -> bool,
{
    if now > end {
        return null_value;
    }

    if filter(now) {
        return accumulate_filter(next(now), end, term, next, combiner, filter, null_value);
    }

    combiner(
        term(now),
        accumulate_filter(next(now), end, term, next, combiner, filter, null_value),
    )
}

fn accumulate_filter_iter<T, FT, FN, FF>(
    mut now: T,
    end: T,
    term: FT,
    next: FN,
    combiner: fn(T, T) -> T,
    filter: FF, // true for filter out
    null_value: T,
) -> T
where
    T: PartialOrd + Copy + Debug,
    FT: Fn(T) -> T,
    FN: Fn(T) -> T,
    FF: Fn(T) -> bool,
{
    let mut result = null_value;
    while now <= end {
        if !filter(now) {
            result = combiner(term(now), result);
        }
        now = next(now);
    }
    result
}

fn eular_prime(n: i32) -> HashSet<i32> {
    let mut primes_map = vec![true; (n + 1) as usize];
    let mut primes = HashSet::<i32>::new();

    for i in 2..(n + 1) {
        if primes_map[i as usize] {
            // find prime
            primes.insert(i);
        }

        // from 2 to current i, mark each i * 2..i as non prime
        // this method guarantee each non-prime only mark once
        // so it's faster than eratos methods
        for j in 2..i + 1 {
            let cur = i * j;
            if cur > n {
                break;
            }
            primes_map[cur as usize] = false;
        }
    }
    primes
}

// combine = a + b
// term = square(now)
// next = now + 1
// filter = is_in_prime_set
fn prime_square_sum(start: i32, end: i32) -> i32 {
    let prime_set = eular_prime(end);

    accumulate_filter(
        start,
        end,
        |now| square(now),
        |now| now + 1,
        |a, b| a + b,
        |now| !prime_set.contains(&now),
        0,
    )
}

fn prime_square_sum_iter(start: i32, end: i32) -> i32 {
    let prime_set = eular_prime(end);

    accumulate_filter_iter(
        start,
        end,
        |now| square(now),
        |now| now + 1,
        |a, b| a + b,
        |now| !prime_set.contains(&now),
        0,
    )
}

#[test]
fn test_prime_square_sum() {
    assert_eq!(339, prime_square_sum(7, 13));
}

#[test]
fn test_prime_square_sum_iter() {
    assert_eq!(339, prime_square_sum_iter(7, 13));
}

// gcb(a, b) = gcd(b, a mod b)
fn gcd(mut a: i32, mut b: i32) -> i32 {
    if b > a {
        // swap
        (a, b) = (b, a);
    }

    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

// combine: a * b
// term:   now
// next: now + 1
// filter gcd(now, n) == 1
fn product_relative_prime(n: i32) -> i32 {
    accumulate_filter(
        1,
        n,
        |now| now,
        |now| now + 1,
        |a, b| a * b,
        |now| gcd(now, n) != 1,
        1,
    )
}

fn product_relative_prime_iter(n: i32) -> i32 {
    accumulate_filter_iter(
        1,
        n,
        |now| now,
        |now| now + 1,
        |a, b| a * b,
        |now| gcd(now, n) != 1,
        1,
    )
}

#[test]
fn test_product_relative_prime_iter() {
    assert_eq!(24, product_relative_prime_iter(5));
    assert_eq!(189, product_relative_prime_iter(10));
}
