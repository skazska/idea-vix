use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::Serialize;

use crate::{error::ModelError};

/// JWT adapter for handling JSON Web Tokens
/// Provides configured params
/// Provides methods to generate JWT tokens
/// Holds secret to sign tokens
pub struct JwtAdapter {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    header: Header,
    pub exp_secs: u64,
}

impl JwtAdapter {
    pub fn new(secret: &str, exp_secs: u64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            header: Header::new(Algorithm::HS256),
            exp_secs,
        }
    }
    
    pub fn generate_token<T: Serialize>(&self, data: &T) -> Result<String, ModelError> {
        let token = encode(
            &self.header,
            data,
            &self.encoding_key
        ).map_err(|e| ModelError::Unexpected(format!("Failed to generate JWT token: {}", e)))?;

        Ok(token)
    }

    pub fn decode_token<T: serde::de::DeserializeOwned>(&self, token: &str, validation: &Validation) -> Result<T, ModelError> {
        let decoded = jsonwebtoken::decode::<T>(
            token,
            &self.decoding_key,
            validation
        ).map_err(|e| {
            eprintln!("Failed to decode JWT token: {:?}", e);
            ModelError::Unexpected(format!("Failed to decode JWT token"))
        })?;

        Ok(decoded.claims)
    }
}

