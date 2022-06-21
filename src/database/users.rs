use crate::models::{ShowUser, User};
use actix_web::web;
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;
pub async fn get_user_by_username(
    client: &Client,
    user_credentials: &web::Json<User>,
) -> Option<ShowUser> {
    let statement = include_str!("./sql/get_user.sql");
    let statement = client.prepare(&statement).await.unwrap();
    let res = client
        .query(&statement, &[&user_credentials.uname])
        .await
        .expect("Something went wrong while fetching user")
        .iter()
        .map(|row| ShowUser::from_row_ref(row).unwrap())
        .collect::<Vec<ShowUser>>()
        .pop();

    res
}
pub async fn add_user(client: &Client, user: &web::Json<User>) -> Result<User, io::Error> {
    let _stmt = include_str!("./sql/add_user.sql");
    let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    let new_user: User = client
        .query(&stmt, &[&user.tid, &user.uname])
        .await
        .expect("Something went wrong while pushing user")
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .unwrap();
    Ok(new_user)
}
