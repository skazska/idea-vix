use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::{db::to_unix_timestamp, error::ModelError, jwt_adapter::JwtAdapter};

/// Session data to be encoded in the JWT
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionJWTData {
    pub aud: String,
    pub sub: String,
    pub exp: u64, // Expiration time in seconds since epoch
    pub iat: u64, // Issued at time in seconds since epoch
}

/// Middleware to validate JWT token
pub async fn jwt(
    jwt_adapter: &JwtAdapter,
    token: &str,
) -> Result<SessionJWTData, ModelError> {
    if token.is_empty() {
        return Err(ModelError::Unauthorized("No token provided".to_string()));
    }

    // Validate the token
    let session = jwt_adapter.decode_token::<SessionJWTData>(&token).or_else(|err| {
        Err(ModelError::Unauthorized(format!("Invalid token: {}", err)))
    })?;

    if session.exp > to_unix_timestamp(SystemTime::now()) {
        return Err(ModelError::Unauthorized("Token has expired".to_string()));
    }

    Ok(session)
}

/// Optional JWT auth middleware that doesn't require a token but adds claims if present
pub async fn optional_jwt(
    jwt_adapter: &JwtAdapter,
    token: &str,
) -> Option<SessionJWTData> {
    let jwt_token = jwt_adapter.decode_token::<SessionJWTData>(token).ok()?;

    if jwt_token.exp > to_unix_timestamp(SystemTime::now()) {
        return None;
    }

    Some(jwt_token)
}

