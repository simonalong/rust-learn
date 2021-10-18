use serde_json::Number;
use std::collections::HashMap;
use dashmap::mapref::one::{Ref, RefMut};
use dashmap::mapref::entry::Entry;
use dashmap::lock::{RwLockWriteGuard, RwLockReadGuard};
use dashmap::iter::{IterMut, Iter};
use serde::ser::SerializeMap;
use serde::Serialize;
use std::ops::Index;
use std::ops;

pub struct NeoMap {

}

impl NeoMap {
    pub fn new() -> Self {
        NeoMap{}
    }
}

impl Index<usize> for NeoMap {
    type Output = i32;

    fn index(&self, index: usize) -> &i32 {
        &12
    }
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
    let data = data_map[12];

    // 12
    println!("{}", data);
}


