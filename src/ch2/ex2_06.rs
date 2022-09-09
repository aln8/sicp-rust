fn zero<T>(f: &Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T> {
    Box::new(move |x| x)
}

fn add_1<T>(
    n: fn(&Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T + '_>,
    f: Box<dyn Fn(T) -> T>,
) -> Box<dyn Fn(T) -> T>
where
    T: 'static,
{
    Box::new(move |x| f(n(&f)(x)))
}

#[test]
fn test_add_1_zero() {
    assert_eq!(4, add_1(zero, Box::new(|x| x * 2))(2));
}

fn one<T>(f: &Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T + '_>
where
    T: 'static,
{
    Box::new(move |x| f(x))
}

#[test]
fn test_one() {
    let f: Box<dyn Fn(i32) -> i32> = Box::new(move |x| x * 2);
    assert_eq!(4, one(&f)(2));
    let one_re = one(&f)(2);
    let add1_zero_re = add_1(zero, f)(2);
    assert_eq!(one_re, add1_zero_re);
}

fn two<T>(f: &Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T + '_>
where
    T: 'static,
{
    Box::new(move |x| f(f(x)))
}

#[test]
fn test_two() {
    let f: Box<dyn Fn(i32) -> i32> = Box::new(move |x| x * 2);
    let two_re = two(&f)(2);
    let add1_one_re = add_1(one, f)(2);
    assert_eq!(two_re, add1_one_re);
}
