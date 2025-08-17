use axum::http::StatusCode;

mod test_app;
mod helpers;

#[tokio::test]
async fn boards_crud_ok() {
    let app = test_app::TestApp::new().await;

    // get home page to ensure server is running
    let resp = helpers::get(&app.router, "/", None).await;
    assert_eq!(resp.status(), StatusCode::OK);
}
