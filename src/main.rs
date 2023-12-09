use chrono::prelude::*;
use mysql::prelude::*;
use mysql::*;

extern crate movies_lib;
extern crate Avariable;

use movies_lib::movies::play;
use Avariable::{get_address, get_name,user_info,user_name};
use Avariable::book::book::{book_info, book_numb};

fn main() {
    println!("Hello, world!");
    //声明变量并初始化赋值
    let name = "bbzywkq.com";
    println!("{}", name);
    //Rust会对未使用的变量发出警告信息。如果确实想保留从未被使用过的变量，可在变量名前加上_前缀。特别提示变量的声明需要指明类型
    let _age: &i32;

    //变量的赋值
    let _name1 = "wangkeqiang";
    //重新赋值  重复声明同名变量，后声明的变量将遮盖(shadow)前面已声明的变量
    let name1 = "bbzyalone";
    //不能如下赋值 变量初始化后，默认不允许再修改该变
    //name1="xxxxx"

    println!("{}", name1);

    let mut book = "太上忘情路";
    book = "九阴白骨爪";
    println!("{}", book);


    play("简单教程".to_string());
    user_info();
    user_name();
    get_address();
    get_name();
    book_info();
    book_numb();
}

struct UrlMap {
    id: i32,
    surl: String,
    lurl: String,
    views: i32,
    createTime: NaiveDate,
}