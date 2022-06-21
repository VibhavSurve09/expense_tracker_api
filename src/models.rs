use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct User {
    pub tid: i32,
    pub uname: String,
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
#[derive(Serialize)]
pub struct Status {
    message: String,
}

#[derive(Serialize, Deserialize, PostgresMapper, Debug)]
#[pg_mapper(table = "debit")]
pub struct ShowDebit {
    pub id: i32,
    pub reason: String,
}
