use axum::{extract::FromRequestParts, http::{header, request::Parts, HeaderMap}};
use serde::{Deserialize, Deserializer};

use crate::error::ModelError;

// Any value that is present is considered Some value, including null.
pub fn deserialize_some<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where T: Deserialize<'de>,
          D: Deserializer<'de>
{
    Deserialize::deserialize(deserializer).map(Some)
}

pub fn extract_cookies_from_headers(headers: &HeaderMap) -> Option<Vec<String>> {
    headers.get(header::COOKIE).and_then(|cookie_header| {
        let cookie_str = cookie_header.to_str().ok()?;
        Some(cookie_str.split(';').map(|s| s.trim().to_string()).collect())
    })
}

/// Extract JWT token from the request cookies
pub fn extract_token_from_cookies(cookies: &Vec<String>) -> Option<String> {
    cookies.iter().find_map(|cookie| {
        if cookie.starts_with("Authorization=Bearer ") {
            let result = cookie.strip_prefix("Authorization=Bearer ").unwrap_or_default();
            Some(result.to_string())
        } else {
            None
        }
    })
}

/// Extract JWT token from the authorization header
pub fn extract_token_from_header(headers: &HeaderMap) -> Option<String> {
    headers.get(header::AUTHORIZATION).and_then(|auth_header| {
        let auth_str = auth_header.to_str().ok()?;
        if auth_str.starts_with("Bearer ") {
                Some(auth_str.replace("Bearer ", ""))
        } else {
            None
        }
    })
}

#[derive(Deserialize, Debug)]
pub struct AuthToken(pub String);

impl<S: Send + Sync> FromRequestParts<S> for AuthToken {
    type Rejection = ModelError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;
        let token = extract_cookies_from_headers(headers)
            .and_then(|cookies| {
                if cookies.is_empty() {
                    None
                } else {
                    extract_token_from_cookies(&cookies)
                }
            })
            .or_else(|| extract_token_from_header(&headers));

        if let Some(token) = token {
            Ok(Self(token))
        } else {
            Ok(Self("".to_string())) // Return empty token if not found
        }
   }
}
