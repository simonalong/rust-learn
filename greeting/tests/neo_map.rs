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
use std::iter::Map;
use serde_json::json;

pub struct NeoMap {

    data_map: DashMap<String, Value>,
    name: Value
}

// pub enum Value{
//     Null,
//
//     Bool(bool),
//     Number(Number),
//
//     String(String),
//
//     Array(Vec<Value>),
//
//     Object(Map<String, Value>),
//
//     NeoMap(NeoMap),
// }

impl NeoMap {
    pub fn new() -> Self {
        NeoMap{name:Value::String(String::from("sdf")), data_map: DashMap::new()}
    }
    //
    // pub fn of(str: &[&str]) -> Self {
    //     NeoMap{name:String::from("adf"), data_map: DashMap::new()}
    // }
}

impl Index<&str> for NeoMap {
    type Output = Value;

    fn index(&self, index: &str) -> &Self::Output {
        &self.name
    }
    //
    // fn index(&self, index: usize) -> &Self::Output {
    //     self.0.index(index)
    // }
}
//
// pub trait Map<K, V> {
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

#[test]
pub fn test1() {
    let data_map = NeoMap::new();
    let data = &data_map["k"];

    // 12
    println!("{}", data);
}


