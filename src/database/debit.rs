use crate::controllers::debit::{DebitUpdateSchema, WebDebit};
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

pub async fn get_debit(
    client: Client,
    tid: i32,
    offset_val: i64,
) -> Result<Vec<WebDebit>, io::Error> {
    let _stmt = include_str!("./sql/get_debit.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    //Int8 is bigint in psql which is i64 in different languages
    let recent: Vec<WebDebit> = client
        .query(&stmt, &[&tid, &offset_val])
        .await
        .expect("Error while credit")
        .iter()
        .map(|row| WebDebit::from_row_ref(row).unwrap())
        .collect::<Vec<WebDebit>>();
    Ok(recent)
}

pub async fn delete_debit(client: Client, debit_id: i32, tid: i32) {
    let _stmt = include_str!("./sql/delete_debit_transaction.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    client.query(&stmt, &[&debit_id, &tid]).await;
}

pub async fn update_debit(
    client: Client,
    data: DebitUpdateSchema,
    cookie: i32,
) -> Result<Vec<WebDebit>, ()> {
    let _stmt = include_str!("./sql/update_debit.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    let transaction: Vec<WebDebit> = client
        .query(
            &stmt,
            &[&data.debit_amount, &data.message, &data.id, &cookie],
        )
        .await
        .expect("Error while updating debit")
        .iter()
        .map(|row| WebDebit::from_row_ref(row).unwrap())
        .collect::<Vec<WebDebit>>();

    return Ok(transaction);
}
