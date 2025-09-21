use axum::{
    body::{self, Body},
    http::{HeaderMap, Request, StatusCode, Response},
    Router,
};
use serde::de::DeserializeOwned;
use tower::ServiceExt;

/// POST JSON with optional Cookie header and return the response
pub async fn post_json(router: &Router, path: &str, json: &str, cookie: Option<&str>) -> Response<Body> {
    let mut req = Request::post(path)
        .header("content-type", "application/json")
        .body(Body::from(json.to_string()))
        .unwrap();
    if let Some(cookie) = cookie {
        req.headers_mut().insert(axum::http::header::COOKIE, cookie.parse().unwrap());
    }
    router.clone().oneshot(req).await.unwrap()
}

/// PUT JSON with optional Cookie header and return the response
pub async fn put_json(router: &Router, path: &str, json: &str, cookie: Option<&str>) -> Response<Body> {
    let mut req = Request::put(path)
        .header("content-type", "application/json")
        .body(Body::from(json.to_string()))
        .unwrap();
    if let Some(cookie) = cookie {
        req.headers_mut().insert(axum::http::header::COOKIE, cookie.parse().unwrap());
    }
    router.clone().oneshot(req).await.unwrap()
}

/// GET with optional Cookie header and return the response
pub async fn get(router: &Router, path: &str, cookie: Option<&str>) -> Response<Body> {
    let mut req = Request::get(path)
        .body(Body::empty())
        .unwrap();
    if let Some(cookie) = cookie {
        req.headers_mut().insert(axum::http::header::COOKIE, cookie.parse().unwrap());
    }
    router.clone().oneshot(req).await.unwrap()
}

/// DELETE with optional Cookie header and return the response
pub async fn delete(router: &Router, path: &str, cookie: Option<&str>) -> Response<Body> {
    let mut req = Request::delete(path)
        .body(Body::empty())
        .unwrap();
    if let Some(cookie) = cookie {
        req.headers_mut().insert(axum::http::header::COOKIE, cookie.parse().unwrap());
    }
    router.clone().oneshot(req).await.unwrap()
}

/// Sign-in and verify for an address, returning a Cookie header value like
/// "Authorization=Bearer <token>" suitable for subsequent requests
pub async fn auth_cookie_for(router: &Router, address: &str) -> String {
    // signin
    let signin_body = serde_json::json!({ "address": address }).to_string();
    let _ = post_json(router, "/api/session/signin", &signin_body, None).await;

    // verify
    let verify_body = serde_json::json!({ "address": address, "code": "some_code" }).to_string();
    let resp = post_json(router, "/api/session/verify", &verify_body, None).await;

    cookie_from_response(&resp)
        .expect("set-cookie header with Authorization should be present")
}

/// Extract a Cookie header value (Authorization=Bearer ...) from a Set-Cookie response
pub fn cookie_from_response(resp: &Response<Body>) -> Option<String> {
    resp.headers()
        .get(axum::http::header::SET_COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|set_cookie| set_cookie.split(';').next().map(|s| s.to_string()))
}

/// Read response body into a type T (JSON)
pub async fn read_json<T: DeserializeOwned>(resp: Response<Body>) -> (StatusCode, T) {
    let status = resp.status();
    let bytes = body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let val: T = match serde_json::from_slice(&bytes) {
        Ok(v) => v,
        Err(e) => { 
            println!("JSON: {}", String::from_utf8_lossy(&bytes));
            panic!("Failed to parse JSON response: {}", e);
        },
    };
    (status, val)
}

/// Convenience: ensure status and return headers for further checks
pub fn assert_status<'a>(resp: &'a Response<Body>, expected: StatusCode) -> &'a HeaderMap {
    assert_eq!(resp.status(), expected);
    resp.headers()
}
