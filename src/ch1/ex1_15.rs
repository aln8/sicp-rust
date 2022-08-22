fn sin(f: f32) -> f32 {
    if f < 0.1 {
        return f;
    }
    p(sin(f / 3.0))
}

fn cube(f: f32) -> f32 {
    return f * f * f;
}

fn p(f: f32) -> f32 {
    return 3.0 * f - 4.0 * cube(f);
}

#[test]
fn test_ex1_15() {
    assert_eq!(0.09, sin(0.09));
    assert_eq!(0.14044023, sin(3.0));
}
