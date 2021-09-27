///
/// 
/// ## 数据类型测试
/// let a = 12;
/// 
/// 
fn main() {
    test_int();
    test_float();
    test_bool();
    test_str();
    test_char();
    test_tup();
    test_tup2();
    test_array();
}

// 整数
fn test_int() {
    let a:i8 = 12;
    println!("test_int1 {}", a);

    let a:i16 = 12;
    println!("test_int2 {}", a);

    let a:i32 = 12;
    println!("test_int3 {}", a);

    let a:i64 = 12;
    println!("test_int4 {}", a);

    let a:i128 = 12;
    println!("test_int5 {}", a);
}

// 测试浮点数
fn test_float() {
    // 默认f64
    let a = 12.3;
    println!("test_float {}", a);

    let a:f32 = 12.3;
    println!("test_float {}", a);
}

// 测试bool类型
fn test_bool() {
    let a = true;
    println!("test_bool {}", a);

    let a = false;
    println!("test_bool {}", a);
}

// 测试str
fn test_str() {
    let a = "nihao";
    println!("test_str {}", a);

    let a = 12;
    println!("test_str {}", a);
}

// 测试char
fn test_char() {
    let a = 'c';
    println!("test_char {}", a);
}

// 测试元组
fn test_tup() {
    let a = (123, "womendoushi", 123.43);
    println!("test_tup.0 {}", a.0);
    println!("test_tup.1 {}", a.1);
    println!("test_tup.2 {}", a.2);
}

// 测试元组
fn test_tup2() {
    let a = (123, "womendoushi", 123.43);
    println!("test_tup.0 {}", a.0);
    println!("test_tup.1 {}", a.1);
    println!("test_tup.2 {}", a.2);

    let (age, name, money) = a;

    println!("test_tup2_1, {}", age);
    println!("test_tup2_2, {}", name);
    println!("test_tup2_3, {}", money);
}

// 测试数组
fn test_array() {
    // 普通用法
    let a = [12, 32, 43, 234,45, 34, 53, 4534, 65, 45645, 6];

    // 数组访问
    println!("test_array_a_0:{}", a[0]);
    println!("test_array_a_1:{}", a[1]);
    println!("test_array_a_2:{}", a[2]);

    // 设置数据的类型的声明方式
    let b:[i64; 3] = [12, 32, 43];
    println!("test_array_b_0:{}", b[0]);
    println!("test_array_b_1:{}", b[1]);

    // 简化使用，这个表示[0, 0, 0, 0]，其中[0;4]前者表示初始值，后者表示个数
    let c = [0;4];
    println!("test_array_c_0:{}", c[0]);
    println!("test_array_c_1:{}", c[1]);
    println!("test_array_c_2:{}", c[2]);
    println!("test_array_c_3:{}", c[3]);

    // 数组不可变，如果需要变化，则需要使用下面的
    // c[0] = 12;
    // println!("test_array_chg_c_0:{}", c[0]);

    // 要设置数组为变量的数组
    let mut c = [0;4];
    c[0] = 12;
    println!("test_array_mut_c_0:{}", c[0]);

    // 表示, f32类型的，初始值为0的，数组长度为4的变量
    let mut c = [0f32;4];
    c[0] = 12;
    println!("test_array_mut_c_0:{}", c[0]);
}