

///
/// 集合
///
fn main() {
    test_vec_1();
    test_vec_2();
    test_vec_3();
    test_vec_4();
    test_vec_5();
    test_vec_6();
    test_vec_7();
}

// 集合创建后添加数据
fn test_vec_1() {
    let mut ve1 = Vec::new();
    ve1.push(12);
    ve1.push(132);

    println!("{:?}", ve1);
}

fn test_vec_2() {
    // 提供宏 vec!进行提供创建数组
    let ve1 = vec![12, 32, 3];

    println!("{:?}", ve1);
}

fn test_vec_3() {
    let mut v1 = vec![1, 2, 3];
    let mut v2 = vec![4, 5, 6];

    v1.append(&mut v2);

    println!("v1 {:?}", v1);
    println!("v2 {:?}", v2);
}

// 访问数组
fn test_vec_4() {
    let ve3 = vec![2, 32, 43, 5];

    if let Some(value) = ve3.get(2) {
        println!("{}", value);
    } else {
        println!("none");
    }
}

// 访问数组
fn test_vec_5() {
    let ve3 = vec![2, 32, 43, 5];

    for ele in ve3 {
        println!("{}", ele);
    }
}

// 访问数组
fn test_vec_6() {
    let ve3 = vec![2, 32, 43, 5];

    // 添加上借用后，下面的还能继续访问，否则下面的就不能访问了
    for ele in &ve3 {
        println!("{}", ele);
    }

    println!("====");
    for ele in &ve3 {
        println!("{}", ele);
    }
}

// 数组中进行循环更改
fn test_vec_7() {
    let mut v1 = [1, 3, 43, 5];

    for ele in &mut v1 {
        *ele += 100;
    }

    println!("{:?}", v1);
}

#[test]
pub fn test1() {
    let mut array = Vec::new();
    array.push(1);
    array.push(2);
    array.push(3);

    // let str = array.iter().map(|&s| s.into()).collect();
    // do_stuff_with_args(args);

    // println!("{}", str);

    // let args:String = ["-w", "60", "args"].iter().map(|s| s.into()).collect();
    // println!("{}", args);

    // let needle = "list".to_string();
    // let haystack = ["some".to_string(), "long".to_string(), "list".to_string(), "of".to_string(), "strings".to_string()].to_vec();
    //
    // if let Some(str) = haystack.iter().find(|&s| *s == needle) {
    //     println!("{}", needle);
    // } else {
    //     println!("Nothing there...");
    // }

    // let needle = "list".into();
    // let haystack = vec!["some", "long", "list", "of", "strings"]
    //     .into_iter()
    //     .map(String::from)
    //     .join(",");
    //
    // println!("{:?}", haystack);

    // let arr: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
    // let s:String = arr.iter().collect();
    //
    // println!("{}", s);


    // let v = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    // let mut r = array.iter().fold(String::new(), |r, c| r + c + ".");
    // let mut r = array.iter().fold(String::new(), |r, c| r + c + ".");
    // let mut r = array.iter().join(".");
    // r.pop();

    // let v = ["a", "b", "c"];
    // let connected = v.iter().rev().join(".");
    // println!("{}", connected);

    // let a = &["a", "b", "c"];
    // let s: String = a.iter().cloned().rev().interleave(".").collect();
    // println!("{}", s);
    //
    // let v = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    // let s: String = v.iter().map(|s| s.as_str()).rev().interleave(".").collect();
    // println!("{}", s);

    let mut v = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    v.reverse();
    println!("{}", v.join("."));
    // println!("{}", array.join("."));

    // let d:Vec<String, _> = array.iter().collect();
    // println!("{}", d.join("."));

    let mut v:Vec<i32> = array.into_iter().map(|s| s+1).collect();
    println!("{:?}", v);

    let mut array_str = Vec::new();
    array_str.push(String::from("a"));
    array_str.push(String::from("b"));
    array_str.push(String::from("c"));

    let arr:Vec<String> = array_str.into_iter().map(|e| e + ".").collect();
    let ds: String = arr.join(", ");
    println!("{}", ds);
}

pub fn to_db_field(column: String) -> String {
    if column.starts_with("`") || column.ends_with("`") {
        return column;
    }

    let mut s1 = "".to_string();
    s1 += "`";
    s1 += &column.as_str();
    s1 += "`";
    return String::from(s1);
}
