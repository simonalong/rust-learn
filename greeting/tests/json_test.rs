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

#[test]
fn test_2() {
    let data_map: DashMap<&str, &str> = DashMap::new();
    data_map.insert("k1", "v1");
    data_map.insert("k2", "v2");

    println!("{}", serde_json::to_string(&data_map).unwrap());

    // let mut neo_map = NeoMap::new();
    // neo_map.put("k1", "v1");
    //
    // println!("{}", neo_map.to_json_string());
}

//
// #[derive(Deserialize, Debug, Clone, Default)]
// struct NeoMap {
//
//     data_map: DashMap<String, String>
// }
//
// impl NeoMap {
//     pub fn new() -> NeoMap {
//         NeoMap{data_map: DashMap::new()}
//     }
//
//     pub fn put(&mut self, key:&str, value: &str) -> &mut NeoMap {
//         let mut map = &mut self.data_map;
//         map.insert(String::from(key), String::from(value));
//         self
//     }
//
//     pub fn to_json_string(&self) -> String {
//         to_string(&self.data_map).unwrap()
//     }
// }
//


