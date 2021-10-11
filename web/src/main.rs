use std::{io::Result, str::FromStr};
use serde::{Serialize, Deserialize};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, guard, post, web};

#[get("/fun1")]
async fn fun1() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/fun2")]
async fn fun2(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/fun3")]
async fn fun3() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/fun4")]
async fn fun4(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// #[post("/echo2")]
// async fn echo2(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

fn app_config(cfg: &mut web::ServiceConfig) {
    cfg.service(fun1).service(fun2);
}

fn item_config(cfg: &mut web::ServiceConfig) {
    cfg.service(fun1).service(fun2);
}

#[get("/get1/{name}")]
async fn get_1(web::Path(name): web::Path<String>) -> String {
    String::from("data") + &name
}

#[get("/get2/{id}")]
async fn get_2(web::Path(id): web::Path<u32>) -> String {
    String::from("data") + &id.to_string()
}

#[get("/get3/{id}/{name}")]
async fn get_3(info: web::Path<Info>) -> String {
    String::from("data") + &info.name + &info.id.to_string()
}

#[get("/get4/{id}/{name}")]
async fn get_4(req: HttpRequest) -> String {
    let id = req.match_info().get("id").unwrap();
    let name = req.match_info().get("name").unwrap();

    String::from("data ") + &name + &id.to_string()
}

#[get("/get5")]
async fn get_5(info: web::Query<Info>) -> String {
    String::from("data5 ") + &info.name + &info.id.to_string()
}

#[post("/post1")]
async fn post_1(info: web::Json<Info>) -> String {
    String::from("data6 ") + &info.name + &info.id.to_string()
}

#[post("/post2")]
async fn post_2(info: web::Form<Info>) -> String {
    String::from("data6 ") + &info.name + &info.id.to_string()
}

#[get("/get_res")]
async fn get_res() -> HttpResponse {
    HttpResponse::Ok().body("data")
}

#[get("/get_str")]
async fn get_str() -> String {
    String::from("data")
}

#[get("/get_res_str")]
async fn get_res_str() -> Result<String> {
    Ok(String::from("data"))
}

#[get("/get_res_res")]
async fn get_res_res() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("data"))
}

#[get("/get_res_res_json")]
async fn get_res_res_json() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("data"))
}


#[derive(Deserialize)]
struct Info{
    id: u32,
    name: String
}

#[get("/obj/get_res")]
async fn get_obj_res() -> HttpResponse {
    HttpResponse::Ok().body(serde_json::to_string(&MyObj {
        name: String::from("data"),
    }).unwrap())
}

#[get("/obj/get_res_str")]
async fn get_obj_res_str() -> Result<String> {
    Ok(serde_json::to_string(&MyObj {
        name: String::from("data"),
    }).unwrap())
}

#[get("/obj/get_res_res")]
async fn get_obj_res_res() -> Result<HttpResponse> {
    let data = MyObj {
        name: String::from("data"),
    };
    Ok(HttpResponse::Ok().body(serde_json::to_string(&data).unwrap()))
}

#[get("/obj/get_res_res_json")]
async fn get_obj_res_res_json() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(MyObj {
        name: String::from("data"),
    }))
}

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
}



fn url_config(cfg: &mut web::ServiceConfig) {
    cfg
    .service(get_1)
    .service(get_2)
    .service(get_3)
    .service(get_4)
    .service(get_5)
    .service(post_1)
    .service(post_2)
    .service(get_res)
    .service(get_str)
    .service(get_res_str)
    .service(get_res_res)
    .service(get_res_res_json)

    .service(get_obj_res)
    .service(get_obj_res_str)
    .service(get_obj_res_res)
    .service(get_obj_res_res_json)
    ;
}

#[actix_web::main]
async fn main() -> Result<()> {
    let app = || {
        App::new()
            .service(web::scope("/api/core/config/app").configure(app_config))
            .service(web::scope("/api/core/config/item").configure(item_config))
            .service(web::scope("/test").configure(url_config))
    };

    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
}





