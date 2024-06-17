use std::env;
use axum::{Json, Router, routing::get};
use csv::ReaderBuilder;
use serde::Deserialize;
use serde_json::Value;
mod get_monthly_value;
mod cors;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use dotenv::dotenv;
#[derive(Debug, Deserialize)]
struct Record{
    date: String,
    narration: String,
    value_dat: String,
    debit_amount: String,
    credit_amount: String,
    chq_ref_number: String,
    closing_balance: String

}

#[tokio::main]
async fn main() {

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("🔥 Failed to connect to the database");

    println!("✅ Connection to the database is successful!");
    let cors = cors::cors();
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/get_monthly_analysis", get(get_month_data))
        .route("/scan_and_insert", get({
            let pool = pool.clone();
            move || scan_and_insert(pool.clone())
        }))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_month_data() -> Json<Value> {
    get_monthly_value::get_monthly_value()
}

async fn scan_and_insert( pool: Pool<Postgres>) -> Result<Json<Value>, String> {
    let file_path = "/Users/anjalafghan/RustroverProjects/react_rust/backend/src/data.delimited";
    let reader = ReaderBuilder::new().delimiter(b',').from_path(file_path).map_err(|e| e.to_string());

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