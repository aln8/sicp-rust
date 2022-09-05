fn zero<T>(f: &Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T + '_>
where
    T: 'static,
{
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

fn one<T>(f: &Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T + '_> {
    Box::new(move |x| f(x))
}

#[test]
fn test_add_1_one() {
    assert_eq!(8, add_1(one, Box::new(|x| x * 2))(2));
}

fn two<T>(f: &Box<dyn Fn(T) -> T>) -> Box<dyn Fn(T) -> T + '_> {
    Box::new(move |x| f(f(x)))
}

#[test]
fn test_add_1_two() {
    assert_eq!(16, add_1(two, Box::new(|x| x * 2))(2));
}
