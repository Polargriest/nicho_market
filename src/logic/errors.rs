use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    Unauthorized,
    Forbidden,
    BadRequest,
    InternalError,
    NotAffordable,
    NotEnoughActions,
    UserNotFound(i32),
    TickerNotFound(i32),
    InvalidInviteCode(String),
    UsernameAlreadyTaken(String),
    UnregisteredUser,
    WrongPassword,
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
            ApiError::InvalidInviteCode(code) => (
                StatusCode::BAD_REQUEST,
                format!("the invitation code '{code}' is invalid"),
            ),
            ApiError::UsernameAlreadyTaken(name) => (
                StatusCode::BAD_REQUEST,
                format!("username '{name}' was already taken"),
            ),
            ApiError::UnregisteredUser => {
                (StatusCode::BAD_REQUEST, format!("username does not exists"))
            }
            ApiError::WrongPassword => {
                (StatusCode::BAD_REQUEST, format!("wrong password for user"))
            }
            ApiError::BadRequest => (
                StatusCode::BAD_REQUEST,
                format!("the request has not the correct format"),
            ),
            ApiError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("something weird happened in our end"),
            ),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status_code, body).into_response()
    }
}
