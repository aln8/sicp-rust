fn yanghui_rec(row: i32, col: i32) -> i32 {
    if col == 1 || row == col {
        return 1;
    }
    return yanghui_rec(row - 1, col - 1) + yanghui_rec(row - 1, col);
}

#[test]
fn test_ex1_12() {
    assert_eq!(1, yanghui_rec(1, 1));
    assert_eq!(6, yanghui_rec(5, 3));
}
