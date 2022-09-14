use std::mem::replace;

// simple cons rust implementation
struct Cons<T, F> {
    car: T,
    cdr: F,
}

impl<T, F> Cons<T, F> {
    pub fn new(car: T, cdr: F) -> Cons<T, F> {
        Cons { car, cdr }
    }

    pub fn cdr(&self) -> &F {
        &self.cdr
    }

    pub fn mut_cdr(&mut self) -> &mut F {
        &mut self.cdr
    }

    pub fn set_cdr(&mut self, cdr: F) {
        self.cdr = cdr
    }

    pub fn car(&self) -> &T {
        &self.car
    }

    pub fn set_car(&mut self, car: T) {
        self.car = car
    }
}

#[test]
fn test_cons() {
    let pair = Cons::new(1, 2);
    assert_eq!(&1, pair.car());
}

enum List<T> {
    Cons(Cons<T, Box<List<T>>>),
    Nil,
}

impl<T> List<T> {
    fn new(car: T) -> List<T> {
        List::Cons(Cons::new(car, Box::new(List::Nil)))
    }

    fn car(&self) -> Option<&T> {
        if let List::Cons(cons) = self {
            Some(cons.car())
        } else {
            None
        }
    }

    fn set_car(&mut self, car: T) {
        if let List::Cons(cons) = self {
            cons.set_car(car)
        }
    }

    fn cdr(&self) -> &List<T> {
        if let List::Cons(cons) = self {
            cons.cdr()
        } else {
            &List::Nil
        }
    }

    fn set_cdr(&mut self, cdr: List<T>) {
        if let List::Cons(cons) = self {
            cons.set_cdr(Box::new(cdr))
        }
    }

    fn next_node(&self) -> &List<T> {
        if let List::Cons(cons) = self {
            cons.cdr()
        } else {
            &List::Nil
        }
    }

    fn mut_next_node(&mut self) -> &mut List<T> {
        if let List::Cons(cons) = self {
            cons.mut_cdr()
        } else {
            self
        }
    }
}

#[macro_export]
macro_rules! list {
    ( $first:expr, $( $a:expr ),* ) => {
        {
            let mut head = List::new($first);
            let mut cur = &mut head;
            $(
                let next = List::new($a);
                cur.set_cdr(next);
                cur = cur.mut_next_node();
            )*
            head
        }
    };
}

impl<'a, T> Iterator for &'a List<T>
where
    T: 'a + Copy,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            List::Cons(cons) => {
                *self = cons.cdr();
                return Some(cons.car());
            }
            List::Nil => None,
        }
    }
}

#[test]
fn test_list_macro() {
    let a = list!(1, 2, 3, 4);
    assert_eq!(&1, a.car().unwrap());
    assert_eq!(&2, a.next_node().car().unwrap());
    assert_eq!(&3, a.next_node().next_node().car().unwrap());
    assert_eq!(&4, a.next_node().next_node().next_node().car().unwrap());
    assert!(a
        .next_node()
        .next_node()
        .next_node()
        .next_node()
        .car()
        .is_none());
}

#[test]
fn test_list() {
    let test_list = [1, 3, 2, 4];
    let mut test_idx = 0;
    let mut head = &List::Cons(Cons::new(
        1,
        Box::new(List::Cons(Cons::new(3, Box::new(List::Nil)))),
    ));

    // direct loop
    while let List::Cons(cons) = head {
        head = cons.cdr();
        assert_eq!(&test_list[test_idx], cons.car());
        test_idx += 1;
    }

    // iteration method
    let mut test_idx = 0;
    for val in head {
        assert_eq!(&test_list[test_idx], val);
        test_idx += 1;
    }
}
