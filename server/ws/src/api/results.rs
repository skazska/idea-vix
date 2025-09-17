use axum::{http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::error::ModelError;

/// Implements conversion from ModelError to a tuple of StatusCode and String for HTTP responses
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
            ModelError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            ModelError::Conflict(msg) => (StatusCode::CONFLICT, msg),
        }
    }
}

/// Implements conversion from ModelError to an Axum response
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    fn model_error_to_status_probe(error: ModelError, expected_status: StatusCode, expected_message: &str) {
        let (status, message): (StatusCode, String) = error.into();
        assert_eq!(status, expected_status);
        assert_eq!(message, expected_message);
    }

    fn model_error_to_response_probe(error: ModelError, expected_status: StatusCode) {
        let response = error.into_response();
        assert_eq!(response.status(), expected_status);
    }

    #[test]
    fn model_error_bad_request_conversion() {
        model_error_to_status_probe(
            ModelError::BadRequest("Invalid input".to_string()),
            StatusCode::BAD_REQUEST,
            "Invalid input"
        );
    }

    #[test]
    fn model_error_bad_response_conversion() {
        model_error_to_status_probe(
            ModelError::BadResponse("Upstream error".to_string()),
            StatusCode::BAD_GATEWAY,
            "Upstream error"
        );
    }

    #[test]
    fn model_error_unauthorized_conversion() {
        model_error_to_status_probe(
            ModelError::Unauthorized("Access denied".to_string()),
            StatusCode::UNAUTHORIZED,
            "Access denied"
        );
    }

    #[test]
    fn model_error_not_found_conversion() {
        model_error_to_status_probe(
            ModelError::NotFound("Resource not found".to_string()),
            StatusCode::NOT_FOUND,
            "Resource not found"
        );
    }

    #[test]
    fn model_error_logic_error_conversion() {
        model_error_to_status_probe(
            ModelError::LogicError("Business logic error".to_string()),
            StatusCode::INTERNAL_SERVER_ERROR,
            "Business logic error"
        );
    }

    #[test]
    fn model_error_unexpected_conversion() {
        model_error_to_status_probe(
            ModelError::Unexpected("Something went wrong".to_string()),
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong"
        );
    }

    #[test]
    fn model_error_timeout_conversion() {
        model_error_to_status_probe(
            ModelError::Timeout("Request timed out".to_string()),
            StatusCode::GATEWAY_TIMEOUT,
            "Request timed out"
        );
    }

    #[test]
    fn model_error_unavailable_conversion() {
        model_error_to_status_probe(
            ModelError::Unavailable("Service unavailable".to_string()),
            StatusCode::SERVICE_UNAVAILABLE,
            "Service unavailable"
        );
    }

    #[test]
    fn model_error_forbidden_conversion() {
        model_error_to_status_probe(
            ModelError::Forbidden("Permission denied".to_string()),
            StatusCode::FORBIDDEN,
            "Permission denied"
        );
    }

    // --- IntoResponse tests ---

    #[test]
    fn model_error_into_response_bad_request() {
        model_error_to_response_probe(
            ModelError::BadRequest("Test error".to_string()),
            StatusCode::BAD_REQUEST
        );
    }

    #[test]
    fn model_error_into_response_unauthorized() {
        model_error_to_response_probe(
            ModelError::Unauthorized("Test error".to_string()),
            StatusCode::UNAUTHORIZED
        );
    }

    #[test]
    fn model_error_into_response_forbidden() {
        model_error_to_response_probe(
            ModelError::Forbidden("Test error".to_string()),
            StatusCode::FORBIDDEN
        );
    }

    #[test]
    fn model_error_into_response_not_found() {
        model_error_to_response_probe(
            ModelError::NotFound("Test error".to_string()),
            StatusCode::NOT_FOUND
        );
    }

    #[test]
    fn model_error_into_response_internal_server_error() {
        model_error_to_response_probe(
            ModelError::LogicError("Test error".to_string()),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }
}