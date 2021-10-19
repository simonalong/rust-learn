
///
///
/// 这个概念与go里面的slice有点像，不过go里面这个切片是可变数组，但是在Rust这里，切片就是把结构中的数据进行切片处理
///
/// ### 测试：切片
fn main() {
    test_slice_1();
    test_slice_2();
    test_slice_3();
    test_slice_4();
    test_slice_array();
}

// 测试切片：字符串的切片
fn test_slice_1() {
    let a = String::from("nihao");

    // 下面这种切片就是错的，切片就是要做借用才行
    //let b = a[0..2];
    let b = &a[0..2];
    println!("test_slice_1 {}", b);
}

// 测试切片：..的使用
#[test]
fn test_slice_2() {
    let a = String::from("womendoushi");

    let b = &a[..1]; // 等价于[0..1]
    // let b = &a[1..];  // 等价于[1..end]
    // let b = &a[..];  // 等价于&a
    println!("test_slice_1 {}", b);
}

// 测试切片：一旦数据被分片租借，则不可在修改原始的数据
fn test_slice_3() {
    let a = String::from("nihao");

    let tem = &a[..2];
    // 下面这句话是有问题的
    //a.push_str(" haode");

    println!("test_slise_3 {}", a);
}

// 测试字符串的转换
fn test_slice_4() {

    let b = &String::from("nihao")[..];

    println!("test_slice_4 {}", b);

    let c = String::from(b);

    println!("test_slice_4 2 {}", c);
}

// 测试分片：数组
fn test_slice_array() {
    let a= [3, 123, 43, 23];
    let b = &a[..3];
    for data in b.iter() {
        println!("test_slice_arrya {}", data);
    }
}
