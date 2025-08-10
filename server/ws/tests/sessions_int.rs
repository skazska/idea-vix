use axum::{body::Body, http::{Request, StatusCode}};
use tower::ServiceExt;

mod test_app;

#[tokio::test]
async fn sessions_flow_ok() {
    let app = test_app::TestApp::new().await;

    // signin
    let req = Request::post("/api/session/signin")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"address":"demo@example.com"}"#))
        .unwrap();
    let resp = app.router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // verify (stub code is "some_code")
    let req = Request::post("/api/session/verify")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"address":"demo@example.com", "code":"some_code"}"#))
        .unwrap();
    let resp = app.router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // extract Set-Cookie
    let cookie_hdr = resp.headers().get("set-cookie").unwrap().to_str().unwrap().to_string();
    assert!(cookie_hdr.contains("Authorization=Bearer "));

    // me
    let req = Request::get("/api/session/me")
        .header("cookie", cookie_hdr)
        .body(Body::empty())
        .unwrap();
    let resp = app.router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}
