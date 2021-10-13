pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[test]
fn a() {
    assert_eq!(4, add_two(2));
}

#[test]
fn b() {
    assert_eq!(5, add_two(3));
}
