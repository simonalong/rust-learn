use serde_json::{Value, to_string};
use serde::{Deserialize, Serialize};
use dashmap::DashMap;

#[test]
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

// #[test]
// fn test_2() {
//     let data_map: DashMap<&str, &str> = DashMap::new();
//     data_map.insert("k1", "v1");
//     data_map.insert("k2", "v2");
//
//     println!("{}", serde_json::to_string(&data_map).unwrap());
//
//     // let mut neo_map = NeoMap::new();
//     // neo_map.put("k1", "v1");
//     //
//     // println!("{}", neo_map.to_json_string());
// }

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct NeoMap {

    data_map: String
}

impl NeoMap {
    pub fn new() -> NeoMap {
        NeoMap{data_map: String::from("test")}
    }
}

#[test]
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

// #[test]
// fn test_3() {
//     let neo_map = NeoMap{data_map: String::from("haode")};
//
//     println!("{}", serde_json::to_string(&neo_map).unwrap());
// }


