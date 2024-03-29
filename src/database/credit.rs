use crate::controllers::credit::{CreditUpdateSchema, WebCredit};
use crate::models::{Credit, ShowCredit};
use actix_web::web;
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn credit(client: Client, credit: web::Json<Credit>) -> Result<ShowCredit, io::Error> {
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

pub async fn get_credit(
    client: Client,
    tid: i32,
    offset_val: i64,
) -> Result<Vec<WebCredit>, io::Error> {
    let _stmt = include_str!("./sql/get_credit.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    //Int8 is bigint in psql which is i64 in different languages
    let recent: Vec<WebCredit> = client
        .query(&stmt, &[&tid, &offset_val])
        .await
        .expect("Error while credit")
        .iter()
        .map(|row| WebCredit::from_row_ref(row).unwrap())
        .collect::<Vec<WebCredit>>();
    Ok(recent)
}
pub async fn delete_credit(client: Client, debit_id: i32, tid: i32) {
    let _stmt = include_str!("./sql/delete_credit_transaction.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    client.query(&stmt, &[&debit_id, &tid]).await;
}

pub async fn update_credit(
    client: Client,
    data: CreditUpdateSchema,
    cookie: i32,
) -> Result<Vec<WebCredit>, ()> {
    let _stmt = include_str!("./sql/update_credit.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();
    let transaction: Vec<WebCredit> = client
        .query(
            &stmt,
            &[&data.credit_amount, &data.message, &data.id, &cookie],
        )
        .await
        .expect("Error while updating debit")
        .iter()
        .map(|row| WebCredit::from_row_ref(row).unwrap())
        .collect::<Vec<WebCredit>>();

    return Ok(transaction);
}
