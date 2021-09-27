use std::collections::HashMap;


fn main() {
   test_map1();
   test_map2();
}

fn test_map1() {
    let mut data_map = HashMap::new();

    data_map.insert("a", 1);
    data_map.insert("b", 2);

    println!("{:?}", data_map);

    println!("{:?}", data_map.get("b").unwrap());

    println!("{:?}", data_map.contains_key("b"));

    // 循环迭代
    for ele in data_map {
        println!("key = {}, value= {}", ele.0, ele.1);
    }
}

// 在当前的值不存在情况下进行插入
fn test_map2() {

    let mut data_map = HashMap::new();

    data_map.insert("a", 1);
    data_map.entry("b").or_insert(3);

    for ele in &data_map {
        println!("{:?}", ele);
    }

    data_map.insert("a", 10);

    println!("{:?}", data_map);
}
