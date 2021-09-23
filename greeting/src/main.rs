
///
/// 
/// ### 测试：循环
/// 循环这里有三个，while，loop。比java、go之类的语言多一个loop
/// 
fn main() {
    test_while();
    test_for();
    test_for2();
    test_loop();
    test_loop2(321);
}

// 测试while
fn test_while() {
    let mut index = 0;
    while index < 10 {
        println!("index :{}", index);
        index += 1;
    }

    println!("finish");
}

// 测试for循环，for这个不能使用三元组那种循环方式，但是这个可以用在数组上面
fn test_for() {
    let array = [21, 32, 43, 23, 32, 3];

    // array.iter 表示的是数组的迭代器
    for data in array.iter() {
        println!("for data1 {}", data);
    }
}

// 测试for循环，数组下标
fn test_for2() {
    let array = [21, 32, 43, 23, 32, 3];

    // array.iter 表示的是数组的迭代器
    for index in 0..3 {
        println!("for data2 {}", array[index]);
    }
}

// 测试loop，这个loop其实相当于while(true)，也就是无限循环
fn test_loop() {
    let array = [1, 32, 43, 4, 54];
    let mut index = 0;
    loop {
        if index+1 >= array.len() {
            break;
        }
        println!("loop data = {}", array[index]);

        index+=1;
    }
}

// 测试loop，这个loop其实相当于while(true)，也就是无限循环
fn test_loop2(x: i32) {
    let array = [1, 32, 43, 4, 54];
    let mut index = 0;
    loop {
        if index+1 >= array.len() {
            index = 0;
        }
        if array[index] >= x {
            break;
        }
        println!("loop data2 = {}", array[index]);

        index+=1;
    }
}
