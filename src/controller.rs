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
        .expect("Error Connecting to database");
    let res = crate::database::get_users(client).await;
    match res {
        Ok(all_users) => HttpResponse::Ok().json(all_users),
        _ => HttpResponse::InternalServerError().into(),
    }
}
