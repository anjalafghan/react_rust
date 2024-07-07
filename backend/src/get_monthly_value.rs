use axum::Json;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{Pool, Postgres, Row};
#[derive(Debug, Deserialize, Serialize)]
struct MonthlyDebit {
    month: String,
    year: String,
    total_debit: Decimal,
    total_credit: Decimal,
    balance: Decimal,
}
pub async fn get_monthly_debit(pool: Pool<Postgres>) -> Result<Json<Value>, String> {
    let query = r#"
        SELECT
            TO_CHAR(TO_DATE(date, 'DD/MM/YYYY'), 'Month') AS month,
            TO_CHAR(TO_DATE(date, 'DD/MM/YYYY'), 'YYYY') AS year,
            SUM(CAST(debit_amount AS NUMERIC)) AS total_debit,
            SUM(CAST(credit_amount AS NUMERIC)) AS total_credit
        FROM
            finance_data
        WHERE
            debit_amount ~ '^[0-9]+(\.[0-9]+)?$' AND
            credit_amount ~ '^[0-9]+(\.[0-9]+)?$'
        GROUP BY
            TO_CHAR(TO_DATE(date, 'DD/MM/YYYY'), 'Month'),
            TO_CHAR(TO_DATE(date, 'DD/MM/YYYY'), 'YYYY')
        ORDER BY
            MIN(TO_DATE(date, 'DD/MM/YYYY'));
    "#;

    let rows = sqlx::query(query)
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())?;

    println!("Fetched {} rows from finance_data table", rows.len());

    let monthly_debit: Vec<MonthlyDebit> = rows
        .into_iter()
        .map(|row| {
            let total_debit: Decimal = row.get("total_debit");
            let total_credit: Decimal = row.get("total_credit");
            let balance = total_credit - total_debit;
            MonthlyDebit {
                month: row.get("month"),
                year: row.get("year"),
                total_debit,
                total_credit,
                balance,
            }
        })
        .collect();

    Ok(Json(json!(monthly_debit)))
}
