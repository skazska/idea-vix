/// Common modules for error handling in the application.

/// Service level CRUD errors
pub enum ModelError {
    BadRequest(String),
    BadResponse(String),
    Unauthorized(String),
    NotFound(String),
    LogicError(String),
    Unexpected(String),
    Timeout(String),
    Unavailable(String),
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
