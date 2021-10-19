use serde_json::{Number, Value};
use std::collections::HashMap;
use dashmap::mapref::one::{Ref, RefMut};
use dashmap::mapref::entry::Entry;
use dashmap::lock::{RwLockWriteGuard, RwLockReadGuard};
use dashmap::iter::{IterMut, Iter};
use serde::ser::SerializeMap;
use serde::Serialize;
use std::ops::Index;
use std::ops;
use dashmap::DashMap;
use serde_json::json;


// pub trait AbstractMap<String, Value> {
//     fn put(key:K, v:v);
//     fn del(key:K);
//
//     fn get_str(key:K);
//     fn get_string(key:K);
//
//     fn get_i8(key:K);
//     fn get_i16(key:K);
//     fn get_i32(key:K);
//     fn get_i64(key:K);
//     fn get_i128(key:K);
//
//     fn get_u8(key:K);
//     fn get_u16(key:K);
//     fn get_u32(key:K);
//     fn get_u64(key:K);
//     fn get_u128(key:K);
//
//     fn get_bool(key:K);
//
//     fn get_vec(key:K);
//
//     fn get_neo_map(key:K);
// }


pub struct NeoMap<'a> {
    data_map: &'a DashMap<String, Value>,
}

/// 提供neo_map["key"]的能力
impl Index<&str> for NeoMap<'_> {
    type Output = Value;

    fn index(&self, index: &str) -> &Value {
        &self.get(index)
    }
}

impl NeoMap<'_> {
    pub fn new<'a>() -> &'a NeoMap<'a> {
        &NeoMap{ data_map: &DashMap::new() }
    }

    pub fn get<'a>(&'a self, key: &str) -> &'a Value {
        // &((*self).data_map.get(key).unwrap())
        self.data_map.get(key).unwrap().value()
    }

    // pub fn of(kv_str: &[&str]) -> Self {
    //     if kv_str.len() % 2 != 0 {
    //         panic!("参数请使用：key,value,key,value...这种参数格式")
    //     }
    //
    //     let neo_map = NeoMap::new();
    //     let mut i = 0;
    //     while i < kv_str.len() {
    //         neo_map.put(kv_str[i], &Value::from(kv_str[i + 1]).clone());
    //         i += 2
    //     }
    //
    //     neo_map
    // }
}

impl NeoMap<'_> {

    pub fn put(&self, key: &str, n: &Value) {

    }


}

#[test]
pub fn test1() {
    let neo_map = NeoMap::new();
    // neo_map.put("k", &Value::from("v"));
    // let data = &data_map["k"];

    println!("{}", neo_map.get("k"));

    // let data_map = DashMap::new();
    // data_map.insert("k", Value::from("v"));
    // // data_map.insert("k", "v");
    //
    // println!("{}", *data_map.get("k").unwrap());
}


