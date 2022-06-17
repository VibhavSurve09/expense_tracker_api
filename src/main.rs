use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use deadpool_postgres::{Manager, Pool, Runtime};
use dotenv::dotenv;
use std::io;
use std::sync::Mutex;
use tokio_postgres::NoTls;
mod config;
mod controllers;
mod database;
mod models;

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let config = crate::config::Configuration::from_env();
    let pool = web::Data::new(Mutex::new(
        config.pg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap(),
    ));
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(controllers::debit::debit_transaction)
            .service(controllers::users::handle_signup)
            .service(hello_world)
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
