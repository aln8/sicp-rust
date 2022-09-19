use crate::utils::ops::*;
use rand::Rng;
use test::Bencher;

fn exhaustive_prime_test(n: i32) -> bool {
    if n < 2 {
        return false;
    }

    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// log(n) complexity calculate exponent
fn fast_expmod(mut base: i64, mut exp: i64, m: i64) -> i64 {
    if exp == 0 {
        return 1;
    }

    let mut re = 1;
    base = base % m;
    while exp != 0 {
        if is_even(exp) {
            base = (base * base) % m;
            exp = exp / 2;
            continue;
        }
        re = base * re % m;
        exp = exp - 1;
    }
    return re % m;

    // base = base % m;
    // // (base ^ exp) % m =
    // // even: (base ^ (exp/2) % m)^2 % m
    // if is_even(exp) {
    //     return square(fast_expmod(base, exp / 2, m)) % m;
    //     // return (fast_expmod(base * base, exp / 2, m)) % m;
    // }
    // // odd: base * base^(exp-1)
    // (base * fast_expmod(base, exp - 1, m)) % m
}

// test use fermat little theory
// p is prime, a ^ p mod p  == a mod p
// same to a^(p-1) mod p == 1 mod p
fn fast_prime_test(p: i64, n: i64) -> bool {
    for _ in 0..n + 1 {
        if !fermat_random_test(p) {
            return false;
        }
    }
    true
}

fn fermat_random_test(p: i64) -> bool {
    if p <= 2 {
        return true;
    }
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(2..p);
    if fast_expmod(a, p - 1, p) != 1 {
        return false;
    }
    true
}

#[test]
fn test_exhaustive_prime_test() {
    assert_eq!(true, exhaustive_prime_test(31));
    assert_eq!(false, exhaustive_prime_test(15));
    assert_eq!(true, exhaustive_prime_test(19));
}

#[test]
fn test_fast_prime_test() {
    assert_eq!(true, fast_prime_test(23, 3));
    assert_eq!(false, fast_prime_test(4, 4));
    assert_eq!(true, fast_prime_test(3, 4));
}

#[test]
fn test_fast_expmod() {
    assert_eq!(2, fast_expmod(2, 3, 3));
    assert_eq!(1, fast_expmod(4, 4, 3));
    assert_eq!(0, fast_expmod(3, 3, 3));
}

#[bench]
fn bench_12_exhaustive_1000_prime_test(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..12 {
            test::black_box(exhaustive_prime_test(test::black_box(997)));
        }
    });
}

// toooooo slow
// #[bench]
// fn bench_12_exhaustive_100000000_prime_test(b: &mut Bencher) {
//     b.iter(|| {
//         for _ in 0..12 {
//             test::black_box(exhaustive_prime_test(test::black_box(9999991)));
//         }
//     });
// }

#[bench]
fn bench_12_fermat_1000_prime_test(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..12 {
            test::black_box(fast_prime_test(test::black_box(997), 12));
        }
    });
}

#[bench]
fn bench_12_fermat_100000000_prime_test(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..12 {
            test::black_box(fast_prime_test(test::black_box(9999991), 12));
        }
    });
}
