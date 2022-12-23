use crate::{
    list,
    utils::{cons::Cons, list::List},
};

#[derive(PartialEq, Debug, Clone)]
struct MobileList {
    data: List,
}

#[derive(Debug, PartialEq, Clone)]
enum StructureList {
    MobileList(MobileList),
    Weight(i32),
}

#[derive(PartialEq, Debug, Clone)]
struct BranchList {
    data: List,
}

impl MobileList {
    fn new(left: BranchList, right: BranchList) -> MobileList {
        MobileList {
            data: list!(left, right),
        }
    }

    fn left(mut self) -> BranchList {
        self.data.car().unwrap()
    }

    fn left_ref(&self) -> &BranchList {
        self.data.car_ref().unwrap()
    }

    fn right(mut self) -> BranchList {
        self.data.cdr().unwrap().car().unwrap()
    }

    fn right_ref(&self) -> &BranchList {
        self.data.cdr_ref().unwrap().car_ref().unwrap()
    }

    fn total_weight(&self) -> i32 {
        fn weight(st: &StructureList) -> i32 {
            match st {
                StructureList::Weight(w) => *w,
                StructureList::MobileList(mobile) => mobile.total_weight(),
            }
        }
        let (_, l_st) = self.left_ref().branch_struct_ref();
        let (_, r_st) = self.right_ref().branch_struct_ref();
        weight(l_st) + weight(r_st)
    }

    fn balanced(&self) -> bool {
        fn weight_balanced(st: &StructureList) -> (bool, i32) {
            match st {
                StructureList::Weight(w) => (true, *w),
                StructureList::MobileList(mobile) => (mobile.balanced(), mobile.total_weight()),
            }
        }
        let (l_len, l_st) = self.left_ref().branch_struct_ref();
        let (l_b, l_w) = weight_balanced(l_st);
        let (r_len, r_st) = self.right_ref().branch_struct_ref();
        let (r_b, r_w) = weight_balanced(r_st);
        if (!l_b || !r_b) {
            return false;
        }
        l_len * l_w == r_len * r_w
    }
}

impl BranchList {
    fn new(length: i32, structure: StructureList) -> BranchList {
        BranchList {
            data: list!(length, structure),
        }
    }

    fn branch_struct(mut self) -> (i32, StructureList) {
        (
            self.data.car().unwrap(),
            self.data.cdr().unwrap().car().unwrap(),
        )
    }

    fn branch_struct_ref(&self) -> (i32, &StructureList) {
        (
            *self.data.car_ref::<i32>().unwrap(),
            self.data.cdr_ref().unwrap().car_ref().unwrap(),
        )
    }
}

#[test]
fn test_list_total_weight() {
    let m = MobileList::new(
        BranchList::new(10, StructureList::Weight(10)),
        BranchList::new(
            10,
            StructureList::MobileList(MobileList::new(
                BranchList::new(3, StructureList::Weight(5)),
                BranchList::new(9, StructureList::Weight(3)),
            )),
        ),
    );
    assert_eq!(18, m.total_weight());
}

#[test]
fn test_list_balance() {
    let m = MobileList::new(
        BranchList::new(10, StructureList::Weight(10)),
        BranchList::new(
            10,
            StructureList::MobileList(MobileList::new(
                BranchList::new(3, StructureList::Weight(5)),
                BranchList::new(9, StructureList::Weight(3)),
            )),
        ),
    );
    assert_eq!(false, m.balanced());

    let m = MobileList::new(
        BranchList::new(5, StructureList::Weight(20)),
        BranchList::new(
            10,
            StructureList::MobileList(MobileList::new(
                BranchList::new(12, StructureList::Weight(4)),
                BranchList::new(8, StructureList::Weight(6)),
            )),
        ),
    );
    assert_eq!(true, m.balanced());
}

#[derive(PartialEq, Debug, Clone)]
struct MobileCons {
    data: Cons,
}

#[derive(Debug, PartialEq, Clone)]
enum StructureCons {
    MobileList(MobileCons),
    Weight(i32),
}

#[derive(PartialEq, Debug, Clone)]
struct BranchCons {
    data: Cons,
}

impl MobileCons {
    fn new(left: BranchCons, right: BranchCons) -> Self {
        Self {
            data: Cons::new(Some(Box::new(left)), Some(Box::new(right))),
        }
    }

    fn left(mut self) -> BranchCons {
        *self.data.car_downcast::<BranchCons>().unwrap()
    }

    fn left_ref(&self) -> &BranchCons {
        self.data.car_downcast_ref::<BranchCons>().unwrap()
    }

    fn right(mut self) -> BranchCons {
        *self.data.cdr_downcast::<BranchCons>().unwrap()
    }

    fn right_ref(&self) -> &BranchCons {
        self.data.cdr_downcast_ref::<BranchCons>().unwrap()
    }

    fn total_weight(&self) -> i32 {
        fn weight(st: &StructureCons) -> i32 {
            match st {
                StructureCons::Weight(w) => *w,
                StructureCons::MobileList(mobile) => mobile.total_weight(),
            }
        }
        let (_, l_st) = self.left_ref().branch_struct_ref();
        let (_, r_st) = self.right_ref().branch_struct_ref();
        weight(l_st) + weight(r_st)
    }

    fn balanced(&self) -> bool {
        fn weight_balanced(st: &StructureCons) -> (bool, i32) {
            match st {
                StructureCons::Weight(w) => (true, *w),
                StructureCons::MobileList(mobile) => (mobile.balanced(), mobile.total_weight()),
            }
        }
        let (l_len, l_st) = self.left_ref().branch_struct_ref();
        let (l_b, l_w) = weight_balanced(l_st);
        let (r_len, r_st) = self.right_ref().branch_struct_ref();
        let (r_b, r_w) = weight_balanced(r_st);
        if (!l_b || !r_b) {
            return false;
        }
        l_len * l_w == r_len * r_w
    }
}

impl BranchCons {
    fn new(length: i32, structure: StructureCons) -> Self {
        Self {
            data: Cons::new(Some(Box::new(length)), Some(Box::new(structure))),
        }
    }

    fn branch_struct(mut self) -> (i32, StructureCons) {
        (
            *self.data.car_downcast::<i32>().unwrap(),
            *self.data.cdr_downcast::<StructureCons>().unwrap(),
        )
    }

    fn branch_struct_ref(&self) -> (i32, &StructureCons) {
        (
            *self.data.car_downcast_ref::<i32>().unwrap(),
            self.data.cdr_downcast_ref::<StructureCons>().unwrap(),
        )
    }
}

#[test]
fn test_cons_total_weight() {
    let m = MobileCons::new(
        BranchCons::new(10, StructureCons::Weight(10)),
        BranchCons::new(
            10,
            StructureCons::MobileList(MobileCons::new(
                BranchCons::new(3, StructureCons::Weight(5)),
                BranchCons::new(9, StructureCons::Weight(3)),
            )),
        ),
    );
    assert_eq!(18, m.total_weight());
}

#[test]
fn test_cons_balance() {
    let m = MobileCons::new(
        BranchCons::new(10, StructureCons::Weight(10)),
        BranchCons::new(
            10,
            StructureCons::MobileList(MobileCons::new(
                BranchCons::new(3, StructureCons::Weight(5)),
                BranchCons::new(9, StructureCons::Weight(3)),
            )),
        ),
    );
    assert_eq!(false, m.balanced());

    let m = MobileList::new(
        BranchList::new(5, StructureList::Weight(20)),
        BranchList::new(
            10,
            StructureList::MobileList(MobileList::new(
                BranchList::new(12, StructureList::Weight(4)),
                BranchList::new(8, StructureList::Weight(6)),
            )),
        ),
    );
    assert_eq!(true, m.balanced());
}
