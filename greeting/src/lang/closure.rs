
fn main() {
    test_closure_1();
    test_closure_2();
    test_closure_3();
    test_closure_4();
}

// 作为函数直接调用
fn test_closure_1() {
    let data = || 42;
    println!("test_closure_1 data1 {}", data());

    let data = |x| 42 + x;
    println!("test_closure_1 data2 {}", data(1));

    let data = |x:String| x;
    println!("test_closure_1 data2 {}", data(String::from("asdf")));

    let data = |x:i32| -> i32{x + 32};
    println!("test_closure_1 data2 {}", data(12));
}

// 也可以在声明的地方直接调用
fn test_closure_2() {
    println!("test_closure_2 data1 {}", (|| 42)());

    println!("test_closure_2 data2 {}", (|x| 42 + x)(1));

    println!("test_closure_2 data2 {}", (|x:String| x)(String::from("asdf")));

    println!("test_closure_2 data2 {}", (|x:i32| -> i32{x + 32})(12));
}

/// 闭包在调用外部的变量的时候，会根据情况，将变量的所有权进行转移，为了更好的运行，因此有如下三种情况的变量会自动采用转移
/// 
/// - Fn：引用(&T)
/// - FnMut：可变引用(& mut T)
/// - FnOnce：值(T)
/// 
/// 转移会默认从上到下，因为约束从上到下越来越严格
/// 
fn test_closure_3() {
    // 引用转换
    let data = 12;
    // 这种可以复制的（算是基本类型的）会采用引用方式，使用后，data还是可以继续被访问
    let call = ||data;
    println!("test_closure_3 data = {}", call());
    // 可以调用
    call();

    // 可变引用转换
    let mut data = 12;
    // 这种可以复制的（算是基本类型的）会采用引用方式，使用后，data还是可以继续被访问
    let mut call = || {data += 1;data};
    println!("test_closure_3 data = {}", call());
    // 可以调用
    call();

    // 值转换
    let data = String::from("sdf");
    let call = ||data;
    println!("test_closure_3 data = {}", call());
    // 不可以调用
    //call();
}

// 闭包做函数入参
fn test_closure_4() {

}

