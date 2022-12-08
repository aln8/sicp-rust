use crate::{list, utils::list::List};

fn square_tree(mut l: List) -> List {
    fn inline_square_tree(list: &mut List) {
        if let Some(mut car) = list.car_ref::<i32>() {
            list.set_car((*car) * (*car));
        }
        if let Some(car) = list.car_mut::<List>() {
            inline_square_tree(list.car_mut().unwrap());
        }
        if let Some(cdr) = list.cdr_mut() {
            inline_square_tree(cdr);
        }
    }
    inline_square_tree(&mut l);
    l
}

#[test]
fn test_2_30_31() {
    let tree = list!(1, list!(2, list!(3, 4), 5), list!(6, 7));
    let expect = list!(1, list!(4, list!(9, 16), 25), list!(36, 49));

    assert!(test_tree_helper(&square_tree(tree), &expect));
}

fn subsets(mut l: List) -> List {
    fn clone_i32_list(l: &List) -> List {
        if l.cdr_ref().is_none() {
            return List::new(l.car_ref::<i32>().unwrap().clone());
        }
        let mut new = List::new(l.car_ref::<i32>().unwrap().clone());
        new.set_cdr(Some(clone_i32_list(l.cdr_ref().unwrap())));
        new
    }

    if l.cdr_ref().is_none() {
        return List::new(l);
    }

    let mut rest = subsets(l.cdr().unwrap());
    let car = l.car_ref::<i32>().unwrap().clone();

    let mut dummy = List::default();
    for val in rest.iter() {
        let list = val.as_ref_any().downcast_ref::<List>().unwrap();
        let mut new = clone_i32_list(list);
        new.tail().set_cdr(Some(List::new(car)));
        dummy.tail().set_cdr(Some(List::new(new)));
    }
    dummy.tail().set_cdr(Some(List::new(List::new(car))));
    rest.tail().set_cdr(Some(dummy.cdr().unwrap()));
    rest
}

#[test]
fn test_2_32() {
    let l = list!(1, 2, 3);
    let expect = list!(
        list!(3),
        list!(3, 2),
        list!(2),
        list!(3, 1),
        list!(3, 2, 1),
        list!(2, 1),
        list!(1)
    );
    assert!(test_tree_helper(&subsets(l), &expect))
}

fn test_tree_helper(l1: &List, l2: &List) -> bool {
    let mut i1 = l1.iter();
    let mut i2 = l2.iter();
    loop {
        let mut x = match i1.next() {
            None => return i2.next().is_none(),
            Some(val) => val,
        };

        let mut y = match i2.next() {
            None => return false,
            Some(val) => val,
        };

        match x.as_ref_any().downcast_ref::<i32>() {
            None => {
                if y.as_ref_any().downcast_ref::<i32>().is_some() {
                    return false;
                }
            }
            Some(x) => match y.as_ref_any().downcast_ref::<i32>() {
                Some(y) => {
                    if x != y {
                        return false;
                    }
                }
                None => return false,
            },
        }

        match x.as_ref_any().downcast_ref::<List>() {
            None => {
                if y.as_ref_any().downcast_ref::<List>().is_some() {
                    return false;
                }
            }
            Some(x) => match y.as_ref_any().downcast_ref::<List>() {
                Some(y) => {
                    if !test_tree_helper(x, y) {
                        return false;
                    }
                }
                None => return false,
            },
        }
    }
}
