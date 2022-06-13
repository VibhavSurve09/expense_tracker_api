use crate::models::{Debit, User};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_users(client: Client) -> Result<Vec<User>, io::Error> {
    let statement = client.prepare("select * from users").await.unwrap();
    let users = client
        .query(&statement, &[])
        .await
        .expect("Error")
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect();
    Ok(users)
}
