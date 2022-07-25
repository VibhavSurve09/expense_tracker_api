use crate::models::{Credit, User};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use chrono::prelude::*;
use deadpool_postgres::{Client, Pool};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
extern crate redis;
use redis::Commands;
use tokio_pg_mapper_derive::PostgresMapper;
//Temp Structs
#[derive(Serialize, Deserialize)]
struct WebCreditResponse {
    pub status: i32,
    pub message: String,
    pub data: Option<Vec<WebCredit>>,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "credit")]
pub struct WebCredit {
    pub credit_amount: i32,
    pub reason: String,
    pub uid: i32,
    pub transaction_date: String,
    pub id: i32,
}

#[post("/credit")]
pub async fn credit_transaction(
    db_pool: web::Data<Mutex<Pool>>,
    credit: web::Json<Credit>,
) -> impl Responder {
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

#[get("/credit")]
pub async fn get_credit(
    db_pool: web::Data<Mutex<Pool>>,
    redit_client: web::Data<Mutex<redis::Connection>>,
    req: HttpRequest,
) -> HttpResponse {
    let cookie = req.cookie("et_tid");
    match (cookie) {
        Some(_cookie) => {
            let tid_str = _cookie.value();
            let tid_num: i32 = _cookie.value().parse().unwrap();
            let pg_client: Client = db_pool.lock().unwrap().get().await.unwrap();
            let res = crate::database::credit::get_credit(pg_client, tid_num)
                .await
                .unwrap();
            let response = WebCreditResponse {
                status: 200,
                message: "success".to_string(),
                data: Some(res),
            };
            return HttpResponse::Ok().json(response);
        }
        _ => {
            let response = WebCreditResponse {
                status: 403,
                message: "Not Authenticated".to_string(),
                data: None,
            };
            return HttpResponse::Ok().json(response);
        }
    }
}
