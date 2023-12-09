extern crate movies_lib;
extern crate Avariable;
extern crate http_server;
#[macro_use]
extern crate rocket;

use rocket::tokio::time::{sleep, Duration};
use rocket::tokio::task::spawn_blocking;
use rocket::serde::{Deserialize, json::Json};
use rocket::fs::TempFile;
use rocket::form::{self, Error};

use std::io;

use movies_lib::movies::play;
use Avariable::{get_address, get_name, user_info, user_name};
use Avariable::book::book::{book_info, book_numb};


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")]              // <- route attribute
fn world() -> &'static str {  // <- request handler
    "hello, world!"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

/**
将同步请求转为异步
 */
#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
    let vec = spawn_blocking(|| std::fs::read("http_server/docker-compose.yml")).await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;
    Ok(vec)
}

//用户信息结构体
#[derive(Deserialize)]
//由于 Rust 对派生宏再导出的支持有限，使用再导出的派生宏需要在结构上注释 #[serde(crate = "rocket::serde")]
//避免这种额外的注解，就必须通过你的 crate 的 Cargo.toml 直接依赖 serde
//serde = { version = "1.0", features = ["derive"] }
//先不依赖了 就这么滴
#[serde(crate = "rocket::serde")]
//字段验证
#[derive(Debug)]
struct User<'r> {
    id: i32,
    name: String,
    age: i8,
    description: &'r str,
}


//接收json数据格式
//Json<T> guard 会将正文数据反序列化为 JSON 格式。唯一的条件是泛型 T 实现了 serde 的 Deserialize 特性。
#[post("/new_user", format = "application/json", data = "<user>")]
fn new_user<'v>(user: Json<User<'_>>){
    println!("获取到的用户信息{:#?}", user);
}

// 上传文件
//#[post("/upload", format = "plain", data = "<file>")]
//{ code: 17, kind: CrossesDevices, message: "系统无法将文件移到不同的磁盘驱动器。" }
#[post("/upload", format = "multipart/form-data", data = "<file>")]
async fn upload(mut file: TempFile<'_>) -> std::io::Result<()> {
    file.persist_to("/rust-learning-records/").await
}

//启动方式一
// #[launch]
// fn rocket() -> _ {
//     rocket::build()
//         .mount("/", routes![index])
//         .mount("/hello", routes![world])
// }

//启动方式二
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index,blocking_task,new_user])
        .mount("/hello", routes![world])
        .mount("/", routes![delay,upload])
        .launch()
        .await?;

    Ok(())
}

// fn main() {
//     //声明变量并初始化赋值
//     let name = "bbzywkq.com";
//     //Rust会对未使用的变量发出警告信息。如果确实想保留从未被使用过的变量，可在变量名前加上_前缀。特别提示变量的声明需要指明类型
//     let _age: &i32;
//
//     //变量的赋值
//     let _name1 = "wangkeqiang";
//     //重新赋值  重复声明同名变量，后声明的变量将遮盖(shadow)前面已声明的变量
//     let name1 = "bbzyalone";
//     //不能如下赋值 变量初始化后，默认不允许再修改该变
//     //name1="xxxxx"
//
//
//     let mut book = "太上忘情路";
//     book = "九阴白骨爪";
//
//
//     // play("简单教程".to_string());
//     // user_info();
//     // user_name();
//     // get_address();
//     // get_name();
//     // book_info();
//     // book_numb();
//     // get_name();
//     // start();
//
// }






