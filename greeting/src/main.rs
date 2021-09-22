fn main() {
    test_print();
    test_print1();
    test_print2();
    test_print3();
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
