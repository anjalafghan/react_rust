use tower_http::cors::{Any, CorsLayer};
use axum::http::Method;

pub fn cors() -> CorsLayer {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    cors
}
