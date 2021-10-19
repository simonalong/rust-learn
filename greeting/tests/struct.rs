///
///
/// 测试结构体
///
/// ### 测试：切片
#[test]
fn main() {
    // test_struct_1();
    // test_struct_2();
    // test_struct_3();
    // test_struct_4();
    // test_struct_5();
    // test_struct_6();
    // test_struct_7();
    // test_struct_8();
    // test_struct_9();
    test_struct_10();
}

#[derive(Debug)]
struct MyClass {
    name: String,
    age: i32,
}

// 测试结构体
fn test_struct_1() {
    let my = MyClass {
        name: String::from("nihao"),
        age: 12,
    };
    println!("test_struct_1: name= {}, age={}", my.name, my.age);
    println!("test_struct_1: name= {:?}", my);
    println!("test_struct_1: name= {:#?}", my);
}

// 测试元组
fn test_struct_2() {
    let my = (String::from("nihao"), 12);
    println!("test_struct_1: name= {}, age={}", my.0, my.1);
}

fn test_struct_3() {
    let my = MyClass {
        name: String::from("nihao"),
        age: 12,
    };

    let my2 = MyClass {
        name: String::from("haode"),
        ..my
    };
    println!("test_struct_3: name= {}, age={}", my2.name, my2.age);
}

#[derive(Debug)]
struct MyClass2(String, i32);

// 测试元组结构
fn test_struct_4() {
    let my = MyClass2(String::from("nihao"), 21);

    println!("test_struct_4: name= {}, age={}", my.0, my.1);
}

fn test_struct_5() {
    let my1 = MyClass {
        name: String::from("zhou"),
        age: 12,
    };
    let my2 = (String::from("zhou"), 12);
    let my3 = MyClass2(String::from("zhou"), 12);

    // test_struct_5: MyClass { name: "zhou", age: 12 }
    println!("test_struct_5: {:?}", my1);
    // test_struct_5: ("zhou", 12)
    println!("test_struct_5: {:?}", my2);
    // test_struct_5: MyClass2("zhou", 12)
    println!("test_struct_5: {:?}", my3);
}

fn test_struct_6() {
    let my1 = MyClass {
        name: String::from("zhou"),
        age: 12,
    };
    let my2 = (String::from("zhou"), 12);
    let my3 = MyClass2(String::from("zhou"), 12);

    /*
    test_struct_5: MyClass {
        name: "zhou",
        age: 12,
    }
    */
    println!("test_struct_5: {:#?}", my1);
    /*
    test_struct_5: (
        "zhou",
        12,
    )
    */
    println!("test_struct_5: {:#?}", my2);
    /*
    test_struct_5: MyClass2(
        "zhou",
        12,
    )
    */
    println!("test_struct_5: {:#?}", my3);
}

struct MyClass3 {
    name: String,
    age: i32,
    addrss: String,
}

impl MyClass3 {
    // 基本结构返回值，直接类型即可
    fn get_age(&self) -> i32 {
        self.age
    }

    // 结构类型返回值，需要借用
    fn get_name(&self) -> &String {
        &self.name
    }

    fn add_data(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn test_struct_7() {
    let a = MyClass3 {
        name: String::from("zhou"),
        age: 12,
        addrss: String::from("hangzhou"),
    };

    println!("test_struct_7: name={}, age={}", a.get_name(), a.get_age());
}

fn test_struct_8() {
    println!("test_struct_8: 1+3={}", MyClass3::add_data(1, 3));
}

// 单元结构体
struct Math;

impl Math {
    fn add_data(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn test_struct_9() {
    println!("test_struct_9: 1+3={}", Math::add_data(1, 3));
}

pub struct Neo {

}

impl Neo {
    pub fn new() -> Neo {
        Neo{}
    }

    pub fn fun(&self) -> u32 {
        1  + 3
    }
}

#[test]
fn test_struct_10() {
    let db = Neo::new();
    println!("=============");
    println!("====={}", db.fun());
}

#[test]
fn test_struct_11() {
    let t = getTest();

    println!("{:?}", t);
}

#[derive(Debug)]
pub struct Test<'a>{
    name: &'a str
}

fn getTest<'a>() -> &'a Test<'a> {
    &Test{name: "zhou"}
}
