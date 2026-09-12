use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    Unauthorized,
    Forbidden,
    NotAffordable,
    NotEnoughActions,
    UserNotFound(i32),
    TickerNotFound(i32),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, error_message) = match self {
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "user is not authentificated".to_string(),
            ),
            ApiError::Forbidden => (
                StatusCode::FORBIDDEN,
                "user does not have enough permissions".to_string(),
            ),
            ApiError::NotAffordable => (
                StatusCode::BAD_REQUEST,
                "user can't afford that amount of actions".to_string(),
            ),
            ApiError::NotEnoughActions => (
                StatusCode::BAD_REQUEST,
                "user does not has enough actions bought in ticker to sell".to_string(),
            ),
            ApiError::UserNotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("user with ID {id} not found"),
            ),
            ApiError::TickerNotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("ticker with ID {id} not found"),
            ),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status_code, body).into_response()
    }
}
