use axum::http::StatusCode;

mod test_app;
mod helpers;

#[tokio::test]
async fn boards_crud_ok() {
    let app = test_app::TestApp::new().await;

    let cookie_hdr = helpers::auth_cookie_for(&app.router, "user@example.com").await;

    // create board
    let resp = helpers::post_json(&app.router, "/api/board", r#"{"name":"Demo Board","description":"desc","is_public":true}"#, Some(&cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    // list boards (no auth) should include public
    let resp = helpers::get(&app.router, "/api/board", None).await;
    assert_eq!(resp.status(), StatusCode::OK);
}
