use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    NotAffordable,
    UserNotFound(i32),
    TickerNotFound(i32),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, error_message) = match self {
            ApiError::NotAffordable => (
                StatusCode::BAD_REQUEST,
                "User can't afford that amount of actions".to_string(),
            ),
            ApiError::UserNotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("User with ID {id} not found."),
            ),
            ApiError::TickerNotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("Ticker with ID {id} not found."),
            ),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status_code, body).into_response()
    }
}
