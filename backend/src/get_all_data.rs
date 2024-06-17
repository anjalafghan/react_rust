use sqlx::{Pool, Postgres};
use axum::Json;
use crate::Record;

pub async fn display(pool: Pool<Postgres>) -> Json<String> {
    let rows = match sqlx::query!("SELECT * FROM finance_data")
        .fetch_all(&pool)
        .await {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("Failed to fetch data from database: {}", e);
            return Json("Failed to fetch data from database".to_string());
        }
    };

    println!("Fetched {} rows from finance_data table", rows.len());

    let finance_data: Vec<Record> = rows.iter().map(|row| {
        Record {
            date: row.date.clone().unwrap_or_else(|| "".to_string()),
            narration: row.narration.clone().unwrap_or_else(|| "".to_string()),
            value_dat: row.value_dat.clone().unwrap_or_else(|| "".to_string()),
            debit_amount: row.debit_amount.clone().unwrap_or_else(|| "".to_string()),
            credit_amount: row.credit_amount.clone().unwrap_or_else(|| "".to_string()),
            chq_ref_number: row.chq_ref_number.clone().unwrap_or_else(|| "".to_string()),
            closing_balance: row.closing_balance.clone().unwrap_or_else(|| "".to_string()),
        }
    }).collect();

    let json_result = serde_json::to_string(&finance_data)
        .expect("Failed to serialize finance_data to JSON");

    Json(json_result)
}
