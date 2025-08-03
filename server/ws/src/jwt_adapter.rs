use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::Serialize;

use crate::{error::ModelError};

/// JWT adapter for handling JSON Web Tokens
/// Provides configured params
/// Provides methods to generate JWT tokens
/// Holds secret to sign tokens
pub struct JwtAdapter {
    secret: String,
    pub exp_secs: u64,
}

impl JwtAdapter {
    pub fn new(secret: &str, exp_secs: u64) -> Self {
        Self {
            secret: secret.to_owned(),
            exp_secs,
        }
    }

    pub fn generate_token<T: Serialize>(&self, data: &T) -> Result<String, ModelError> {
        let token = encode(
            &Header::new(Algorithm::HS256),
            data,
            &EncodingKey::from_secret(self.secret.as_bytes())
        ).map_err(|_| ModelError::Unexpected("Failed to generate JWT token".to_owned()))?;
        
        Ok(token)
    }
}

