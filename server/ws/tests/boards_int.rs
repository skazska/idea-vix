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

#[tokio::test]
async fn board_access_invite_and_permissions() {
    let app = test_app::TestApp::new().await;

    // Owner session
    let owner_cookie = helpers::auth_cookie_for(&app.router, "owner@example.com").await;

    // Create a private board
    let resp = helpers::post_json(&app.router, "/api/board", r#"{"name":"Secret Board","description":"hidden","is_public":false}"#, Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    #[derive(serde::Deserialize)]
    struct BoardResp { id: i32, name: String }
    let (_status, created): (StatusCode, BoardResp) = helpers::read_json(resp).await;
    let board_id = created.id;

    // Unauthenticated cannot access
    let resp = helpers::get(&app.router, &format!("/api/board/{}", board_id), None).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    // Grant view to guest
    let resp = helpers::post_json(&app.router, &format!("/api/board/{}/access", board_id), r#"{"address":"guest@example.com","role":"view"}"#, Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Guest session
    let guest_cookie = helpers::auth_cookie_for(&app.router, "guest@example.com").await;

    // Owner can list access and should see owner + guest mapping
    let resp = helpers::get(&app.router, &format!("/api/board/{}/access", board_id), Some(&owner_cookie)).await;
    #[derive(serde::Deserialize)]
    struct AccessMapping { board_id: i32, address: String, role: String }
    let (_status, list): (StatusCode, Vec<AccessMapping>) = helpers::read_json(resp).await;
    assert!(list.iter().any(|m| m.address == "owner@example.com" && m.role == "owner"));
    assert!(list.iter().any(|m| m.address == "guest@example.com" && m.role == "view"));

    // Guest cannot list access (owner-only)
    let resp = helpers::get(&app.router, &format!("/api/board/{}/access", board_id), Some(&guest_cookie)).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    // Unauthenticated must be 401
    let resp = helpers::get(&app.router, &format!("/api/board/{}/access", board_id), None).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    // Guest can GET the board now
    let resp = helpers::get(&app.router, &format!("/api/board/{}", board_id), Some(&guest_cookie)).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Guest cannot update with view
    let resp = helpers::put_json(&app.router, &format!("/api/board/{}", board_id), r#"{"name":"New name"}"#, Some(&guest_cookie)).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    // Owner revokes then grants manage
    let resp = helpers::delete(&app.router, &format!("/api/board/{}/access/{}", board_id, "guest@example.com"), Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let resp = helpers::post_json(
        &app.router,
        &format!("/api/board/{}/access", board_id),
        r#"{"address":"guest@example.com","role":"manage"}"#,
        Some(&owner_cookie)
    ).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Guest can update now
    let resp = helpers::put_json(&app.router, &format!("/api/board/{}", board_id), r#"{"name":"Managed name"}"#, Some(&guest_cookie)).await;
    assert_eq!(resp.status(), StatusCode::OK);
}
