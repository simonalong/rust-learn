
fn main() {
 
}

pub struct MyClass{
    pub name: String,
    age: i32
}

impl MyClass{
    fn test_1(&self) -> i32{
        return self.age
    }

    pub fn test_2(a: i32, b: i32) -> i32 {
        a + b
    }
}
