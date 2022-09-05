fn cons<T>(x: T, y: T) -> Box<dyn Fn(Box<dyn Fn(T, T) -> T>) -> T + 'static>
where
    T: Copy + 'static,
{
    Box::new(move |f: Box<dyn Fn(T, T) -> T>| f(x, y))
}

fn car<T>(z: Box<dyn Fn(Box<dyn Fn(T, T) -> T>) -> T + '_>) -> T {
    z(Box::new(|p: T, q: T| p))
}

fn cdr<T>(z: Box<dyn Fn(Box<dyn Fn(T, T) -> T>) -> T + '_>) -> T {
    z(Box::new(|p: T, q: T| q))
}

#[test]
fn test_car() {
    let a = cons(3, 5);
    assert_eq!(3, car(Box::new(&a)));
    assert_eq!(5, cdr(Box::new(&a)));
}
