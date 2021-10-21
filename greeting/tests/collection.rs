

///
/// 集合
///
fn main() {
    test_vec_1();
    test_vec_2();
    test_vec_3();
    test_vec_4();
    test_vec_5();
    test_vec_6();
    test_vec_7();
}

// 集合创建后添加数据
fn test_vec_1() {
    let mut ve1 = Vec::new();
    ve1.push(12);
    ve1.push(132);

    println!("{:?}", ve1);
}

fn test_vec_2() {
    // 提供宏 vec!进行提供创建数组
    let ve1 = vec![12, 32, 3];

    println!("{:?}", ve1);
}

fn test_vec_3() {
    let mut v1 = vec![1, 2, 3];
    let mut v2 = vec![4, 5, 6];

    v1.append(&mut v2);

    println!("v1 {:?}", v1);
    println!("v2 {:?}", v2);
}

// 访问数组
fn test_vec_4() {
    let ve3 = vec![2, 32, 43, 5];

    if let Some(value) = ve3.get(2) {
        println!("{}", value);
    } else {
        println!("none");
    }
}

// 访问数组
fn test_vec_5() {
    let ve3 = vec![2, 32, 43, 5];

    for ele in ve3 {
        println!("{}", ele);
    }
}

// 访问数组
fn test_vec_6() {
    let ve3 = vec![2, 32, 43, 5];

    // 添加上借用后，下面的还能继续访问，否则下面的就不能访问了
    for ele in &ve3 {
        println!("{}", ele);
    }

    println!("====");
    for ele in &ve3 {
        println!("{}", ele);
    }
}

// 数组中进行循环更改
fn test_vec_7() {
    let mut v1 = [1, 3, 43, 5];

    for ele in &mut v1 {
        *ele += 100;
    }

    println!("{:?}", v1);
}
