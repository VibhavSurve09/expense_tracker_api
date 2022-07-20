use crate::models::{Response, ShowUser, User};
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
#[post("/login")]
pub async fn handle_login(
    db_pool: web::Data<Mutex<Pool>>,
    username: web::Json<ShowUser>,
) -> impl Responder {
    println!("Requesting..");
    let client: Client = db_pool.lock().unwrap().get().await.unwrap();
    let user = crate::database::users::handle_login(&client, &username).await;
    match user {
        Ok(credentials) => return web::Json(credentials),
        _ => {
            let response = Response::new(401, "Unauthorized".to_string());
            return web::Json(user.unwrap());
        }
    }
}
