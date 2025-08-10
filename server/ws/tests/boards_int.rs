use axum::{body::Body, http::{Request, StatusCode}};
use tower::ServiceExt;

mod test_app;

#[tokio::test]
async fn boards_crud_ok() {
    let app = test_app::TestApp::new().await;

    // signin + verify to get cookie
    let req = Request::post("/api/session/signin")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"address":"user@example.com"}"#))
        .unwrap();
    let _ = app.router.clone().oneshot(req).await.unwrap();

    let req = Request::post("/api/session/verify")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"address":"user@example.com", "code":"some_code"}"#))
        .unwrap();
    let resp = app.router.clone().oneshot(req).await.unwrap();
    let cookie_hdr = resp.headers().get("set-cookie").unwrap().to_str().unwrap().to_string();

    // create board
    let req = Request::post("/api/board")
        .header("content-type", "application/json")
        .header("cookie", &cookie_hdr)
        .body(Body::from(r#"{"name":"Demo Board","description":"desc","is_public":true}"#))
        .unwrap();
    let resp = app.router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    // list boards (no auth) should include public
    let req = Request::get("/api/board")
        .body(Body::empty())
        .unwrap();
    let resp = app.router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}
