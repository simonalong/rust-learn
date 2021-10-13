// use std::{io::Result, str::FromStr};
// use serde::{Serialize, Deserialize};
// use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, guard, post, web};
//
// #[get("/fun1")]
// async fn fun1() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }
//
// #[post("/fun2")]
// async fn fun2(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }
//
// #[get("/fun3")]
// async fn fun3() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }
//
// #[post("/fun4")]
// async fn fun4(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }
//
// // #[post("/echo2")]
// // async fn echo2(req_body: String) -> impl Responder {
// //     HttpResponse::Ok().body(req_body)
// // }
//
// // async fn manual_hello() -> impl Responder {
// //     HttpResponse::Ok().body("Hey there!")
// // }
//
// fn app_config(cfg: &mut web::ServiceConfig) {
//     cfg.service(fun1).service(fun2);
// }
//
// fn item_config(cfg: &mut web::ServiceConfig) {
//     cfg.service(fun1).service(fun2);
// }
//
// #[get("/get1/{name}")]
// async fn get_1(web::Path(name): web::Path<String>) -> String {
//     String::from("data") + &name
// }
//
// #[get("/get2/{id}")]
// async fn get_2(web::Path(id): web::Path<u32>) -> String {
//     String::from("data") + &id.to_string()
// }
//
// #[get("/get3/{id}/{name}")]
// async fn get_3(info: web::Path<Info>) -> String {
//     String::from("data") + &info.name + &info.id.to_string()
// }
//
// #[get("/get4/{id}/{name}")]
// async fn get_4(req: HttpRequest) -> String {
//     let id = req.match_info().get("id").unwrap();
//     let name = req.match_info().get("name").unwrap();
//
//     String::from("data ") + &name + &id.to_string()
// }
//
// #[get("/get5")]
// async fn get_5(info: web::Query<Info>) -> String {
//     String::from("data5 ") + &info.name + &info.id.to_string()
// }
//
// #[post("/post1")]
// async fn post_1(info: web::Json<Info>) -> String {
//     String::from("data6 ") + &info.name + &info.id.to_string()
// }
//
// #[post("/post2")]
// async fn post_2(info: web::Form<Info>) -> String {
//     String::from("data6 ") + &info.name + &info.id.to_string()
// }
//
// #[get("/get_res")]
// async fn get_res() -> HttpResponse {
//     HttpResponse::Ok().body("data")
// }
//
// #[get("/get_str")]
// async fn get_str() -> String {
//     String::from("data")
// }
//
// #[get("/get_res_str")]
// async fn get_res_str() -> Result<String> {
//     Ok(String::from("data"))
// }
//
// #[get("/get_res_res")]
// async fn get_res_res() -> Result<HttpResponse> {
//     Ok(HttpResponse::Ok().body("data"))
// }
//
// #[get("/get_res_res_json")]
// async fn get_res_res_json() -> Result<HttpResponse> {
//     Ok(HttpResponse::Ok().json("data"))
// }
//
// #[derive(Deserialize)]
// struct Info{
//     id: u32,
//     name: String
// }
//
// #[get("/obj/get_res")]
// async fn get_obj_res1() -> HttpResponse {
//     HttpResponse::Ok().body(serde_json::to_string(&MyObj {
//         name: String::from("data"),
//     }).unwrap())
// }
//
// #[get("/obj/get_res_str")]
// async fn get_obj_res_str() -> Result<String> {
//     Ok(serde_json::to_string(&MyObj {
//         name: String::from("data"),
//     }).unwrap())
// }
//
// #[get("/obj/get_res_res")]
// async fn get_obj_res_res() -> Result<HttpResponse> {
//     let data = MyObj {
//         name: String::from("data"),
//     };
//     Ok(HttpResponse::Ok().body(serde_json::to_string(&data).unwrap()))
// }
//
// #[get("/obj/get_res_res_json")]
// async fn get_obj_res_res_json() -> Result<HttpResponse> {
//     Ok(HttpResponse::Ok().json(MyObj {
//         name: String::from("data"),
//     }))
// }
//
// #[derive(Serialize, Deserialize)]
// struct MyObj {
//     name: String,
// }
//
// fn url_config(cfg: &mut web::ServiceConfig) {
//     cfg
//     .service(get_1)
//     .service(get_2)
//     .service(get_3)
//     .service(get_4)
//     .service(get_5)
//     .service(post_1)
//     .service(post_2)
//     .service(get_res)
//     .service(get_str)
//     .service(get_res_str)
//     .service(get_res_res)
//     .service(get_res_res_json)
//     .service(get_obj_res1)
//     .service(get_obj_res_str)
//     .service(get_obj_res_res)
//     .service(get_obj_res_res_json)
//     ;
// }
//
// #[actix_web::main]
// async fn main() -> Result<()> {
//     let app = || {
//         App::new()
//             .service(web::scope("/api/core/config/app").configure(app_config))
//             .service(web::scope("/api/core/config/item").configure(item_config))
//             .service(web::scope("/test").configure(url_config))
//     };
//
//     HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
// }



#[macro_use]
extern crate rbatis;

use rbatis::rbatis::Rbatis;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};

use rbatis::crud::{CRUDTable, CRUD};
use std::error::Error;
use rbatis::core::value::DateTimeNow;

#[derive(CRUDTable, Serialize, Deserialize, Clone, Debug)]
//表名称 NeoTable1=> "neo_table1"
pub struct NeoTable1 {
    pub id: Option<String>,
    pub name: Option<String>,
    pub group: Option<String>,
    pub user_name: Option<String>,
    pub age: Option<i32>,
    pub sort: Option<i32>,
}

#[tokio::main]
async fn main() {
    /// enable log crate to show sql logs
    fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
    /// initialize rbatis. May use `lazy_static` crate to define rbatis as a global variable because rbatis is thread safe
    let rb = Rbatis::new();
    /// connect to database
    rb.link("mysql://neo_test:neo@Test123@127.0.0.1:3306/neo").await.unwrap();


    let activity = NeoTable1 {
        id: Some("12312".to_string()),
        name: None,
        group: None,
        user_name: Some(String::from("haode")),
        age: Some(1),
        sort: Some(1),
    };

    ///保存
    rb.save(&activity,&[]).await;
//Exec ==> INSERT INTO biz_activity (create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version) VALUES ( ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? )

    ///批量保存
    rb.save_batch(&vec![activity],&[]).await;
//Exec ==> INSERT INTO biz_activity (create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version) VALUES ( ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? ),( ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? )

    ///查询, Option包装，有可能查不到数据则为None
    let result: Option<NeoTable1> = rb.fetch_by_column("id", &"1".to_string()).await.unwrap();
//Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1  AND id =  ?
//
//   ///查询-全部
//   let result: Vec<NeoTable1> = rb.fetch_list("").await.unwrap();
// //Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1
//
//   ///批量-查询id
//   let result: Vec<NeoTable1> = rb.fetch_list_by_column("id",&["1".to_string()]).await.unwrap();
//Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1  AND id IN  (?)

//   ///自定义查询(使用wrapper)
//   let w = rb.new_wrapper().eq("id", "1");
//   let r: Result<Option<NeoTable1>, Error> = rb.fetch_by_wrapper( &w).await;
// //Query ==> SELECT  create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1  AND id =  ?
//
//   ///删除
//   rb.remove_by_column::<NeoTable1,_>("id", &"1".to_string()).await;
// //Exec ==> UPDATE biz_activity SET delete_flag = 0 WHERE id = 1
//
//   ///批量删除
//   rb.remove_batch_by_column::<NeoTable1, _>("id", &["1".to_string(), "2".to_string()]).await;
// //Exec ==> UPDATE biz_activity SET delete_flag = 0 WHERE id IN (  ?  ,  ?  )
//
//   ///修改(使用wrapper)
//   let w = rb.new_wrapper().eq("id", "12312");
//   rb.update_by_wrapper( &activity, &w).await;
// //Exec ==> UPDATE biz_activity SET  create_time =  ? , delete_flag =  ? , status =  ? , version =  ?  WHERE id =  ?
}
