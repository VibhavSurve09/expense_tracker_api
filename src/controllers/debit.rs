use crate::models::{Debit, User};
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
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
#[derive(Serialize, Deserialize)]
pub struct DebitUpdateSchema {
    pub id: i32,
    pub message: String,
    pub debit_amount: i32,
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

#[delete("/debit/delete/{id}")]
pub async fn delete_debit(
    db_pool: web::Data<Mutex<Pool>>,
    request: HttpRequest,
    id: web::Path<String>,
) -> HttpResponse {
    let cookie = request.cookie("et_tid");
    let pg_client: Client = db_pool.lock().unwrap().get().await.unwrap();
    match cookie {
        Some(cookie) => {
            let debit_id: i32 = id.into_inner().trim().parse().unwrap();
            let cookie_val: i32 = cookie.value().to_string().trim().parse().unwrap();
            crate::database::debit::delete_debit(pg_client, debit_id, cookie_val).await;
            return HttpResponse::Ok().finish();
        }
        None => {
            return HttpResponse::Forbidden().finish();
        }
    }
}

#[post("/debit/update")]
pub async fn update_debit(
    db_pool: web::Data<Mutex<Pool>>,
    request: HttpRequest,
    transaction: web::Json<DebitUpdateSchema>,
) -> HttpResponse {
    let cookie = request.cookie("et_tid");
    let pg_client: Client = db_pool.lock().unwrap().get().await.unwrap();
    match cookie {
        Some(cookie_) => {
            let cookie_val: i32 = cookie_.value().to_string().trim().parse().unwrap();
            let res = crate::database::debit::update_debit(
                pg_client,
                transaction.into_inner(),
                cookie_val,
            )
            .await;
            if let Ok(valid_update) = res {
                let new_res = WebDebitResponse {
                    status: 201,
                    message: "success".to_string(),
                    data: Some(valid_update),
                };
                return HttpResponse::Ok().json(new_res);
            } else {
                let new_res = WebDebitResponse {
                    status: 400,
                    message: "fail".to_string(),
                    data: None,
                };
                return HttpResponse::Ok().json(new_res);
            }
        }
        _ => {
            let new_res = WebDebitResponse {
                status: 400,
                message: "fail".to_string(),
                data: None,
            };
            return HttpResponse::Ok().json(new_res);
        }
    }
}
