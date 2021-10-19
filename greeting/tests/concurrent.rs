use std::{sync::mpsc, time::Duration};


fn main() {
    test_thread_1();
    test_thread_2();
    test_thread_3();
    test_thread_4();
    test_thread_5();
    test_thread_6();
}

// 测试线程
fn test_thread_1() {
    std::thread::spawn(||{
        println!("sdfasd");
    });

    std::thread::sleep(Duration::from_secs(1));
}

// 测试线程
fn test_thread_2() {
    std::thread::spawn(test_run);

    std::thread::sleep(Duration::from_secs(1));
}

fn test_run() {
    println!("sdfasd");
}

fn test_thread_3() {
    let data = std::thread::spawn(||{
        println!("data");
        std::thread::sleep(Duration::from_secs(1));
    });

    data.join().unwrap();

    println!("end");
}

// 闭包的move关键字
fn test_thread_4() {
    let data = "str";
    std::thread::spawn(move||{
        println!("test_thread_4 {}", data);
    });

    println!("test_thread_4 end")
}

// 处理消息传递
fn test_thread_5() {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        tx.send("haode")
    });

    let data = rx.recv().unwrap();
    println!("test_thread_5 receive {}", data);
}

fn test_thread_6() {
    let data = || {
        println!("test_thread_6");
    };

    std::thread::spawn(data);
}

