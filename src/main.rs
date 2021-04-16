use actix_web::{
    error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
};
use futures::StreamExt;
use json::JsonValue;
use serde::{Deserialize, Serialize};

mod db;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    user: String,
    email: String,
    pass: String,
    name: String,
    subname: String,
    phone: String,
    token: String,
}

async fn index(item: web::Json<User>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0) // <- send response
}

async fn extract_item(item: web::Json<User>, req: HttpRequest) -> HttpResponse {
    println!("request: {:?}", req);
    println!("model: {:?}", item);

    HttpResponse::Ok().json(item.0) // <- send json response
}

////////////////////////////////////////////////////////////

async fn resetDb() -> HttpResponse {
    db::run();
    db::get_user();
    return HttpResponse::Ok().finish(); // <- send response
}

async fn getUserWithToken(req: HttpRequest) -> HttpResponse {
    let token = req.headers().get("token").unwrap();
    println!("token: {:?}", token);
    HttpResponse::Ok().json(User{
        id: 0, 
        user: "andreslab".to_string(),
        email: "andreslab.dev@gmail.com".to_string(),
        pass: "eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ".to_string(),
        name:"Jaime".to_string(),
        subname: "Andrade".to_string(),
        phone: "0992811123".to_string(),
        token: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9".to_string()
    }) // <- send response
}

async fn deleteUserWithToken(req: HttpRequest) -> HttpResponse {
    let token = req.headers().get("token").unwrap();
    println!("token: {:?}", token);
    HttpResponse::Accepted().finish() // <- send response
}

async fn getUser(req: HttpRequest) -> HttpResponse {
    let user = req.match_info().get("user").unwrap_or("World");
    format!("Hello {}!", &user);
    HttpResponse::Ok().json(User{
        id: 0, 
        user: "andreslab".to_string(),
        email: "andreslab.dev@gmail.com".to_string(),
        pass: "eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ".to_string(),
        name:"Jaime".to_string(),
        subname: "Andrade".to_string(),
        phone: "0992811123".to_string(),
        token: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9".to_string()
    }) // <- send response
}

async fn addUser(item: web::Json<User>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Created().finish() // <- send response
}

async fn updateUser(item: web::Json<User>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Accepted().finish()// <- send response
}

async fn deleteUser(req: HttpRequest) -> HttpResponse {
    let user = req.match_info().get("user").unwrap_or("World");
    format!("Hello {}!", &user);
    HttpResponse::Accepted().finish() // <- send response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/extractor").route(web::post().to(index)))
            .service(
                web::resource("/extractor2")
                    .data(web::JsonConfig::default().limit(1024)) // <- limit size of the payload (resource level)
                    .route(web::post().to(extract_item)),
            )
            .service(web::resource("/").route(web::post().to(index)))
            .service(web::resource("/reset").route(web::get().to(resetDb)))
            .service(web::resource("/get/{user}").route(web::get().to(getUser)))
            .service(web::resource("/getwithtoken").route(web::get().to(getUserWithToken)))
            .service(web::resource("/add").route(web::post().to(addUser)))
            .service(web::resource("/update").route(web::post().to(updateUser)))
            .service(web::resource("/delete/{user}").route(web::get().to(deleteUser)))
            .service(web::resource("/deletewithtoken").route(web::get().to(deleteUserWithToken)))
            
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}