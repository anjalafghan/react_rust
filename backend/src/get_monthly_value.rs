use axum::Json;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{Pool, Postgres, Row};

#[derive(Debug, Deserialize, Serialize)]
struct MonthlyDebit {
    year_month: String,
    total_debit: Decimal,
    total_credit: Decimal,
    balance: Decimal,
}

pub async fn get_monthly_debit(pool: Pool<Postgres>) -> Result<Json<Value>, String> {
    let query = r#"
         SELECT
    TO_CHAR(TO_DATE(date, 'DD/MM/YYYY'), 'MM-YYYY') AS month_year,
    SUM(CAST(REPLACE(debit_amount, ',', '') AS NUMERIC)) AS total_debit,
    SUM(CAST(REPLACE(credit_amount, ',', '') AS NUMERIC)) AS total_credit,
    AVG(CAST(REPLACE(closing_balance, ',', '') AS NUMERIC)) AS avg_closing_balance
FROM
    finance_data
GROUP BY
    TO_CHAR(TO_DATE(date, 'DD/MM/YYYY'), 'MM-YYYY')
ORDER BY
    TO_DATE('01-' || TO_CHAR(TO_DATE(date, 'DD/MM/YYYY'), 'MM-YYYY'), 'DD-MM-YYYY');
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
                year_month: row.get("month_year"),
                total_debit,
                total_credit,
                balance,
            }
        })
        .collect();

    Ok(Json(json!(monthly_debit)))
}
