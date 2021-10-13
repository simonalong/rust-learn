pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[test]
fn a1() {
    assert_eq!(4, add_two(2));
}

#[test]
fn a2() {
    assert_eq!(4, add_two(2));
}

#[test]
fn b_fun_test() {
    assert_eq!(5, add_two(3));
}

#[test]
fn b2_fun_test() {
    assert_eq!(5, add_two(3));
}
