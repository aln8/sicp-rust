use std::collections::HashMap;
use test::Bencher;

// find smallest divider with prime methods
// eratosthenes method
fn eratos_prime(n: i32) -> Vec<i32> {
    let mut prime_map: HashMap<i32, bool> = HashMap::new();
    let mut i = 2;
    let mut primes = vec![];
    // mark all nums prime
    for k in 2..n + 1 {
        prime_map.insert(k, true);
    }
    // only check sqrt(n) times, because after i * i, it will marked
    while i * i < n {
        match prime_map.get(&i) {
            Some(is_prime) => {
                if *is_prime {
                    let mut j = i + i;
                    while j <= n {
                        // mark non prime
                        prime_map.insert(j, false);
                        j = j + i;
                    }
                }
            }
            _ => {}
        }
        i = i + 1;
    }
    for (k, v) in prime_map.into_iter() {
        if v {
            primes.push(k);
        }
    }
    primes.sort();
    primes
}

// using eratos prime method to check if prime is divider
fn eratos_smallest_divider(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    let mut primes = vec![true; (n + 1) as usize];
    let mut i = 2;
    while i * i < n {
        // handle prime
        if primes[i as usize] {
            // found prime mod 0, return
            if n % i == 0 {
                return i;
            }

            // not found, mark non prime and go next iter
            let mut j = i + i;
            while j <= n {
                primes[j as usize] = false;
                j += i
            }
        }
        i += 1;
    }

    return n;
}

fn eular_prime(n: i32) -> Vec<i32> {
    let mut primes_map = vec![true; (n + 1) as usize];
    let mut primes = Vec::<i32>::new();

    for i in 2..(n + 1) {
        if primes_map[i as usize] {
            // find prime
            primes.push(i);
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

fn eular_smallest_divider(n: i32) -> i32 {
    let mut primes_map = vec![true; (n + 1) as usize];

    for i in 2..(n + 1) {
        if primes_map[i as usize] {
            // current prime
            if n % i == 0 {
                return i;
            }
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
    n
}

#[test]
fn test_eratos_prime() {
    assert_eq!(vec![2, 3, 5, 7], eratos_prime(10));
    assert_eq!(vec![2, 3, 5, 7, 11, 13, 17, 19], eratos_prime(19));
}

#[test]
fn test_eratos_smallest_divider() {
    assert_eq!(2, eratos_smallest_divider(10));
    assert_eq!(3, eratos_smallest_divider(15));
    assert_eq!(19, eratos_smallest_divider(19));
}

#[test]
fn test_eular_prime() {
    assert_eq!(vec![2, 3, 5, 7], eular_prime(10));
    assert_eq!(vec![2, 3, 5, 7, 11, 13, 17, 19], eular_prime(19));
}

#[test]
fn test_eular_smallest_divider() {
    assert_eq!(2, eular_smallest_divider(10));
    assert_eq!(3, eular_smallest_divider(15));
    assert_eq!(19, eular_smallest_divider(19));
}

#[bench]
fn bench_100_eratos_prime(b: &mut Bencher) {
    b.iter(|| {
        let n = test::black_box(100);
        for _ in 0..n {
            eratos_prime(1000);
        }
    });
}

#[bench]
fn bench_100_eular_prime(b: &mut Bencher) {
    b.iter(|| {
        let n = test::black_box(100);
        for _ in 0..n {
            eular_prime(1000);
        }
    });
}
