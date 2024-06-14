use axum::http::Method;
use axum::{routing::get, Json, Router};
use serde_json::Value;
use tower_http::cors::{Any, CorsLayer};
mod get_monthly_value;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/get_monthly_analysis", get(get_month_data))
        .layer(cors);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_month_data() -> Json<Value> {
    get_monthly_value::get_monthly_value()
}
