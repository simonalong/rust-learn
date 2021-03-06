
fn main() {
    test_string_1();
    test_string2();
    test_string3();
}

fn test_string_1() {
    // 字符串转换为字节
    let sdf = b"asdfadf";
    println!("{:?}", sdf);
}

// 字符串的循环
fn test_string2() {
    let data = "zhou";

    for c in data.chars() {
        println!("{}", c);
    }
}

fn test_string3() {
    let data = "zhou";

    // Some('h')
    println!("{:?}", data.chars().nth(1));
}