use crate::models::{Debit, User};
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
struct WebDebitResponse {
    pub status: i32,
    pub message: String,
    pub data: Option<Vec<WebDebit>>,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "debit")]
pub struct WebDebit {
    pub debit_amount: i32,
    pub reason: String,
    pub uid: i32,
    pub transaction_date: String,
    pub id: i32,
}
const TIME_TO_LIVE: usize = 180;
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

#[get("/debit/{pageNum}")]
pub async fn get_debit(
    db_pool: web::Data<Mutex<Pool>>,
    redit_client: web::Data<Mutex<redis::Connection>>,
    req: HttpRequest,
    path: web::Path<String>,
) -> HttpResponse {
    let cookie = req.cookie("et_tid");
    let page_no = path.into_inner().trim().parse::<i64>().unwrap() - 1;
    let offset_val: i64 = page_no * 10;
    match (cookie) {
        Some(_cookie) => {
            let mut redis = redit_client.lock().unwrap();
            let tid_str = _cookie.value().to_string();
            let page_num_str = page_no.to_string();
            let key = tid_str + "_debit_page_num=" + page_num_str.as_str();
            let cache_response: Result<String, redis::RedisError> = redis.get(key.clone());
            match cache_response {
                Ok(res) => {
                    let response: Vec<WebDebit> =
                        serde_json::from_str(&res).expect("Something went");
                    let response = WebDebitResponse {
                        status: 200,
                        message: "success".to_string(),
                        data: Some(response),
                    };
                    return HttpResponse::Ok().json(response);
                }
                _ => {
                    let tid_num: i32 = _cookie.value().parse().unwrap();
                    let pg_client: Client = db_pool.lock().unwrap().get().await.unwrap();
                    let res = crate::database::debit::get_debit(pg_client, tid_num, offset_val)
                        .await
                        .unwrap();
                    let string_res = serde_json::to_string(&res).unwrap();
                    let _: () = redis.set(key.clone(), string_res).unwrap();
                    let _: () = redis.expire(key, TIME_TO_LIVE).unwrap();
                    let response = WebDebitResponse {
                        status: 200,
                        message: "success".to_string(),
                        data: Some(res),
                    };
                    return HttpResponse::Ok().json(response);
                }
            }
        }
        _ => {
            let response = WebDebitResponse {
                status: 403,
                message: "Not Authenticated".to_string(),
                data: None,
            };
            return HttpResponse::Ok().json(response);
        }
    }
}
