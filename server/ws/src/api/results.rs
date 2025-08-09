use axum::{http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::error::ModelError;

impl Into<(StatusCode, String)> for ModelError {
    fn into(self) -> (StatusCode, String) {
        match self {
            ModelError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ModelError::BadResponse(msg) => (StatusCode::BAD_GATEWAY, msg),
            ModelError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            ModelError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ModelError::LogicError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ModelError::Unexpected(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ModelError::Timeout(msg) => (StatusCode::GATEWAY_TIMEOUT, msg),
            ModelError::Unavailable(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg),
        }
    }
}

impl IntoResponse for ModelError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = self.into();
        (status, message).into_response()
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Success {
    pub success: bool,
}
