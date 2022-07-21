use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct User {
    pub tid: i32,
    pub uname: String,
}
#[derive(Serialize, Deserialize, PostgresMapper, Debug)]
#[pg_mapper(table = "users")]
pub struct WebUser {
    pub tid: i32,
    pub uname: String,
    pub email: Option<String>,
}
#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct ShowUser {
    pub uname: String,
}
#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "debit")]
pub struct Debit {
    pub debit_amount: i32,
    pub reason: String,
    pub uid: i32,
    pub transaction_date: String,
}

#[derive(Serialize, Deserialize, PostgresMapper, Debug)]
#[pg_mapper(table = "debit")]
pub struct ShowDebit {
    pub id: i32,
    pub reason: String,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "credit")]
pub struct Credit {
    pub credit_amount: i32,
    pub reason: String,
    pub uid: i32,
    pub transaction_date: String,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "credit")]
pub struct ShowCredit {
    pub reason: String,
    pub id: i32,
}
#[derive(Serialize, Deserialize)]
pub struct WebUserResponse {
    status_code: u16,
    message: String,
    user: Option<WebUser>,
}

impl WebUserResponse {
    pub fn new(code: u16, message: String, user: Option<WebUser>) -> Self {
        WebUserResponse {
            status_code: code,
            message,
            user,
        }
    }
}
