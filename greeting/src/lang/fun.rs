///
///
/// 测试：函数
///
///
fn main() {
    test_fun1();
    test_fun2(12, 32);
    test_fun3("woomen", "haode");
    test_fun4();
    test_fun5();
    println!("{}", test_fun6(212, 3));
    test_fun7();
    test_fun7_2();
}

// 函数命名是小写字母和下划线进行拼接
pub fn test_fun1() {
    println!("test_fun1");
}

// 参数必须注明参数类型
fn test_fun2(x: i32, y: i32) {
    println!("test_fun1: {}, {}", x, y);
}

// 如果要求是字符串类型，则必须是指针才行
fn test_fun3(x: &str, y: &str) {
    println!("test_fun1: {}, {}", x, y);
}

// 复杂的块，最后的表达式表示返回结果
fn test_fun4() {
    let a = {
        let b = 12;
        let c = 32;
        b + c
    };

    println!("age is {}", a);
}

// 复杂的块，最后一个是表达式，表示的是整个函数块的返回值，注意最后一个表达式没有分号
fn test_fun5() {
    let a = {
        let b = 12;
        let c = 32;
        let d = b + c + 1;
        d
    };

    println!("age is {}", a);
}

// 测试函数返回值
fn test_fun6(x: i32, y: i32) -> i32 {
    println!("test_fun6 x: {}, y:{}", x, y);
    return x + y;
}

// 这个是函数体，也叫函数块，函数块不同于函数，函数块不能使用return关键字
fn test_fun7() {
    //
    fn test_inner() -> i32 {
        32
    }

    println!("test_fun7 {}", test_inner());
}

// 这个有点像匿名函数，然后这个函数的返回值是str类型，但是必须为static形式才行
fn test_fun7_2() {
    fn test_inner() -> &'static str {
        "haode"
    }

    println!("test_fun7_2 {}", test_inner());
}
