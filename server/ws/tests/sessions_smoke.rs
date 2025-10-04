use axum::http::StatusCode;

mod test_app;
mod helpers;

#[tokio::test]
async fn sessions_flow_smoke_ok() {
    let app = test_app::TestApp::new().await;

    let cookie_hdr = helpers::auth_cookie_for(&app.router, "demo@example.com").await;

    let resp = helpers::get(&app.router, "/api/session/me", Some(&cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::OK);
}
