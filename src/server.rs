use axum::{
    Json, Router,
    extract::{FromRequestParts, Path, State},
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{Arc, Mutex};

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

async fn get_users_endpoint(State(store): State<Arc<Mutex<Market>>>) -> impl IntoResponse {
    Json(store.lock().unwrap().list_users())
}

async fn get_user_endpoint(
    State(store): State<Arc<Mutex<Market>>>,
    Path(id): Path<i32>,
) -> Result<Json<User>, ApiError> {
    let user = store
        .lock()
        .unwrap()
        .user_data_by_id(id)
        .ok_or(ApiError::UserNotFound(id))?;

    Ok(Json(user))
}

async fn get_tickers_endpoint(State(store): State<Arc<Mutex<Market>>>) -> impl IntoResponse {
    Json(store.lock().unwrap().list_tickers())
}

async fn get_ticker_endpoint(
    State(store): State<Arc<Mutex<Market>>>,
    Path(id): Path<i32>,
) -> Result<Json<Ticker>, ApiError> {
    let ticker = store
        .lock()
        .unwrap()
        .get_ticker_by_id(id)
        .ok_or(ApiError::TickerNotFound(id))?;

    Ok(Json(ticker))
}

struct AuthenticatedUser(i32);

impl FromRequestParts<Arc<Mutex<Market>>> for AuthenticatedUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &Arc<Mutex<Market>>,
    ) -> Result<Self, Self::Rejection> {
        // try exctracting the "Authorization" header
        let header = parts
            .headers
            .get("Authorization")
            .ok_or(ApiError::Unauthorized)?
            .to_str()
            .map_err(|_| ApiError::Unauthorized)?;

        // we got the header! now let's strip the "Bearer " prefix
        let token = header
            .strip_prefix("Bearer ")
            .ok_or(ApiError::Unauthorized)?;

        // we now have a token that allegedly belongs to an user. Let's fetch it.
        let user_id = state
            .lock()
            .unwrap()
            .get_user_id_from_token(token)
            .ok_or(ApiError::Unauthorized)?;

        Ok(AuthenticatedUser(user_id))
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddTickerRequest {
    name: String,
    description: String,
}

async fn buy_actions_endpoint(
    State(store): State<Arc<Mutex<Market>>>,
    Path((ticker_id, amount)): Path<(i32, i32)>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<Json<TransactionResult>, ApiError> {
    let result = store
        .lock()
        .unwrap()
        .buy_actions(user_id, ticker_id, amount)?;

    Ok(Json(result))
}

async fn sell_actions_endpoint(
    State(store): State<Arc<Mutex<Market>>>,
    Path((ticker_id, amount)): Path<(i32, i32)>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<Json<TransactionResult>, ApiError> {
    let result = store
        .lock()
        .unwrap()
        .sell_actions(user_id, ticker_id, amount)?;

    Ok(Json(result))
}

async fn add_ticker(
    State(store): State<Arc<Mutex<Market>>>,
    AuthenticatedUser(user_id): AuthenticatedUser,
    contents: Json<AddTickerRequest>,
) -> Json<Ticker> {
    let result = store
        .lock()
        .unwrap()
        .add_ticker(user_id, &contents.name, &contents.description);

    Json(result)
}

pub fn create_app(market: Arc<Mutex<Market>>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/tickers", get(get_tickers_endpoint))
        .route("/ticker/{id}", get(get_ticker_endpoint))
        .route("/users", get(get_users_endpoint))
        .route("/user/{id}", get(get_user_endpoint))
        .route("/buy/{ticker_id}/{amount}", post(buy_actions_endpoint))
        .route("/sell/{ticker_id}/{amount}", post(sell_actions_endpoint))
        .route("/add_ticker", post(add_ticker))
        .with_state(market)
}
