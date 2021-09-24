
///
/// 
/// ### 测试：所有权测试
/// 
fn main() {
    test_owner_1(); 
    test_owner_2();
    test_owner_3();
    test_owner_4();
    test_owner_5();
    test_owner_6();
    test_owner_7();
    test_owner_8();
    test_owner_9();
    test_owner_10();
    test_owner_11();
    test_owner_12();
    test_owner_13();
}

fn test_owner_1() {
    let s1 = String::from("haode");

    println!("test_owner1 {}", &s1);
}

fn test_owner_2() {
    let s1 = String::from("haode");
    // 赋值后，s1就失效了
    let s2 = s1;

    println!("test_owner_2 {}", &s2);
}

fn test_owner_3() {
    let s1 = String::from("haode");
    // 赋值后，s1就失效了
    let s2 = s1.clone();

    println!("test_owner_3 {}", &s1);
    println!("test_owner_3 {}", &s2);
}

// 基本变量赋值没有问题，因为基本类型在赋值的时候，其实是栈里面做了克隆
fn test_owner_4() {
    let s1 = 1;
    // 赋值后，s1就失效了
    let s2 = s1;

    println!("test_owner_4 {}", &s1);
    println!("test_owner_4 {}", &s2);
}

// 测试经过函数后的失效
fn test_owner_5() {
    let s1 = String::from("test_owner_5");

    // s1进去后就会失效了
    test_fun1(s1);

    // s1失效，这个时候，如下访问会有问题
    //println!("test_owner_4 {}", s1);
}

fn test_fun1(name: String) {
    println!("test_fun1 {}", name);
}

// 测试经过函数后的失效
fn test_owner_6() {
    let s1 = String::from("test_owner_6");

    // s1进去后就会失效了
    let s2 = test_fun2(s1);

    // s1失效，这个时候，如下访问会有问题
    println!("test_owner_6 {}", s2);
}

fn test_fun2(name: String) -> String {
    println!("test_fun2 {}", name);
    return String::from("haode");
}

// 测试租借
fn test_owner_7() {
    let a = String::from("test");

    let b = &a;

    println!("test_owner_7 {}", a);
    println!("test_owner_7 {}", b);
}

// 测试租借
fn test_owner_8() {
    let a = String::from("test");
    let b = &a;
    let c = a;

    // 现在b虽然借了a，但是b并没有a的所有权，而a后来把所有权给了c，因此b就是失效了
    //println!("test_owner_7 {}", a);
    //println!("test_owner_7 {}", b);
    println!("test_owner_8 {}", c);
}

// 测试租借：一旦被借走，则原来的值就不能修改数据了
fn test_owner_9() {
    let mut a = String::from("test");
    let b= &a;

    // 下面报错
    // a.push_str("newData");
    println!("test_owner_9 a: {}", a);
    println!("test_owner_9 a: {}", b);
}

// 测试租借：一旦被借走，则原来的值就不能修改数据了
fn test_owner_10() {
    let mut a = String::from("test");
    let b= &mut a;

    b.clear();
    b.push_str("newData");
    println!("test_owner_10 a: {}", b);
}

// 测试租借：只能借可变权给一个引用
fn test_owner_11() {
    let a = String::from("test");
    let b= &a;
    let c= &a;

    println!("test_owner_11 {}", a);
    println!("test_owner_11 {}", b);
    println!("test_owner_11 {}", c);
}

// 测试租借：只能借可变权给一个引用
fn test_owner_12() {
    let mut a = String::from("test");
    let b= &mut a;
    // 如果再租借个一个，则会报错
    // let c= &mut a;

    println!("test_owner_11 {}", b);
}

fn test_owner_13() {
    let data = test_fun3();
    println!("test_owner_13 {}", data);
}

fn test_fun3() -> String {
    return String::from("haode");
}

// fn test_owner_14() {
//     let data = test_fun3();
//     println!("test_owner_14 {}", data);
// }

// // 下面这种租借就有问题，或者说这总叫做悬垂引用，这种就是租借的值其实真的已经不再存在了
// fn test_fun4() -> &String {
//     return &String::from("haode");
// }