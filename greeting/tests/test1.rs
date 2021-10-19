pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[test]
fn add_two_and_two() {
    assert_eq!(4, add_two(2));
}

#[test]
fn add_three_and_two() {
    assert_eq!(5, add_two(3));
}



fn max_num(x: &i32, y: &i32) -> &i32 {
    if x > y {
        &x
    } else {
        &y
    }
}


#[test]
fn test1() {
    let x = 1;                // -------------+-- x start
    let max;                  // -------------+-- max start
    {                         //              |
        let y = 8;              // -------------+-- y start
        max = max_num(&x, &y);  //              |
    }                         // -------------+-- y over
    println!("max: {}", max); //              |
}

#[test]
fn test2() {
    let y = 8;                  // -------------+-- y start
    let x = 15;                  // -------------+-- x start
    let max = max_num(&x, &y);
    println!("max: {}", max); //              |
}
