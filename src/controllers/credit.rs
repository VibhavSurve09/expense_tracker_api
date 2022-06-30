use crate::models::{Credit, User};
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use deadpool_postgres::{Client, Pool};
use std::sync::Mutex;

#[post("/credit")]
pub async fn credit_transaction(
    db_pool: web::Data<Mutex<Pool>>,
    credit: web::Json<Credit>,
) -> impl Responder {
    println!("Crediting..");
    let client: Client = db_pool
        .lock()
        .unwrap()
        .get()
        .await
        .expect("Error occured while connecting with database");
    let res = crate::database::credit::credit(client, credit).await;
    match res {
        Ok(transaction_) => HttpResponse::Ok().json(transaction_),
        _ => HttpResponse::InternalServerError().into(),
    }
}
