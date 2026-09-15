use axum::{
    Json, Router,
    extract::{FromRequestParts, Path, Request, State},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};

use crate::{
    DATABASE_PATH,
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
) -> Result<Json<Value>, ApiError> {
    let user = store
        .lock()
        .unwrap()
        .user_data_by_id(id)
        .ok_or(ApiError::UserNotFound(id))?;

    // esto es muuuy flojo. No quise hacer otra estructura PublicUser para esto. Perdón.
    let public_user = json!({
        "id": user.id,
        "name": user.name,
        "nichoCoins": user.nicho_coins,
        "portfolio": user.portfolio,
        "admin": user.admin,
    });

    Ok(Json(public_user))
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

async fn remove_ticker_endpoint(
    State(store): State<Arc<Mutex<Market>>>,
    Path(ticker_id): Path<i32>,
    AuthenticatedUser(user_id): AuthenticatedUser,
) -> Result<Json<Ticker>, ApiError> {
    let mut lock = store.lock().unwrap();

    let ticker_creator = lock
        .get_ticker_by_id(ticker_id)
        .ok_or(ApiError::TickerNotFound(ticker_id))?
        .author;

    if !(lock.is_user_admin(user_id) || ticker_creator == user_id) {
        return Err(ApiError::Forbidden);
    }

    let result = lock.remove_ticker(ticker_id)?;

    Ok(Json(result))
}

async fn remove_user_endpoint(
    State(store): State<Arc<Mutex<Market>>>,
    Path(user_id): Path<i32>,
    AuthenticatedUser(auth_user_id): AuthenticatedUser,
) -> Result<Json<User>, ApiError> {
    let mut lock = store.lock().unwrap();

    if !(lock.is_user_admin(auth_user_id) || user_id == auth_user_id) {
        return Err(ApiError::Forbidden);
    }

    let result = lock.remove_user(user_id)?;

    Ok(Json(result))
}

// this function was written by Claude. This function is called on every request, so every writting
// function can automatically save the market state.
async fn persist_after_writing(
    State(store): State<Arc<Mutex<Market>>>,
    request: Request,
    next: Next,
) -> Response {
    let response = next.run(request).await;
    store.lock().unwrap().save_market(DATABASE_PATH);
    response
}

pub fn create_app(market: Arc<Mutex<Market>>) -> Router {
    let read_config = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(1)
            .burst_size(30)
            .finish()
            .unwrap(),
    );

    let write_config = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(6)
            .burst_size(5)
            .finish()
            .unwrap(),
    );

    let read_router = Router::new()
        .route("/tickers", get(get_tickers_endpoint))
        .route("/ticker/{id}", get(get_ticker_endpoint))
        .route("/users", get(get_users_endpoint))
        .route("/user/{id}", get(get_user_endpoint))
        .layer(GovernorLayer::new(read_config));

    let write_router = Router::new()
        .route("/buy/{ticker_id}/{amount}", post(buy_actions_endpoint))
        .route("/sell/{ticker_id}/{amount}", post(sell_actions_endpoint))
        .route("/remove_ticker/{ticker_id}", post(remove_ticker_endpoint))
        .route("/remove_user/{user_id}", post(remove_user_endpoint))
        .route("/add_ticker", post(add_ticker))
        .layer(GovernorLayer::new(write_config))
        .layer(middleware::from_fn_with_state(
            market.clone(),
            persist_after_writing,
        ));

    Router::new()
        .route("/health", get(health_check))
        .merge(read_router)
        .merge(write_router)
        .with_state(market)
}
