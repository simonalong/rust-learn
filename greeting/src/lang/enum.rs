///
///
/// 测试枚举类
///
fn main() {
    test_enum_1();
    test_enum_2();
    test_enum_3();
    test_enum_4();
    test_enum_5();
    match_test();
    test_enum_option();
    test_enum_option2();
    test_enum_option3();
    test_enum_option4();
    test_enum_option5();
}

#[derive(Debug)]
enum ChannelEnum {
    Open,
    Close,
}
fn test_enum_1() {
    let status1 = ChannelEnum::Open;
    println!("test_enum_1 {:?}", status1);

    let status2 = ChannelEnum::Close;
    println!("test_enum_1 {:?}", status2);
}

fn test_enum_2() {
    let status1 = ChannelEnum::Open;

    // 这里通过
    match status1 {
        ChannelEnum::Close => {
            println!("close");
        }
        ChannelEnum::Open => {
            println!("open");
        }
    }
}

enum Book {
    ZhiBook { name: String },
    ElecBook { url: String },
}

fn test_enum_3() {
    let book = Book::ZhiBook {
        name: String::from("java书"),
    };

    match book {
        Book::ZhiBook { name } => {
            println!("book {}", name);
        }

        Book::ElecBook { url } => {
            println!("book {}", url);
        }
    }
}

enum Book2 {
    ZhiBook(String),
    ElecBook(i32),
}

fn test_enum_4() {
    let book = Book2::ZhiBook(String::from("java书"));

    match book {
        Book2::ZhiBook(url) => {
            println!("book {}", url);
        }

        Book2::ElecBook(size) => {
            println!("book {}", size);
        }
    }
}

enum Book3 {
    ZhiBook(String),
    ElecBook(i32),
    ElecBook2(i32)
}

fn test_enum_5() {
    let book = Book3::ZhiBook(String::from("java书"));
    if let Book3::ElecBook(size) = book {
        println!("enum_5 1 {}", size);
    }  else if let Book3::ZhiBook(name) = book{
        println!("enum_5 1 {}", name);
    } else {
        println!("enum_5 1 ");
    }
}

fn match_test() {
    let data = 122;
    match data {
        12 => {
            println!("12")
        }
        _ => {
            println!("meiyoupipei")
        }
    }
}

// 测试option枚举
fn test_enum_option() {
    let data = Option::Some(String::from("nihao"));
    match data {
        Option::Some(data) => {
            println!("test_enum_option {}", data);
        }
        Option::None => {
            println!("kong");
        }
    }
}

fn test_enum_option2() {
    let data: Option<String> = None;
    match data {
        Some(data) => {
            println!("test_enum_option {}", data);
        }
        None => {
            println!("kong");
        }
    }
}

fn test_enum_option3() {
    let data: Option<String> = None;
    match data {
        Some(data) => {
            println!("test_enum_option {}", data);
        }
        _ => {
            println!("kong");
        }
    }
}

// 简化写法
fn test_enum_option4() {
    let data: Option<String> = None;
    if let Some(data) = data {
        println!("test_enum_option {}", data);
    } else {
        println!("kong");
    }
}

// 更加简化的写法，就是如果非空，则直接报错
fn test_enum_option5() {
    let data: Option<String> = Option::Some(String::from("nihao"));
    let name = data.unwrap();
    println!("{}", name)
}
