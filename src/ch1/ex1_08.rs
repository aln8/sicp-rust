use crate::utils::ops::*;

// if (valid)
//     guess
//     then next approximate
//
//                    curt
//                     |
//                  curt_iter
//                 /        \
//             valid     approximate
//             /   \
//         cubed   abs
//
pub fn curt(x: i64) -> f64 {
    curt_iter(x as f64, 1.0)
}

fn curt_iter(x: f64, now: f64) -> f64 {
    if valid(x, now) {
        return now;
    }

    let next = approximate(x, now);
    curt_iter(x, next)
}

fn approximate(x: f64, now: f64) -> f64 {
    ((x / (now * now)) + 2.0 * now) / 3.0
}

fn valid(x: f64, now: f64) -> bool {
    abs(cube(now) - x) < 0.001
}

#[test]
fn test_ex1_08() {
    fn good(expect: f64, real: f64) -> bool {
        let mind = real - expect;
        if mind > 0.0 {
            return mind < 0.001;
        }
        return mind > -0.001;
    }

    assert_eq!(true, good(2.0, curt(8)));
    assert_eq!(true, good(-2.0, curt(-8)));
}
