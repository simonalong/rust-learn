use dashmap::{DashMap};
use serde_json::{Value, to_string};
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeMap;
use std::collections::HashMap;
// use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};
// use dashmap::DashMap;
// use serde::ser::SerializeMap;

#[test]
fn test_json1() {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ],
            "k1":{
                "k2": 23,
                "k3": "ok",
                "k4": "haode"
            }
        }"#;

    let value:Value = serde_json::from_str(data).unwrap();
    let data = value.get("k1").unwrap();
    println!("{}", value["k1"]);
    println!("{}", value.get("k1").unwrap());
    println!("{}", value["k1"]["k2"]);
    println!("{}", value["k1"]["k2"]["k3"]);
    println!("{}", value["k1"]["k4"]);
    println!("Please call {} at the number {}", value["name"], value["phones"][0]);
}

#[test]
fn test_json2() {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43
        }"#;

    let value:Info = serde_json::from_str(data).unwrap();
    println!("{:?}", value);
}

#[test]
fn test_json3() {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43
        }"#;

    let value:Value = serde_json::from_str(data).unwrap();
    println!("{:?}", value);
    let value:Info = serde_json::from_value(value).unwrap();
    println!("{:?}", value);
}

#[test]
fn test_json4() {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43
        }"#;

    let value:Value = serde_json::from_str(data).unwrap();
    let map = value.as_object().unwrap();

    println!("{:?}", map);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    name: String,
    age: i32
}

#[test]
fn test_2() {
    let mut data_map = DashMap::new();
    // let mut  data_map= HashMap::new();
    data_map.insert("name", Value::from("zhou"));
    data_map.insert("age", Value::from(12));

    // let string = serde_json::to_value(data_map).unwrap();
    // let data: AsClass = serde_json::from_value(string).unwrap();

    let string = serde_json::to_string(&data_map).unwrap();
    let data: AsClass = serde_json::from_str(string.as_str()).unwrap();

    println!("{:?}", data);
}

#[test]
fn test_3() {
    let v = Value::from("va");
    let s = v.as_str().unwrap();
    println!("{}", s);
}

#[derive(Serialize, Deserialize, Debug)]
struct AsClass{
    name: String,
    age: i32
}

//
// impl Serialize for DashMap<String, String>
// {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut map: dyn SerializeMap = serializer.serialize_map(Some(self.len()))?;
//         for (k, v) in self {
//             map.serialize_entry(k, v)?;
//         }
//         map.end()
//     }
// }

//
// impl Serialize for DashMap<&str, &str>
// {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: Serializer,
//     {
//         let mut map: dyn SerializeMap = serializer.serialize_map(Some(self.len()))?;
//
//         for ref_multi in self.iter() {
//             map.serialize_entry(ref_multi.key(), ref_multi.value())?;
//         }
//
//         map.end()
//     }
// }

// let mut neo_map = NeoMap::new();
// neo_map.put("k1", "v1");
//
// println!("{}", neo_map.to_json_string());

// #[derive(Serialize, Deserialize, Debug, Clone, Default)]
// struct NeoMap {

//     data_map: String
// }

// impl NeoMap {
//     pub fn new() -> NeoMap {
//         NeoMap{data_map: String::from("test")}
//     }
// }

// #[test]
// fn test_json2() {
//     let data = MyData {
//         name: String::from("data"),
//         id: 321,
//     };
//     let data_json = serde_json::to_string(&data).unwrap();
//     println!("data ===== {}", data_json);
// }
//
// #[derive(Serialize, Deserialize)]
// struct MyData{
//     name: String,
//     id: i128
// }

// #[test]
// fn test_3() {
//     let neo_map = NeoMap{data_map: String::from("haode")};
//
//     println!("{}", serde_json::to_string(&neo_map).unwrap());
// }

