use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub(crate) type Result<T> = std::result::Result<T, Response>;

pub(crate) fn error_response(msg: impl Into<String>) -> Response {
    let err_msg = msg.into();
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"error": err_msg})),
    )
        .into_response()
}
