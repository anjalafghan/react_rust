use axum::Json;
use serde_json::{json, Value};

pub fn get_monthly_value() -> Json<Value> {
    Json(json!({ "data": 42 }))
}
