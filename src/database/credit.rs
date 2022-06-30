use crate::models::{Credit, ShowCredit};
use actix_web::web;
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn credit(client: Client, credit: web::Json<Credit>) -> Result<ShowDebit, io::Error> {
    let _stmt = include_str!("./sql/credit_transaction.sql");
    let _stmt = _stmt.replace("$table_fields", &ShowCredit::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    let recent: ShowCredit = client
        .query(
            &stmt,
            &[
                &credit.credit_amount,
                &credit.reason,
                &credit.uid,
                &credit.transaction_date,
            ],
        )
        .await
        .expect("Error while credit")
        .iter()
        .map(|row| ShowCredit::from_row_ref(row).unwrap())
        .collect::<Vec<ShowCredit>>()
        .pop()
        .unwrap();
    Ok(recent)
}
