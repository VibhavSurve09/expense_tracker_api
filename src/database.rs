use crate::models::{Debit, User};
use actix_web::web;
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn debit(client: Client, debit: web::Json<Debit>) -> Result<Debit, io::Error> {
    let _stmt = include_str!("../sql/debit_transaction.sql");
    let _stmt = _stmt.replace("$table_fields", &Debit::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    let recent: Debit = client
        .query(
            &stmt,
            &[
                &debit.debit_amount,
                &debit.reason,
                &debit.uid,
                &debit.transaction_date,
            ],
        )
        .await
        .expect("Error while debit")
        .iter()
        .map(|row| Debit::from_row_ref(row).unwrap())
        .collect::<Vec<Debit>>()
        .pop()
        .unwrap();
    Ok(recent)
}
