/// Common modules for error handling in the application.

/// Service level CRUD errors
#[derive(Debug)]
pub enum ModelError {
    BadRequest(String),
    BadResponse(String),
    Unauthorized(String),
    NotFound(String),
    LogicError(String),
    Unexpected(String),
    Timeout(String),
    Unavailable(String),
    Forbidden(String),
}

impl std::fmt::Display for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            ModelError::BadResponse(msg) => write!(f, "Bad Response: {}", msg),
            ModelError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            ModelError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ModelError::LogicError(msg) => write!(f, "Logic Error: {}", msg),
            ModelError::Unexpected(msg) => write!(f, "Unexpected Error: {}", msg),
            ModelError::Timeout(msg) => write!(f, "Timeout Error: {}", msg),
            ModelError::Unavailable(msg) => write!(f, "Service Unavailable: {}", msg),
            ModelError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
        }
    }
    
}

impl From<sqlx::Error> for ModelError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => ModelError::NotFound("Row not found".to_string()),
            sqlx::Error::InvalidArgument(msg) => ModelError::BadRequest(msg),
            sqlx::Error::PoolTimedOut => ModelError::Timeout("Connection pool timed out".to_string()),
            sqlx::Error::Io(msg) => ModelError::Unavailable(msg.to_string()),
            _ => ModelError::Unexpected(err.to_string()),
        }
    }
}
