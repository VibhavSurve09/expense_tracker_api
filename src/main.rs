use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::io;
mod config;
use dotenv::dotenv;
#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("data")
}
#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let config = crate::config::ServerConfig::from_env();
    HttpServer::new(|| App::new().service(hello_world))
        .bind((config.host, config.port))?
        .run()
        .await
}
