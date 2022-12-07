use crate::{list, utils::list::List};

fn count_leaves(list: &List) -> usize {
    // recursion
    fn cnt_lvs(list: Option<&List>) -> usize {
        match list {
            None => 0,
            Some(list) => {
                if list.car_ref::<List>().is_none() {
                    return 1 + cnt_lvs(list.cdr_ref());
                }
                cnt_lvs(list.car_ref()) + cnt_lvs(list.cdr_ref())
            }
        }
    }
    cnt_lvs(Some(list))

    // loop
    // let mut len = 0;
    // let mut cur = Some(list);
    // while cur.is_some() {
    //     let cur_list = cur.unwrap();
    //     if let Some(car) = cur_list.car_ref::<List>() {
    //         len += count_leaves(car);
    //     } else {
    //         len += 1;
    //     }
    //     cur = cur_list.cdr_ref();
    // }
    // len
}

#[test]
fn test_count_leaves() {
    let a = list!(list!(1, 2), 3, 4);
    assert_eq!(a.len(), 3);
    assert_eq!(count_leaves(&a), 4);
    let aa = list!(list!(1, 2), 3, 4);
    let b = list!(a, aa);
    assert_eq!(count_leaves(&b), 8);
}

#[test]
fn test_2_25() {
    let list1 = list!(1, 3, list!(5, 7), 9);
    // cdr(cdr(car(cdr(car))))
    let l1_7 = list1
        .cdr_ref()
        .unwrap()
        .cdr_ref()
        .unwrap()
        .car_ref::<List>()
        .unwrap()
        .cdr_ref()
        .unwrap()
        .car_ref::<i32>()
        .unwrap();
    assert_eq!(l1_7, &7);

    let list2 = list!(list!(7));
    // car(car)
    let l2_7 = list2.car_ref::<List>().unwrap().car_ref::<i32>().unwrap();
    assert_eq!(l2_7, &7);

    let list3 = list!(1, list!(2, list!(3, list!(4, list!(5, list!(6, 7))))));
    // (cdr(car))^6
    let l3_7 = list3
        .cdr_ref()
        .unwrap()
        .car_ref::<List>()
        .unwrap()
        .cdr_ref()
        .unwrap()
        .car_ref::<List>()
        .unwrap()
        .cdr_ref()
        .unwrap()
        .car_ref::<List>()
        .unwrap()
        .cdr_ref()
        .unwrap()
        .car_ref::<List>()
        .unwrap()
        .cdr_ref()
        .unwrap()
        .car_ref::<List>()
        .unwrap()
        .cdr_ref()
        .unwrap()
        .car_ref::<i32>()
        .unwrap();
    assert_eq!(l3_7, &7);
}

fn deep_reverse(mut list: List) -> List {
    // handle car List deep reverse
    if list.car_ref::<List>().is_some() {
        let car = deep_reverse(list.car::<List>().unwrap());
        list.set_car(car);
    }

    if list.cdr_ref().is_none() {
        return list;
    }

    // break link, list to Nil, hold next
    let mut head = deep_reverse(list.set_cdr(None).unwrap());
    head.tail().set_cdr(Some(list));
    head
}

#[test]
fn test_2_27() {
    let list = list!(list!(1, 2), list!(3, 4));
    let expect = vec![4, 3, 2, 1];
    let mut exp_idx = 0;
    for l in deep_reverse(list).iter_downcast::<List>() {
        for val in l.iter() {
            assert_eq!(val, &expect[exp_idx]);
            exp_idx += 1;
        }
    }
}

fn fringe(l: List) -> List {
    let mut dummy = List::default();
    let mut cur = Some(l);
    while let Some(mut ll) = cur {
        cur = ll.cdr();
        if ll.car_ref::<List>().is_some() {
            dummy.tail().set_cdr(Some(fringe(ll.car().unwrap())));
        } else {
            // this move must after ll.cdr()
            dummy.tail().set_cdr(Some(ll));
        }
    }
    dummy.cdr().unwrap()
}

#[test]
fn test_2_28() {
    let l = list!(list!(1, 2), list!(3, 4));
    let expect = [1, 2, 3, 4];
    assert!(fringe(l).iter().eq(expect.iter()));

    let l1 = list!(list!(1, 2), list!(3, 4));
    let l2 = list!(list!(1, 2), list!(3, 4));
    let l = list!(l1, l2);
    let expect = [1, 2, 3, 4, 1, 2, 3, 4];
    assert!(fringe(l).iter().eq(expect.iter()));
}
