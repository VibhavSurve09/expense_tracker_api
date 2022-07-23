use crate::models::{ShowUser, User, WebUserResponse};
use actix_web::http::header;
use actix_web::{cookie::Cookie, get, http, post, web, HttpRequest, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
//All Temp Structs
#[derive(Serialize, Deserialize)]
pub struct Email {
    pub email: String,
}
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
) -> HttpResponse {
    let client: Client = db_pool.lock().unwrap().get().await.unwrap();
    let user = crate::database::users::handle_login(&client, &username).await;
    match user {
        Ok(credentials) => {
            let cookie: Cookie = Cookie::build("et_tid", format!("{}", &credentials.tid))
                .http_only(true)
                .finish();

            let response = WebUserResponse::new(200, "success".to_string(), Some(credentials));
            return HttpResponse::Ok().cookie(cookie).json(response);
        }
        _ => {
            let response = WebUserResponse::new(401, "Unauthorized".to_string(), None);
            return HttpResponse::Ok().json(response);
        }
    }
}

#[post("/update/email")]
pub async fn handle_change_email(
    db_pool: web::Data<Mutex<Pool>>,
    email: web::Json<Email>,
    req: HttpRequest,
) -> HttpResponse {
    let client: Client = db_pool.lock().unwrap().get().await.unwrap();
    let cookie_tid = req.cookie("et_tid");
    println!("{:?}", cookie_tid);
    match cookie_tid {
        Some(tid) => {
            let tid: i32 = tid.to_string().parse().unwrap();
            crate::database::users::update_email(&client, &email, tid).await;
            return HttpResponse::Ok().finish();
        }
        _ => {
            return HttpResponse::Forbidden().finish();
        }
    }
}
