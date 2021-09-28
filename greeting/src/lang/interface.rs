use std::{collections::HashMap, iter::repeat_with};


fn main() {
    test_1();
    test_2();
    test_3();
    test_4();
}

trait my_interface {
    fn test_fun(&self) -> i32;
}

struct MyPeople {
    name: String,
    age: i32
}

impl my_interface for MyPeople {
    fn test_fun(&self) -> i32 {
        self.age
    }
}

// 特性提供默认的函数实现
trait my_interface2{
    fn test_fun2(&self) -> i32 {
        323
    }
}

impl my_interface2 for MyPeople {

}

fn test_1() {
    let data = MyPeople{age: 33, name: String::from("zhou")};

    println!("{}", data.test_fun());
}

fn test_2() {
    let data = MyPeople{name: String::new(), age: 32};
    println!("{}", data.test_fun2());
}

trait Comparable{
    fn compare(&self, object: &Self) -> i8;
}

struct MyClass2 {
    age: i32,
    name: String
}

impl Comparable for MyClass2 {
    
    fn compare(&self, object: &Self) -> i8 { 
        if &self.age > &object.age {
            1
        } else if &self.age == &object.age {
            0
        } else {
            -1
        }
    }
}

fn test_3() {
    let data1 = MyClass2{age: 112, name: String::from("zhou")};
    let data2 = MyClass2{age: 21, name: String::from("zhou")};

    println!("compare {}",  data1.compare(&data2));

    let inc = |num| {
        num + 1
    };
    println!("inc(5) = {}", inc(5));
}

// 测试闭包
fn test_4() {
    let data = |num:i32| -> i32{
        num +1
    };

    // 22
    println!("{}", data(21));

    let data = |num|{
        num +1
    };
    // 22
    println!("{}", data(21));

    let add = |a, b|{
        a+b
    };
    // 22
    println!("{}", add(2, 3));
}