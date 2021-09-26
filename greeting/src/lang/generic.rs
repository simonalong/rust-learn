
///
/// 泛型
/// 
fn main() {
    test_generic_1();
    test_generic_2();
    test_generic_3();
    test_generic_4();
    test_generic_5();
}

// 测试泛型
fn test_generic_1() {
    println!("max value: {}", max(12, 32));
}

fn test_generic_2() {
    let t = GenericType{name: "zhou", age: 12};
    println!("test_generic_2 value {}", t.get());
}

fn test_generic_3() {
    let t = GenericType1{name: "zhou", age: 12};
    println!("test_generic_3 value {}", t.get());
}

fn test_generic_4() {
    let t = GenericType2{name: "zhou", age: 12};
    println!("test_generic_4 value {}", t.get());
}

fn test_generic_5() {
    let t = GenericType3{name: String::from("chou"), age: 12};
    let t1 = &GenericType3{name: String::from("chou"), age: 12};
    println!("test_generic_5 value {}", t.get(t1));
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        return a
    } else {
        return b
    }
}

struct GenericType<T, M> {
    name: T,
    age: M
}

impl<T, M> GenericType<T, M> {
    
    fn get(&self) -> &T {
        return &self.name
    }
}

struct GenericType1<T> {
    name: T,
    age: i32
}

impl<T> GenericType1<T> {
    
    fn get(&self) -> &T {
        return &self.name
    }
}

struct GenericType2<T> {
    name: T,
    age: i32
}

// impl即使已经明确了类型，但是还是要明确指明
impl<String> GenericType2<String> {
    
    fn get(&self) -> &String {
        return &self.name
    }
}

struct GenericType3<T> {
    name: T,
    age: i32
}

impl<String> GenericType3<String> {
    
    fn get<'a, T>(&self, type3: &'a GenericType3<T>) -> &'a T {
        return &type3.name
    }
}