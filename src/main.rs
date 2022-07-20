use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder};
use deadpool_postgres::{Manager, Pool, Runtime};
use dotenv::dotenv;
use std::io;
use std::sync::Mutex;
use tokio_postgres::NoTls;
mod config;
mod controllers;
mod database;
mod models;
use std::env;
mod errors;
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
    let env_front_end = "FRONT_END";
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default() // <- Construct CORS middleware builder
                    .allowed_origin(env::var(env_front_end).unwrap().as_str())
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .app_data(pool.clone())
            .service(controllers::debit::debit_transaction)
            .service(controllers::users::handle_signup)
            .service(controllers::credit::credit_transaction)
            .service(hello_world)
            .service(controllers::users::handle_login)
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
