use axum::{Json, Router, response::IntoResponse, routing::get};
use serde_json::json;
use std::sync::Arc;

use crate::logic::Market;

async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "message": "La estamos jogeando"
    }))
}

pub fn create_app(market: Arc<Market>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .with_state(market)
}
