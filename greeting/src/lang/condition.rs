///
/// 
/// 测试：if else
/// 
/// 
fn main() {
    test_if();
    test_if2();
}

fn test_if() {
    let a = 12;
    if a > 1 && a < 10 {
        println!("大于1小于10");
    } else if a >= 10 && a < 20 {
        println!("大于10 小于20");
    } else {
        println!("未知");
    }
}

// 测试if的一行
fn test_if2() {
    let a = 12;
    let num = if a > 12 {1} else {0};

    println!("jieguo: {}", num);
}