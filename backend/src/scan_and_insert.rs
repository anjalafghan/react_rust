use axum::Json;
use serde_json::Value;
use csv::{ReaderBuilder, Trim, StringRecord};
use sqlx::{Pool, Postgres};
use crate::Record;

pub async fn scan_and_insert(pool: Pool<Postgres>) -> Result<Json<Value>, String> {
    let file_path = "/Users/anjalafghan/RustroverProjects/react_rust/backend/src/output.csv";
    let mut reader = ReaderBuilder::new()
        .trim(Trim::All)
        .delimiter(b',')
        .flexible(true)  // This allows for a flexible number of fields
        .from_path(file_path)
        .map_err(|e| e.to_string())?;

    let expected_fields = 7;  // The number of fields we expect (excluding Label)

    for result in reader.records() {
        let record = result.map_err(|e| e.to_string())?;
        let record_struct = parse_record(&record, expected_fields)?;

        sqlx::query!(
            "INSERT INTO finance_data (date, narration,chq_ref_number, value_dat, debit_amount, credit_amount,closing_balance, label) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            record_struct.date, record_struct.narration,record_struct.chq_ref_number, record_struct.value_dat, record_struct.debit_amount, record_struct.credit_amount, record_struct.closing_balance, record_struct.label
        )
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;
    }
    Ok(Json(serde_json::json!({"status": "Data has been inserted successfully!"})))
}

fn parse_record(record: &StringRecord, expected_fields: usize) -> Result<Record, String> {
    Ok(Record {
        date: record.get(0).unwrap_or("").to_string(),
        narration: record.get(1).unwrap_or("").to_string(),
        chq_ref_number: record.get(2).unwrap_or("").to_string(),
        value_dat: record.get(3).unwrap_or("").to_string(),
        debit_amount: record.get(4).unwrap_or("").to_string(),
        credit_amount: record.get(5).unwrap_or("").to_string(),
        closing_balance: record.get(6).unwrap_or("").to_string(),
        label: if record.len() > expected_fields {
            record.get(7).unwrap_or("").to_string()
        } else {
            String::new()
        },
    })
}
