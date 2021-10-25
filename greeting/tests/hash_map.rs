use std::collections::HashMap;
use serde_json::Value;

// #[test]
// fn test_2() {
//     // let mut item = HashMap::new();
//     // let mut map2 = HashMap::new();
//     // map2.insert("kk", "fs");
//     //
//     // let mut ve1:Vec<&str> = Vec::new();
//     // ve1.push("test");
//     //
//     // item.insert("key1", "sdf"); //type is &str
//     // item.insert("key2", ve1); //type is Vec<String>
//     // item.insert("key3", map2); //Type is HashMap<&str, u32>
//     //
//     // let response = json::encode(&item).unwrap();
//
//     let mut inner_map = Map::new();
//     // inner_map.insert("x".to_string(), Value::Number(Number::from()));
//     // inner_map.insert("y".to_string(), Value::Number(Number::from(20)));
//
//     let mut map = Map::new();
//     map.insert("key1".to_string(), Value::String("test".to_string()));
//     map.insert(
//         "key2".to_string(),
//         Value::Array(vec![
//             Value::String("a".to_string()),
//             Value::String("b".to_string()),
//         ]),
//     );
//     map.insert("key3".to_string(), Value::Object(inner_map));
//
//     println!("{}", serde_json::to_string(&map).unwrap());
// }


#[test]
fn test_map1() {
    // let mut data_map = HashMap::new();
    //
    // data_map.insert("a", 1);
    // data_map.insert("b", 2);
    //
    // let data = data_map.iter();
    // for (k, v) in data_map {
    //
    // }
    //
    // // let keys = &data_map.keys();
    // println!("keys {:?}", data_map.keys());
    //
    // println!("{:?}", data_map);
    //
    // println!("{:?}", data_map.get("b").unwrap());
    //
    // println!("{:?}", data_map["a"]);
    //
    // println!("{:?}", data_map.contains_key("b"));
    //
    // // 循环迭代
    // for ele in data_map {
    //     println!("key = {}, value= {}", ele.0, ele.1);
    // }
}

#[test]
fn test_map2() {
    let mut data_map = HashMap::new();

    data_map.insert("a", 1);
    data_map.insert("b", 2);

    data_map.keys();
}

// #[test]
// fn test4() {
//     let m = tt();
//
//     println!("{}", (*m).0);
// }

// fn tt() -> &(i32, Vec<String>) {
//     let mut array = Vec::new();
//     array.push(String::from("a"));
//     array.push(String::from("b"));
//     &(12, array)
// }

