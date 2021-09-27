use std::{fs::OpenOptions, io::{Read, Write}};


///
/// io
/// 
fn main() {
    test_io_1();
    // test_io_2();
    test_io_file_3();
    test_io_file_4();
    test_io_file_5();
    test_io_file_write_1();
    test_io_file_write_2();
    test_file_write_3();
}

fn test_io_1() {
    let args = std::env::args();
    for arg in args {
        println!("{}", arg);
    }
}

// 读取终端
fn test_io_2() {
    let mut data = String::new();
    std::io::stdin().read_line(&mut data).expect("read fail");

    println!("data = {}", data);
}

// 读取文件
fn test_io_file_3() {

    let data = std::fs::read_to_string("/Users/zhouzhenyong/tem/src1.txt").unwrap();
    println!("{}", data);
}

// 字节方式读取
fn test_io_file_4() {

    // 读取到的是 u8 类型的数据
    let data = std::fs::read("/Users/zhouzhenyong/tem/src1.txt").unwrap();
    // [104, 97, 111, 100, 101, 10]
    println!("{:?}", data);
}

// 分段读取
fn test_io_file_5() {

    let mut content = [0u8;5];
    let mut file = std::fs::File::open("/Users/zhouzhenyong/tem/src1.txt").unwrap();
    file.read(&mut content).unwrap();
    // [104, 97, 111, 100, 101]
    println!("{:?}", content);

    // [10, 0, 0, 0, 0]
    content = [0u8;5];
    file.read(&mut content).unwrap();
    println!("{:?}", content);
}

// 一次性写入文件
fn test_io_file_write_1() {
    std::fs::write("/Users/zhouzhenyong/tem/dst.txt", "neirong").unwrap();

    let content = std::fs::read_to_string("/Users/zhouzhenyong/tem/dst.txt").unwrap();
    println!("{}", content);
}

// 流的处理方式
fn test_io_file_write_2() {
    let mut file = std::fs::File::create("/Users/zhouzhenyong/tem/dst.txt").unwrap();
    file.write(b"wenjian").unwrap();

    let content = std::fs::read_to_string("/Users/zhouzhenyong/tem/dst.txt").unwrap();
    println!("{}", content);

    let ff = b"asdfadf";
        println!("{:?}", ff);
}

fn test_file_write_3() {
    let mut file = OpenOptions::new().read(true).write(true).open("/Users/zhouzhenyong/tem/dst.txt").unwrap();
    file.write(b" append");

    file.write(b" append2 ");

    let content = std::fs::read_to_string("/Users/zhouzhenyong/tem/dst.txt").unwrap();
    println!("{}", content);
}