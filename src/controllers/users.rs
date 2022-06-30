use crate::models::User;
use actix_web::{get, http, post, web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
use std::sync::Mutex;

#[post("/signup")]
pub async fn handle_signup(
    db_pool: web::Data<Mutex<Pool>>,
    user_credentials: web::Json<User>,
) -> impl Responder {
    let client: Client = db_pool.lock().unwrap().get().await.unwrap();
    let _user = crate::database::users::get_user_by_username(&client, &user_credentials).await;
    match _user {
        None => {
            let new_user = crate::database::users::add_user(&client, &user_credentials)
                .await
                .unwrap();
            HttpResponse::Ok().json(new_user)
        }
        Some(_) => {
            let res = HttpResponse::new(http::StatusCode::CONFLICT);
            res
        }
    }
}
