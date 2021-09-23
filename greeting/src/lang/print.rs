fn main() {
    test_print();
    test_print1();
    test_print2();
    test_print3();
    test_print4();
    test_print5();
    test_print6();
    test_print7();
}

fn test_print() {
    println!("we are familly");
}

// 测试
fn test_print1() {
    let a = "test_print1";
    println!("Hello, {}!", a);
}

fn test_print2() {
    let a = "test_print2";
    println!("nihao {0}, haode {0}", a);
}

// 输出{}这个字符
fn test_print3() {
    println!("haode {{}}");
}

// 测试变量
fn test_print4() {
    let mut b: i32 = 12;
    println!("test_print4-1 {}", b);
    b = 32;

    println!("test_print4-2 {}", b);
}

// 测试常量
fn test_print5() {
    const A:i32 = 12;
    println!("test_print5 {}", A);
    // let a = 13;
    // print!("test_print5 {}", a);
}

// 测试重影
fn test_print6() {
    let a = 12;
    println!("test_print6 {}", a);
    let a = 13;
    println!("test_print6 {}", a);
}

// 测试不可变变量
fn test_print7() {
    let a = 12;
    println!("test_print7 {}", a);
    // 下面有问题
    // a = 13;
    // println!("test_print6 {}", a);
}

