use flurry::HashMap;
use chashmap::CHashMap;

#[test]
pub fn test1() {
    let map = HashMap::new();

    let guard = map.pin();
    guard.insert("k1", "v1");
    guard.insert("k2", "v2");

    println!("{:?}", guard);

    let value = guard.get("k1").unwrap();
    println!("{:?}", value);

    let data = CHashMap::new();
    data.insert("key1", "v2");

    println!("{:?}", data);
}
