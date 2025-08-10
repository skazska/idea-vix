use axum::{body::Body, http::{Request, StatusCode}};
use tower::ServiceExt;

mod test_app;

#[tokio::test]
async fn boards_crud_ok() {
    let app = test_app::TestApp::new().await;

    // get home page to ensure server is running
    let req = Request::get("/")
        .body(Body::empty())
        .unwrap();
    let resp = app.router.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}
