use axum::{body::Body, http::{Request, StatusCode}};
use tower::ServiceExt;

mod test_app;

#[tokio::test]
async fn packages_crud_ok() {
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

    // create package (body shape may differ; using minimal fields)
    let req = Request::post("/api/package")
        .header("content-type", "application/json")
        .header("cookie", &cookie_hdr)
        .body(Body::from(r#"{"name":"Demo Package","description":"desc"}"#))
        .unwrap();
    let resp = app.router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    // list packages
    let req = Request::get("/api/package")
        .body(Body::empty())
        .unwrap();
    let resp = app.router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}
