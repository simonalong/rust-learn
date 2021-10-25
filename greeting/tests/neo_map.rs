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
use std::any::Any;


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

/// 提供neo_map["key"]的能力
// impl Index<&str> for NeoMap<'_> {
//     type Output = Value;
//
//     fn index(&self, index: &str) -> &Value {
//         &self.get(index)
//     }
// // }
// #[test]
// fn test_struct_11() {
//     let t = getTest();
//
//     println!("{:?}", t);
// }
//
// #[derive(Debug)]
// pub struct Test<'a> {
//     name: &'a str
// }
//
// fn getTest<'a>() -> &'a Test<'a> {
//     &Test { name: "zhou" }
// }
//
//
// // pub fn get_data_map(&self) -> &DashMap<String, Value> {
// //     &self.data_map
// // }
// //
// // pub fn put(&self, key: &str, value: &Value) {
// //     self.data_map.insert(String::from(key), value.clone());
// // }
// // neo_map.put("key", &Value::from("v"));
// // let map = neo_map.get_data_map();
// // println!("{:?}", map);
// // let value = &*map.get("key").unwrap();
//
// // #[rustc_diagnostic_item = "From"]
// // #[stable(feature = "rust1", since = "1.0.0")]
// // #[rustc_on_unimplemented(on(
// // all(_Self = "&str", T = "std::string::String"),
// // note = "to coerce a `{T}` into a `{Self}`, use `&*` as a prefix",
// // ))]
// // pub trait Putter<T>: Sized {
// //     /// Performs the conversion.
// //     #[lang = "from"]
// //     #[must_use]
// //     #[stable(feature = "rust1", since = "1.0.0")]
// //     fn put(&self, key: &str, _: T) -> &Self;
// // }
//
// pub trait Put<T> {
//     fn put(&self, key: &str, value: T) -> &Self;
// }
//
// struct InnerValue<T>
//     where T: Any + 'static
// {
//     original_value: T,
//     value: Value
// }
//
// pub struct NeoMap {
//     data_map: DashMap<String, InnerValue<dyn Any>>,
// }
//
// impl NeoMap {
//     pub fn new() -> Self {
//         NeoMap { data_map: DashMap::new() }
//     }
//     //
//     // pub fn put(&self, key: &str, value: &Value) {
//     //     self.data_map.insert(String::from(key), value.clone());
//     // }
//
//     pub fn get_i64(&self, key: &str) -> Option<i64> {
//         if let Some(v) = self.data_map.get(key) {
//             let value = self.data_map.get(key).unwrap();
//             let l = value.value();
//             l.value.as_i64()
//         } else {
//             None
//         }
//     }
//     //
//     // pub fn get_original(&self, key: &str) -> Option<Any> {
//     //     if let Some(v) = self.data_map.get(key) {
//     //         let value = self.data_map.get(key).unwrap();
//     //         let l = &value.value();
//     //         Some(*l.original_value)
//     //     } else {
//     //         None
//     //     }
//     // }
// }
//
// impl Put<&str> for NeoMap {
//
//     fn put(&self, key: &str, value: &str) -> &NeoMap {
//         self.data_map.insert(String::from(key), InnerValue{original_value: value, value: Value::from(value)});
//         self
//     }
// }
//
// impl Put<i64> for NeoMap {
//
//     fn put(&self, key: &str, value: i64) -> &NeoMap {
//         self.data_map.insert(String::from(key), InnerValue{original_value: value, value: Value::from(value)});
//         self
//     }
// }
// //
// // impl Putter<String> for NeoMap {
// //     /// Converts a `&str` into a [`String`].
// //     ///
// //     /// The result is allocated on the heap.
// //     #[inline]
// //     fn put(&self, key: &str, value: String) -> &NeoMap {
// //         self.data_map.insert(String::from(key), Value::from(value));
// //         self
// //     }
// // }


#[test]
pub fn test1() {
    // let neo_map = NeoMap::new();
    // neo_map.put("key", 12);
    // println!("{:?}", neo_map.get_i64("key"));
    // println!("{:?}", neo_map.get_original("key"));
    // println!("data");

    let x = 12;
    let s = match x {
        10 => 10 * 100,
        11 => 11 * 100,
        12 => 12 * 100,
        13 => 13 * 100,
        _ => 0
    };

    println!("{}", s);
}


