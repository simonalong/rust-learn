use dashmap::{DashMap};
use std::collections::HashMap;

#[test]
fn test1() {
    // let mut map:DashMap<&str,&str> = DashMap::new();
    // map.insert("key1", "v1");
    // map.insert("key2", "v2");
    // println!("{}", serde_json::to_string(&map).unwrap());

    let mut map = HashMap::new();
    map.insert("key1", "v1");
    map.insert("key2", "v2");
    println!("{}", serde_json::to_string(&map).unwrap());
}

#[test]
pub fn test2() {
    let map = DashMap::new();
    map.insert("key1", 12);
    map.insert("key2", 32);
    map.insert("key3", 21);

    map.retain(|_,v|(*v > 20));

    // {"key2": 32, "key3": 21}
    println!("{:?}", map);
}

#[test]
pub fn test3() {
    let map = DashMap::new();
    map.insert("key1", 12);
    map.insert("key2", 32);
    map.insert("key3", 21);

    // 3
    println!("{:?}", map.len());
}

#[test]
pub fn test4() {
    let map = DashMap::new();
    map.insert("key1", 12);
    map.insert("key2", 32);

    // 修改一个值
    map.alter("key1", |_, v|(v * 100));
    // {"key2": 32, "key1": 1200}
    println!("{:?}", map);

    // 修改所有值
    map.alter_all(|_, v|(v * 100));

    // {"key2": 3200, "key1": 120000}
    println!("{:?}", map);
}

#[test]
pub fn test5() {
    let map = DashMap::new();
    map.insert("key1", 12);
    map.insert("key2", 32);
    map.insert("key3", 21);

    // 3
    println!("{}", *map.get("key1").unwrap());
}

#[test]
pub fn test6() {
    let youtubers = DashMap::new();
    youtubers.insert("Bosnian Bill", 457000);
    assert_eq!(*youtubers.get("Bosnian Bill").unwrap(), 457000);
}
