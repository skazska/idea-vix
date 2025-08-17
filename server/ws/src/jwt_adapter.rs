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
        ).map_err(|_e| {
            // Map all validation/decoding errors to Unauthorized for protected endpoints
            ModelError::Unauthorized("Invalid or expired token".to_string())
        })?;

        Ok(decoded.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use jsonwebtoken::{Algorithm, Validation};

    // Test payload structure
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestClaims {
        sub: String,
        exp: u64,
        data: String,
    }

    // Helper function to create test JWT adapter
    fn create_test_jwt_adapter() -> JwtAdapter {
        JwtAdapter::new("test_secret_key_123", 3600)
    }

    // Helper function to create test claims
    fn create_test_claims() -> TestClaims {
        TestClaims {
            sub: "test_user".to_string(),
            exp: 9999999999, // Far future expiration
            data: "test_data".to_string(),
        }
    }

    // Helper function to create validation for testing
    fn create_test_validation() -> Validation {
        Validation::new(Algorithm::HS256)
    }

    // Helper function to create custom test claims with specific values
    fn create_custom_claims(sub: &str, exp: u64, data: &str) -> TestClaims {
        TestClaims {
            sub: sub.to_string(),
            exp,
            data: data.to_string(),
        }
    }

    // Helper function to assert that token decoding fails with Unauthorized error
    fn assert_decode_fails_with_unauthorized(
        adapter: &JwtAdapter,
        token: &str,
        validation: &Validation,
        expected_msg: Option<&str>,
    ) {
        let result: Result<TestClaims, ModelError> = adapter.decode_token(token, validation);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            ModelError::Unauthorized(msg) => {
                if let Some(expected) = expected_msg {
                    assert_eq!(msg, expected);
                }
            }
            _ => panic!("Expected Unauthorized error"),
        }
    }

    // Helper function to test round-trip encode/decode success
    fn assert_round_trip_success(adapter: &JwtAdapter, claims: &TestClaims, validation: &Validation) {
        let token = adapter.generate_token(claims).unwrap();
        let decoded_claims: TestClaims = adapter.decode_token(&token, validation).unwrap();
        assert_eq!(decoded_claims, *claims);
    }

    // Helper function to test multiple tokens that should all fail with unauthorized errors
    fn test_tokens_for_unauthorized_errors(
        adapter: &JwtAdapter,
        validation: &Validation,
        tokens: Vec<&str>,
        expected_msg: Option<&str>,
    ) {
        for token in tokens {
            assert_decode_fails_with_unauthorized(adapter, token, validation, expected_msg);
        }
    }

    #[test]
    fn test_new_creates_jwt_adapter_with_correct_config() {
        let secret = "my_secret_key";
        let exp_secs = 7200;
        
        let adapter = JwtAdapter::new(secret, exp_secs);
        
        assert_eq!(adapter.exp_secs, exp_secs);
        // Note: encoding_key, decoding_key, and header are private fields
        // We can only verify behavior through public methods
    }

    #[test]
    fn test_new_with_different_secrets_creates_different_adapters() {
        let adapter1 = JwtAdapter::new("secret1", 3600);
        let adapter2 = JwtAdapter::new("secret2", 3600);
        
        let claims = create_test_claims();
        
        // Tokens generated with different secrets should be different
        let token1 = adapter1.generate_token(&claims).unwrap();
        let token2 = adapter2.generate_token(&claims).unwrap();
        
        assert_ne!(token1, token2);
    }

    #[test]
    fn test_generate_token_success() {
        let adapter = create_test_jwt_adapter();
        let claims = create_test_claims();
        
        let result = adapter.generate_token(&claims);
        
        assert!(result.is_ok());
        let token = result.unwrap();
        assert!(!token.is_empty());
        // JWT tokens have 3 parts separated by dots
        assert_eq!(token.matches('.').count(), 2);
    }

    #[test]
    fn test_generate_token_with_different_data_produces_different_tokens() {
        let adapter = create_test_jwt_adapter();
        
        let claims1 = create_custom_claims("user1", 9999999999, "data1");
        let claims2 = create_custom_claims("user2", 9999999999, "data2");
        
        let token1 = adapter.generate_token(&claims1).unwrap();
        let token2 = adapter.generate_token(&claims2).unwrap();
        
        assert_ne!(token1, token2);
    }

    #[test]
    fn test_generate_token_with_minimal_serializable_struct() {
        let adapter = create_test_jwt_adapter();
        
        #[derive(Serialize)]
        struct MinimalClaims {
            sub: String,
        }
        
        let claims = MinimalClaims {
            sub: "test".to_string(),
        };
        
        let result = adapter.generate_token(&claims);
        assert!(result.is_ok());
    }

    #[test]
    fn test_decode_token_success() {
        let adapter = create_test_jwt_adapter();
        let original_claims = create_test_claims();
        let validation = create_test_validation();
        
        assert_round_trip_success(&adapter, &original_claims, &validation);
    }

    #[test]
    fn test_decode_token_round_trip_preserves_data() {
        let adapter = create_test_jwt_adapter();
        let validation = create_test_validation();
        
        let original_claims = create_custom_claims(
            "round_trip_user",
            9999999999,
            "complex data with spaces and symbols !@#$%",
        );
        
        assert_round_trip_success(&adapter, &original_claims, &validation);
    }

    #[test]
    fn test_decode_token_with_invalid_token_returns_unauthorized() {
        let adapter = create_test_jwt_adapter();
        let validation = create_test_validation();
        
        assert_decode_fails_with_unauthorized(
            &adapter,
            "invalid.jwt.token",
            &validation,
            Some("Invalid or expired token"),
        );
    }

    #[test]
    fn test_decode_token_with_empty_token_returns_unauthorized() {
        let adapter = create_test_jwt_adapter();
        let validation = create_test_validation();
        
        assert_decode_fails_with_unauthorized(&adapter, "", &validation, None);
    }

    #[test]
    fn test_decode_token_with_malformed_token_returns_unauthorized() {
        let adapter = create_test_jwt_adapter();
        let validation = create_test_validation();
        
        let malformed_tokens = vec![
            "not.a.jwt",
            "header.payload", // Missing signature
            "too.many.parts.here.invalid",
            "header.payload.signature.extra",
        ];
        
        test_tokens_for_unauthorized_errors(&adapter, &validation, malformed_tokens, None);
    }

    #[test]
    fn test_decode_token_with_wrong_secret_returns_unauthorized() {
        let adapter1 = JwtAdapter::new("secret1", 3600);
        let adapter2 = JwtAdapter::new("secret2", 3600);
        let validation = create_test_validation();
        let claims = create_test_claims();
        
        // Generate token with adapter1
        let token = adapter1.generate_token(&claims).unwrap();
        
        // Try to decode with adapter2 (different secret)
        assert_decode_fails_with_unauthorized(&adapter2, &token, &validation, None);
    }

    #[test]
    fn test_decode_token_with_different_algorithm_validation_fails() {
        let adapter = create_test_jwt_adapter();
        let claims = create_test_claims();
        
        // Generate token (uses HS256)
        let token = adapter.generate_token(&claims).unwrap();
        
        // Try to validate with different algorithm
        let mut validation = Validation::new(Algorithm::HS512);
        validation.validate_exp = false; // Disable exp validation for this test
        
        assert_decode_fails_with_unauthorized(&adapter, &token, &validation, None);
    }

    #[test]
    fn test_decode_token_maps_all_jwt_errors_to_unauthorized() {
        let adapter = create_test_jwt_adapter();
        let validation = create_test_validation();
        
        // Test various types of invalid tokens that would cause different JWT errors
        let problematic_tokens = vec![
            "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.invalid_payload.signature", // Invalid base64 payload
            "invalid_header.eyJzdWIiOiJ0ZXN0In0.signature", // Invalid base64 header
        ];
        
        test_tokens_for_unauthorized_errors(
            &adapter,
            &validation,
            problematic_tokens,
            Some("Invalid or expired token"),
        );
    }

    #[test]
    fn test_jwt_adapter_consistent_behavior_across_instances() {
        let secret = "consistent_secret";
        let exp_secs = 1800;
        
        let adapter1 = JwtAdapter::new(secret, exp_secs);
        let adapter2 = JwtAdapter::new(secret, exp_secs);
        let validation = create_test_validation();
        let claims = create_test_claims();
        
        // Token generated by adapter1 should be decodable by adapter2 (same secret)
        let token = adapter1.generate_token(&claims).unwrap();
        let decoded: TestClaims = adapter2.decode_token(&token, &validation).unwrap();
        
        assert_eq!(decoded, claims);
    }
}

