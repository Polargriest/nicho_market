use axum::{
    Json, Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};

use crate::logic::{errors::ApiError, market::Market};

async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "message": "La estamos jogeando"
    }))
}

async fn get_users(State(store): State<Arc<Mutex<Market>>>) -> impl IntoResponse {
    Json(store.lock().unwrap().list_users())
}

async fn get_tickers(State(store): State<Arc<Mutex<Market>>>) -> impl IntoResponse {
    Json(store.lock().unwrap().list_tickers())
}

async fn buy_actions_endpoint(
    State(store): State<Arc<Mutex<Market>>>,
    Path((user_id, ticker_id, amount)): Path<(i32, i32, i32)>,
) -> Result<Json<Value>, ApiError> {
    store
        .lock()
        .unwrap()
        .buy_actions(user_id, ticker_id, amount)?;

    Ok(Json(json!({
        "result": "ok",
    })))
}

async fn sell_actions_endpoint(
    State(store): State<Arc<Mutex<Market>>>,
    Path((user_id, ticker_id, amount)): Path<(i32, i32, i32)>,
) -> Result<Json<Value>, ApiError> {
    store
        .lock()
        .unwrap()
        .sell_actions(user_id, ticker_id, amount)?;

    Ok(Json(json!({
        "result": "ok",
    })))
}

pub fn create_app(market: Arc<Mutex<Market>>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/tickers", get(get_tickers))
        .route("/users", get(get_users))
        .route(
            "/buy/{user_id}/{ticker_id}/{amount}",
            post(buy_actions_endpoint),
        )
        .route(
            "/sell/{user_id}/{ticker_id}/{amount}",
            post(sell_actions_endpoint),
        )
        .with_state(market)
}
