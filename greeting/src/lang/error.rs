
fn main() {
    test_error1();
    test_error2();
    test_error3();
}

fn test_error1() {
    let data = test_inner0(1);
    if let Ok(re) = data {
        println!("test_error ok {}", re);
    } else if let Err(b) = data {

        println!("test_error err {}", b);
    } else {
        println!("dataadf");
    }
}

fn test_inner0(data: i32) -> Result<String, bool> {
    let data = test_inner1(data)?;
    Ok(data)
}

fn test_inner1(data: i32) -> Result<String, bool> {
    if data > 12 {
        Ok(String::from("haode"))
    } else {
        Err(false)
    }
}

fn test_error2() {
    let a = test_inner1(2).unwrap();

    println!("test_error2 {}", a);
}

fn test_error3() {
    panic!("出现错误");
}