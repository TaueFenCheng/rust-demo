use std::{
    collections::HashMap,
    fs::File,
    io::{self, ErrorKind, Read},
};
use rust_demo::Summary;
use rust_demo::Class;

enum Stu {
    Name(String),
    Id(f64),
    Class(String),
}

// trait

fn test_trait_fn(){
    let cls = Class {
        name:String::from("zhangsan"),
        stu_number: 66
    };
    println!("trait =================");
    println!("{}", cls.output_detail());
}


// 传播错误
// ? 运算符只能用于返回Result的函数
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn count_str() {
    let text = "hello world hello string world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1
    }
    println!("{:#?}", map);
}

fn match_file() {
    let f = File::open("hello.txt");
    match f {
        Ok(_file) => {
            println!("file found\n")
        }
        Err(err) => {
            println!("file open err {}\n", err)
        }
    };

    // unwrap方法 匹配match
    let un_f = File::open("world.txt").unwrap();
    println!("{:?}", un_f);

    // expect 可以指定错误信息
    let expect_f = File::open("tang.txt").expect("not found tang.txt文件");

    println!("{:?}", expect_f);
}

fn open_file_not_create() {
    let file = File::open("world.txt");
    match file {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("world.txt") {
                Ok(file) => file,
                Err(e) => {
                    panic!("error {}", e);
                }
            },
            _ => panic!("error opening file"),
        },
    };

    let fs = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|err| panic!("create file error {:?}", err))
        } else {
            panic!("not found file,{:?}", error)
        }
    });
}

fn main() {
    let v1: Vec<i32> = Vec::new();
    let mut v3 = Vec::new();
    v3.push(1);
    let v = vec![1, 2, 3];
    println!("{:?}", v);
    println!("{:?}", v1);
    println!("{:?}", v3);

    let var = &v[2];
    print!("{}\n", var);

    println!("Hello, world!");

    match v.get(3) {
        Some(var) => print!("{}", var),
        None => println!("none"),
    }

    let mut v4 = vec![3, 4, 5];

    for i in &mut v4 {
        // print!("{}\n",i)
        *i += 50 // 解引用
    }

    for i in &v4 {
        print!("{}\n", i)
    }

    let v5 = vec![
        Stu::Name(String::from("tang")),
        Stu::Id(4.44),
        Stu::Class(String::from("2-1")),
    ];

    //String

    let s = "hello world";
    let mut a = s.to_string();
    a.push_str("hi hao");
    a.push('a');
    let mut b = String::from("gegege");
    b.push_str("yes");
    print!("{}\n,{}\n", a, b);
    let join_str = format!("{}-{}", a, b);
    println!("{}\n", join_str);

    let mut map = HashMap::new();
    let key = String::from("Blue");
    map.insert(&key, 100);
    // let num = map.get(&key);
    // println!("{}",num);
    // match num {
    //     Some(s) => println!("{}\n", s),
    //     None => println!("nothing"),
    // }

    // for (k,v) in &mut map  {
    //     println!("{},{}",k,v)
    // }

    let key_yellow = String::from("Yellow");
    // 插入k v
    map.entry(&key_yellow).or_insert(180);

    // println!("{:?}",map);
    for (k, v) in &map {
        println!("{},{}", k, v)
    }
    count_str();
    // match_file();
    // open_file_not_create();
    let e = read_username_from_file();
    match e {
        Ok(s) => println!("{}", s),
        Err(e) => panic!("{:?}", e),
    }

    test_trait_fn();
}
