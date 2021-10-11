use serde::{Deserialize, Serialize};
use serde_json::json;

fn main() {
    // test_json();
    test_json1();
}

fn test_json() {
    // let data = MyData {
    //     name: String::from("data"),
    //     id: 321,
    // };
    // let data_json = serde_json::to_string(&data).unwrap();
    // println!("data ===== {}", data_json);
}

// #[derive(Serialize, Deserialize)]
// struct MyData{
//     name: String,
//     id: i128
// }

fn test_json1() {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let v: Value = serde_json::from_str(data)?;

    println!("Please call {} at the number {}", v["name"], v["phones"][0]);
}
