use sqlx::{Pool, Postgres};
use axum::Json;
use serde_json::Value;
use csv::{ReaderBuilder, Trim};
use serde::{Deserialize};
#[derive(Debug, Deserialize)]
struct Record{
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Narration")]
    narration: String,
    #[serde(rename = "Value Dat")]
    value_dat: String,
    #[serde(rename = "Debit Amount")]
    debit_amount: String,
    #[serde(rename = "Credit Amount")]
    credit_amount: String,
    #[serde(rename = "Chq/Ref Number")]
    chq_ref_number: String,
    #[serde(rename = "Closing Balance")]
    closing_balance: String

}
pub async fn scan_and_insert(pool: Pool<Postgres>) -> Result<Json<Value>, String> {
    let file_path = "/Users/anjalafghan/RustroverProjects/react_rust/backend/src/data.delimited";
    let reader = ReaderBuilder::new().trim(Trim::All).delimiter(b',').from_path(file_path).map_err(|e| e.to_string());

    for result in reader?.deserialize(){
        let record: Record = result.map_err(|e| e.to_string())?;
        sqlx::query!("INSERT INTO finance_data (date, narration, value_dat, debit_amount, credit_amount, chq_ref_number, closing_balance) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        record.date, record.narration, record.value_dat, record.debit_amount, record.credit_amount, record.chq_ref_number, record.closing_balance)
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;
    }
    Ok(Json(serde_json::json!({"status": "Data has been inserted successfully!"})))
}
