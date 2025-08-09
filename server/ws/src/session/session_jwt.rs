use std::{sync::Arc};

use jsonwebtoken::Algorithm;
use serde::{Deserialize, Serialize};

use crate::{error::ModelError, jwt_adapter::JwtAdapter, session::session_service::{Session, SessionData}};

/// Session data to be encoded in the JWT
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionJWTData {
    pub aud: String,
    pub sub: String,
    pub exp: u64, // Expiration time in seconds since epoch
    pub iat: u64, // Issued at time in seconds since epoch
}


/// Session JWT service
pub struct SessionJWTService {
    pub jwt_adapter: JwtAdapter,
}

impl SessionJWTService {
    pub fn new(jwt_adapter: JwtAdapter) -> Self {
        Self { jwt_adapter }
    }

    /// Generates a JWT token for the given session data
    pub fn generate_token(&self, session: Session) -> Result<(SessionData, String), ModelError> {

        let data = SessionJWTData::new(&session);

        println!("Token for session: {:?}", data);

        // Generate JWT token
        let token = self.jwt_adapter.generate_token(&data)?;

        Ok((session.into(), token))
    }

    /// returns session data from token
    /// If token is not valid, returns Error
    pub fn get_session_data(&self, token: &str) -> Result<SessionData, ModelError> {
        let mut validation = jsonwebtoken::Validation::new(Algorithm::HS256);
        validation.set_audience(&["app".to_string()]);

        let session = self.jwt_adapter.decode_token::<SessionJWTData>(token, &validation)?;

        Ok(session.into())
    }

    /// returns optional session data from token
    /// If token is not valid or expired, returns None
    pub fn get_optional_session_data(&self, token: &str) -> Result<Option<SessionData>, ModelError> {
        if token.is_empty() {
            return Ok(None);
        }

        let result = self.get_session_data(token).map(|data| Some(data))?;

        Ok(result)
    }

}

// /// Middleware to validate JWT token
// pub async fn jwt(
//     jwt_adapter: &JwtAdapter,
//     token: &str,
// ) -> Result<SessionJWTData, ModelError> {
//     if token.is_empty() {
//         return Err(ModelError::Unauthorized("No token provided".to_string()));
//     }

//     // Validate the token
//     let session = jwt_adapter.decode_token::<SessionJWTData>(&token).or_else(|err| {
//         Err(ModelError::Unauthorized(format!("Invalid token: {}", err)))
//     })?;

//     if session.exp > to_unix_timestamp(SystemTime::now()) {
//         return Err(ModelError::Unauthorized("Token has expired".to_string()));
//     }

//     Ok(session)
// }

// /// Optional JWT auth middleware that doesn't require a token but adds claims if present
// pub async fn optional_jwt(
//     jwt_adapter: &JwtAdapter,
//     token: &str,
// ) -> Option<SessionJWTData> {
//     let jwt_token = jwt_adapter.decode_token::<SessionJWTData>(token).ok()?;

//     if jwt_token.exp > to_unix_timestamp(SystemTime::now()) {
//         return None;
//     }

//     Some(jwt_token)
// }

