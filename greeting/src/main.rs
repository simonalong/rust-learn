use serde_json::Value;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;

fn main() {
    test_json1();
    test_json2();
    test_json3();
}

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

    let value:Value = serde_json::from_str(data).unwrap();
    println!("{}", value);
    println!("Please call {} at the number {}", value["name"], value["phones"][0]);
}

fn test_json2() {
    let data = MyData {
        name: String::from("data"),
        id: 321,
    };
    let data_json = serde_json::to_string(&data).unwrap();
    println!("data ===== {}", data_json);
}

#[derive(Serialize, Deserialize)]
struct MyData{
    name: String,
    id: i128
}

fn test_json3() {
    // let data = r#"
    //     {
    //         "name": "John Doe",
    //         "age": 43,
    //         "phones": [
    //             "+44 1234567",
    //             "+44 2345678"
    //         ]
    //     }"#;
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    println!("first phone number: {}", john["phones"][0]);

    // Convert to a string of JSON and print it out
    println!("{}", john.to_string());
}
