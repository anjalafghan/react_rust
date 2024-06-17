use std::env;
use axum::{Json, Router, routing::get};
use serde_json::Value;
mod get_monthly_value;
mod cors;
mod scan_and_insert;
mod get_all_data;

use sqlx::postgres::PgPoolOptions;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
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
#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect(" Failed to connect to the database");

    println!(" Connection to the database is successful!");
    let cors = cors::cors();
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/get_monthly_analysis", get(get_month_data))
        .route("/scan_and_insert", get({
            let pool = pool.clone();
            move || scan_and_insert::scan_and_insert(pool.clone())
        }))
        .route("/display", get({
            let pool = pool.clone();
            move || get_all_data::display(pool.clone())
        }))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_month_data() -> Json<Value> {
    get_monthly_value::get_monthly_value()
}
