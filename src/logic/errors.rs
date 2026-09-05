use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    BuyError,
    UserNotFound(i32),
    TickerNotFound(i32),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, error_message) = match self {
            ApiError::BuyError => (
                StatusCode::BAD_REQUEST,
                "Error while buying actions".to_string(),
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
