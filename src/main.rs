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
extern crate redis;
use redis::Commands;
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
    let con_uri: String = dotenv::var("REDIS_URI").unwrap();

    let client = redis::Client::open(con_uri).unwrap();
    let connection = web::Data::new(Mutex::new(client.get_connection().unwrap()));
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default() // <- Construct CORS middleware builder
                    .allowed_origin(env::var(env_front_end).unwrap().as_str())
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials()
                    .expose_any_header(),
            )
            .app_data(pool.clone())
            .app_data(connection.clone())
            .service(controllers::users::handle_change_email)
            .service(controllers::debit::debit_transaction)
            .service(controllers::users::handle_signup)
            .service(controllers::credit::credit_transaction)
            .service(hello_world)
            .service(controllers::users::handle_login)
            .service(controllers::credit::get_credit)
            .service(controllers::debit::delete_debit)
            .service(controllers::debit::get_debit)
            .service(controllers::credit::delete_credit)
            .service(controllers::credit::update_credit)
            .service(controllers::debit::update_debit)
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
