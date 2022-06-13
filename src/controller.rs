use crate::models::{Debit, User};
use actix_web::{get, web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
use std::sync::Mutex;
#[get("/allusers")]
pub async fn get_all_users(db_pool: web::Data<Mutex<Pool>>) -> impl Responder {
    let client: Client = db_pool
        .lock()
        .unwrap()
        .get()
        .await
        .expect("Error occured while connecting with database");
    let res = crate::database::get_users(client).await;
    match res {
        Ok(all_users) => HttpResponse::Ok().json(all_users),
        _ => HttpResponse::InternalServerError().into(),
    }
}

//Todo::This should be post method
#[get("/debit")]
pub async fn debit_transaction(db_pool: web::Data<Mutex<Pool>>) -> impl Responder {
    let client: Client = db_pool
        .lock()
        .unwrap()
        .get()
        .await
        .expect("Error occured while connecting with database");
    let obj: Debit = Debit {
        id: 3,
        debit_amount: 500,
        reason: String::from("Party"),
        uid: 1,
    };
    let res = crate::database::debit(client, obj).await;
    match res {
        Ok(all_users) => HttpResponse::Ok().json(all_users),
        _ => HttpResponse::InternalServerError().into(),
    }
}
