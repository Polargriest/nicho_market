use axum::{
    Json, Router,
    extract::{Path, State},
    http::HeaderMap,
    response::IntoResponse,
    routing::{get, post},
};
use serde::Serialize;
use serde_json::json;
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use crate::{
    logic::{errors::ApiError, market::Market},
    schema::{Ticker, User},
};

#[derive(Serialize)]
pub struct TransactionResult {
    pub user: User,
    pub ticker: Ticker,
}

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

async fn check_user(token: &str) -> Result<String, ApiError> {
    if !token.starts_with("Bearer ") {
        return Err(ApiError::Unauthorized);
    }

    let uuid_token = token.trim_start_matches("Bearer ");

    let Ok(valid) = uuid::Uuid::from_str(uuid_token) else {
        return Err(ApiError::Unauthorized);
    };

    Ok(valid.to_string())
}

async fn buy_actions_endpoint(
    State(store): State<Arc<Mutex<Market>>>,
    Path((ticker_id, amount)): Path<(i32, i32)>,
    headers: HeaderMap,
) -> Result<Json<TransactionResult>, ApiError> {
    let Some(token) = headers.get("Authorization") else {
        return Err(ApiError::Unauthorized);
    };

    let Ok(token_str) = token.to_str() else {
        return Err(ApiError::Unauthorized);
    };

    let valid_token = check_user(token_str).await?;

    let Some(user_id) = store.lock().unwrap().get_user_id_from_token(&valid_token) else {
        return Err(ApiError::Unauthorized);
    };

    let result = store
        .lock()
        .unwrap()
        .buy_actions(user_id, ticker_id, amount)?;

    Ok(Json(result))
}

async fn sell_actions_endpoint(
    State(store): State<Arc<Mutex<Market>>>,
    Path((ticker_id, amount)): Path<(i32, i32)>,
    headers: HeaderMap,
) -> Result<Json<TransactionResult>, ApiError> {
    let Some(token) = headers.get("Authorization") else {
        return Err(ApiError::Unauthorized);
    };

    let Ok(token_str) = token.to_str() else {
        return Err(ApiError::Unauthorized);
    };

    let valid_token = check_user(token_str).await?;

    let Some(user_id) = store.lock().unwrap().get_user_id_from_token(&valid_token) else {
        return Err(ApiError::Unauthorized);
    };

    let result = store
        .lock()
        .unwrap()
        .sell_actions(user_id, ticker_id, amount)?;

    Ok(Json(result))
}

pub fn create_app(market: Arc<Mutex<Market>>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/tickers", get(get_tickers))
        .route("/users", get(get_users))
        .route("/buy/{ticker_id}/{amount}", post(buy_actions_endpoint))
        .route("/sell/{ticker_id}/{amount}", post(sell_actions_endpoint))
        .with_state(market)
}
