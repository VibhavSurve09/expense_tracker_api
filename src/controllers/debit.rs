use crate::models::{Debit, User};
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use deadpool_postgres::{Client, Pool};
use std::sync::Mutex;

#[post("/debit")]
pub async fn debit_transaction(
    db_pool: web::Data<Mutex<Pool>>,
    debit_: web::Json<Debit>,
) -> impl Responder {
    let client: Client = db_pool
        .lock()
        .unwrap()
        .get()
        .await
        .expect("Error occured while connecting with database");
    let res = crate::database::debit::debit(client, debit_).await;
    match res {
        Ok(all_users) => HttpResponse::Ok().json(all_users),
        _ => HttpResponse::InternalServerError().into(),
    }
}
