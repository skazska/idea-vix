use axum::{extract::FromRequestParts, http::{header, request::Parts, HeaderMap}};
use serde::{Deserialize, Deserializer};

use crate::error::ModelError;

/// Deserialize an optional value from the request body.
/// Any value that is present is considered Some value, including null.
/// to be used for struct field annotation for serde deserialization
pub fn deserialize_some<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where T: Deserialize<'de>,
          D: Deserializer<'de>
{
    Deserialize::deserialize(deserializer).map(Some)
}

/// Extract cookies from the request headers
fn extract_cookies_from_headers(headers: &HeaderMap) -> Option<Vec<String>> {
    headers.get(header::COOKIE).and_then(|cookie_header| {
        let cookie_str = cookie_header.to_str().ok()?;
        Some(cookie_str.split(';').map(|s| s.trim().to_string()).collect())
    })
}

/// Extract JWT token from the request cookies
fn extract_token_from_cookies(cookies: &Vec<String>) -> Option<String> {
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
fn extract_token_from_header(headers: &HeaderMap) -> Option<String> {
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

    /// Extract the JWT token from the request parts
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    // Test struct that mirrors the actual usage pattern in PatchPackageItem
    #[derive(Deserialize, Debug, PartialEq)]
    struct TestPatchStruct {
        pub name: Option<String>,
        #[serde(default, deserialize_with = "deserialize_some")]
        pub description: Option<Option<String>>,
        #[serde(default, deserialize_with = "deserialize_some")]
        pub icon: Option<Option<String>>,
    }

    fn deserialize_some_probe(json: &str, expected: TestPatchStruct) -> () {
        let result: TestPatchStruct = serde_json::from_str(json).unwrap();
        assert_eq!(result, expected)
    }

    #[test]
    fn deserialize_some_missing_field() {
        deserialize_some_probe(
            r#"{"name": "test"}"#,
            TestPatchStruct {
                name: Some("test".to_string()),
                description: None,
                icon: None,
            },
        );
    }

    #[test]
    fn deserialize_some_null_field() {
        deserialize_some_probe(
            r#"{"name": "test", "description": null, "icon": null}"#,
            TestPatchStruct {
                name: Some("test".to_string()),
                description: Some(None),
                icon: Some(None),
            },
        );
    }

    #[test]
    fn deserialize_some_with_value() {
        deserialize_some_probe(
            r#"{"name": "test", "description": "A description", "icon": "icon.png"}"#,
            TestPatchStruct {
                name: Some("test".to_string()),
                description: Some(Some("A description".to_string())),
                icon: Some(Some("icon.png".to_string())),
            },
        );
    }

    #[test]
    fn deserialize_some_mixed_fields() {
        deserialize_some_probe(
            r#"{"name": "test", "description": null}"#,
            TestPatchStruct {
                name: Some("test".to_string()),
                description: Some(None),
                icon: None,
            },
        );
    }

    #[test]
    fn deserialize_some_empty_object() {
        deserialize_some_probe(
            r#"{}"#,
            TestPatchStruct {
                name: None,
                description: None,
                icon: None,
            },
        );
    }

    // --- AuthToken::from_request_parts tests ---

    async fn auth_token_probe(headers: HeaderMap, expected_token: &str) {
        let (mut parts, _) = axum::http::Request::builder()
            .method("GET")
            .uri("/")
            .body(())
            .unwrap()
            .into_parts();
        parts.headers = headers;

        let state = ();
        let result = AuthToken::from_request_parts(&mut parts, &state).await;
        assert_eq!(result.unwrap().0, expected_token);
    }

    fn build_headers(auth_header: Option<&str>, cookie_header: Option<&str>) -> HeaderMap {
        let mut headers = HeaderMap::new();
        
        if let Some(auth) = auth_header {
            headers.insert(axum::http::header::AUTHORIZATION, auth.parse().unwrap());
        }
        
        if let Some(cookie) = cookie_header {
            headers.insert(axum::http::header::COOKIE, cookie.parse().unwrap());
        }
        
        headers
    }

    #[tokio::test]
    async fn auth_token_from_authorization_header() {
        auth_token_probe(
            build_headers(Some("Bearer test_token_123"), None),
            "test_token_123"
        ).await;
    }

    #[tokio::test]
    async fn auth_token_from_cookie() {
        auth_token_probe(
            build_headers(None, Some("Authorization=Bearer cookie_token_456; other=value")),
            "cookie_token_456"
        ).await;
    }

    #[tokio::test]
    async fn auth_token_cookie_takes_precedence_over_header() {
        auth_token_probe(
            build_headers(Some("Bearer header_token"), Some("Authorization=Bearer cookie_token")),
            "cookie_token"
        ).await;
    }

    #[tokio::test]
    async fn auth_token_no_token_returns_empty() {
        auth_token_probe(build_headers(None, None), "").await;
    }

    #[tokio::test]
    async fn auth_token_invalid_authorization_header() {
        auth_token_probe(
            build_headers(Some("Basic invalid_format"), None),
            ""
        ).await;
    }

    #[tokio::test]
    async fn auth_token_invalid_cookie_format() {
        auth_token_probe(
            build_headers(None, Some("session=value; other=another")),
            ""
        ).await;
    }

    #[tokio::test]
    async fn auth_token_empty_cookie_header() {
        auth_token_probe(
            build_headers(None, Some("")),
            ""
        ).await;
    }
}
