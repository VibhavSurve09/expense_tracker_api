use crate::models::{Debit, ShowDebit};
use actix_web::web;
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn debit(client: Client, debit: web::Json<Debit>) -> Result<ShowDebit, io::Error> {
    let _stmt = include_str!("./sql/debit_transaction.sql");
    let _stmt = _stmt.replace("$table_fields", &ShowDebit::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    let recent: ShowDebit = client
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
        .map(|row| ShowDebit::from_row_ref(row).unwrap())
        .collect::<Vec<ShowDebit>>()
        .pop()
        .unwrap();
    Ok(recent)
}
