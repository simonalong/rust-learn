use std::collections::HashMap;
use serde_json::Value;

#[test]
fn test_2() {
    // let mut item = HashMap::new();
    // let mut map2 = HashMap::new();
    // map2.insert("kk", "fs");
    //
    // let mut ve1:Vec<&str> = Vec::new();
    // ve1.push("test");
    //
    // item.insert("key1", "sdf"); //type is &str
    // item.insert("key2", ve1); //type is Vec<String>
    // item.insert("key3", map2); //Type is HashMap<&str, u32>
    //
    // let response = json::encode(&item).unwrap();

    let mut inner_map = Map::new();
    // inner_map.insert("x".to_string(), Value::Number(Number::from()));
    // inner_map.insert("y".to_string(), Value::Number(Number::from(20)));

    let mut map = Map::new();
    map.insert("key1".to_string(), Value::String("test".to_string()));
    map.insert(
        "key2".to_string(),
        Value::Array(vec![
            Value::String("a".to_string()),
            Value::String("b".to_string()),
        ]),
    );
    map.insert("key3".to_string(), Value::Object(inner_map));

    println!("{}", serde_json::to_string(&map).unwrap());
}
