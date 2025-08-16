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
}
