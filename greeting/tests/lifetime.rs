
///
/// 生命周期
/// 
fn main() {
    test_lifetime_1();
}

fn test_lifetime_1() {
    let t = GenericType3{name: String::from("chou"), age: 12};
    let t1 = &GenericType3{name: String::from("chou"), age: 12};
    println!("test_lifetime_1 value {}", t.get(t1));
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