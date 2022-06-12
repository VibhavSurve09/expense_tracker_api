use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::io;
#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("data")
}
#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(hello_world))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
