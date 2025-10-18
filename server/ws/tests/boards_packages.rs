use assert_struct::assert_struct;
use axum::http::StatusCode;

mod test_app;
mod helpers;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, PartialEq)]
struct Board { id: i32, name: String, slug: String, description: Option<String>, icon: Option<String>, is_public: bool }

#[derive(Deserialize, Debug, PartialEq)]
struct Package { id: i32, name: String, slug: String, description: Option<String>, icon: Option<String>, is_public: bool }

#[derive(Serialize, Debug)]
struct AddPackageRequest { package_id: i64 }

/// Helper to create a board and return ID and owner cookie
async fn create_board(app: &test_app::TestApp, name: &str, is_public: bool, owner_email: &str) -> (i32, String) {
    let owner_cookie = helpers::auth_cookie_for(&app.router, owner_email).await;
    let payload = format!(r#"{{"name":"{}","is_public":{}}}"#, name, is_public);
    let resp = helpers::post_json(&app.router, "/api/board", &payload, Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let (_status, created): (StatusCode, Board) = helpers::read_json(resp).await;
    (created.id, owner_cookie)
}

/// Helper to create a package and return ID and owner cookie
async fn create_package(app: &test_app::TestApp, name: &str, is_public: bool, owner_email: &str) -> (i32, String) {
    let owner_cookie = helpers::auth_cookie_for(&app.router, owner_email).await;
    let payload = format!(r#"{{"name":"{}","is_public":{}}}"#, name, is_public);
    let resp = helpers::post_json(&app.router, "/api/package", &payload, Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let (_status, created): (StatusCode, Package) = helpers::read_json(resp).await;
    (created.id, owner_cookie)
}

/// Helper to add a package to a board
async fn add_package_to_board(app: &test_app::TestApp, board_id: i32, package_id: i32, cookie: &str) -> axum::response::Response {
    let payload = format!(r#"{{"package_id":{}}}"#, package_id);
    helpers::post_json(&app.router, &format!("/api/board/{}/packages", board_id), &payload, Some(cookie)).await
}

#[tokio::test]
async fn test_list_board_packages_empty() {
    let app = test_app::TestApp::new().await;
    let (board_id, owner_cookie) = create_board(&app, "Empty Board", false, "owner@example.com").await;

    let resp = helpers::get(&app.router, &format!("/api/board/{}/packages", board_id), Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<Package>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 0);
}

#[tokio::test]
async fn test_add_package_to_board_ok() {
    let app = test_app::TestApp::new().await;
    let (board_id, board_cookie) = create_board(&app, "Test Board", false, "board_owner@example.com").await;
    let (package_id, _pkg_cookie) = create_package(&app, "Public Package", true, "pkg_owner@example.com").await;

    let resp = add_package_to_board(&app, board_id, package_id, &board_cookie).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<Package>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, package_id);
}

#[tokio::test]
async fn test_add_own_private_package() {
    let app = test_app::TestApp::new().await;
    let owner_cookie = helpers::auth_cookie_for(&app.router, "owner@example.com").await;

    // Create board
    let payload = r#"{"name":"My Board","is_public":false}"#;
    let resp = helpers::post_json(&app.router, "/api/board", payload, Some(&owner_cookie)).await;
    let (_status, board): (StatusCode, Board) = helpers::read_json(resp).await;

    // Create private package
    let payload = r#"{"name":"My Package","is_public":false}"#;
    let resp = helpers::post_json(&app.router, "/api/package", payload, Some(&owner_cookie)).await;
    let (_status, package): (StatusCode, Package) = helpers::read_json(resp).await;

    // Add package to board
    let resp = add_package_to_board(&app, board.id, package.id, &owner_cookie).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<Package>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, package.id);
}

#[tokio::test]
async fn test_list_board_packages_ok() {
    let app = test_app::TestApp::new().await;
    let (board_id, board_cookie) = create_board(&app, "Test Board", false, "owner@example.com").await;
    let (pkg1_id, _) = create_package(&app, "Package 1", true, "owner@example.com").await;
    let (pkg2_id, _) = create_package(&app, "Package 2", true, "owner@example.com").await;

    // Add both packages
    add_package_to_board(&app, board_id, pkg1_id, &board_cookie).await;
    add_package_to_board(&app, board_id, pkg2_id, &board_cookie).await;

    // List packages
    let resp = helpers::get(&app.router, &format!("/api/board/{}/packages", board_id), Some(&board_cookie)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<Package>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 2);
    assert!(list.iter().any(|p| p.id == pkg1_id));
    assert!(list.iter().any(|p| p.id == pkg2_id));
}

#[tokio::test]
async fn test_remove_package_ok() {
    let app = test_app::TestApp::new().await;
    let (board_id, board_cookie) = create_board(&app, "Test Board", false, "owner@example.com").await;
    let (package_id, _) = create_package(&app, "Package", true, "owner@example.com").await;

    // Add package
    add_package_to_board(&app, board_id, package_id, &board_cookie).await;

    // Remove package
    let resp = helpers::delete(&app.router, &format!("/api/board/{}/packages/{}", board_id, package_id), Some(&board_cookie)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<Package>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 0);
}

#[tokio::test]
async fn test_add_package_requires_auth() {
    let app = test_app::TestApp::new().await;
    let (board_id, _board_cookie) = create_board(&app, "Test Board", false, "owner@example.com").await;
    let (package_id, _) = create_package(&app, "Package", true, "owner@example.com").await;

    // Try to add package without auth
    let resp = add_package_to_board(&app, board_id, package_id, "").await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_add_package_requires_owner_role() {
    let app = test_app::TestApp::new().await;
    let (board_id, board_cookie) = create_board(&app, "Test Board", false, "owner@example.com").await;
    let (package_id, _) = create_package(&app, "Package", true, "owner@example.com").await;

    // Grant manage role to another user
    let guest_cookie = helpers::auth_cookie_for(&app.router, "guest@example.com").await;
    let payload = r#"{"address":"guest@example.com","role":"manage"}"#;
    let resp = helpers::post_json(&app.router, &format!("/api/board/{}/access", board_id), payload, Some(&board_cookie)).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Try to add package as guest (has manage role, not owner)
    let resp = add_package_to_board(&app, board_id, package_id, &guest_cookie).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_list_packages_requires_access() {
    let app = test_app::TestApp::new().await;
    let (board_id, _board_cookie) = create_board(&app, "Private Board", false, "owner@example.com").await;

    // Try to list packages without access
    let other_cookie = helpers::auth_cookie_for(&app.router, "other@example.com").await;
    let resp = helpers::get(&app.router, &format!("/api/board/{}/packages", board_id), Some(&other_cookie)).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_remove_package_requires_owner() {
    let app = test_app::TestApp::new().await;
    let (board_id, board_cookie) = create_board(&app, "Test Board", false, "owner@example.com").await;
    let (package_id, _) = create_package(&app, "Package", true, "owner@example.com").await;

    // Add package
    add_package_to_board(&app, board_id, package_id, &board_cookie).await;

    // Grant manage role to guest
    let guest_cookie = helpers::auth_cookie_for(&app.router, "guest@example.com").await;
    let payload = r#"{"address":"guest@example.com","role":"manage"}"#;
    helpers::post_json(&app.router, &format!("/api/board/{}/access", board_id), payload, Some(&board_cookie)).await;

    // Try to remove as guest
    let resp = helpers::delete(&app.router, &format!("/api/board/{}/packages/{}", board_id, package_id), Some(&guest_cookie)).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_add_package_invalid_package_id() {
    let app = test_app::TestApp::new().await;
    let (board_id, board_cookie) = create_board(&app, "Test Board", false, "owner@example.com").await;

    // Try with invalid package_id (0)
    let payload = r#"{"package_id":0}"#;
    let resp = helpers::post_json(&app.router, &format!("/api/board/{}/packages", board_id), payload, Some(&board_cookie)).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_add_package_board_not_found() {
    let app = test_app::TestApp::new().await;
    let (package_id, _) = create_package(&app, "Package", true, "owner@example.com").await;
    let owner_cookie = helpers::auth_cookie_for(&app.router, "owner@example.com").await;

    // Try with non-existent board
    // Note: Returns 403 Forbidden (not 404) because we check authorization first
    // This is correct security behavior - don't reveal if resource exists when no permission
    let resp = add_package_to_board(&app, 99999, package_id, &owner_cookie).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_add_package_package_not_found() {
    let app = test_app::TestApp::new().await;
    let (board_id, board_cookie) = create_board(&app, "Test Board", false, "owner@example.com").await;

    // Try with non-existent package
    let resp = add_package_to_board(&app, board_id, 99999, &board_cookie).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_add_package_not_accessible() {
    let app = test_app::TestApp::new().await;
    let (board_id, board_cookie) = create_board(&app, "Test Board", false, "board_owner@example.com").await;
    let (package_id, _) = create_package(&app, "Private Package", false, "pkg_owner@example.com").await;

    // Board owner tries to add someone else's private package
    let resp = add_package_to_board(&app, board_id, package_id, &board_cookie).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_add_package_duplicate() {
    let app = test_app::TestApp::new().await;
    let (board_id, board_cookie) = create_board(&app, "Test Board", false, "owner@example.com").await;
    let (package_id, _) = create_package(&app, "Package", true, "owner@example.com").await;

    // Add package first time
    let resp = add_package_to_board(&app, board_id, package_id, &board_cookie).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Try to add same package again
    let resp = add_package_to_board(&app, board_id, package_id, &board_cookie).await;
    assert_eq!(resp.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_remove_package_not_in_board() {
    let app = test_app::TestApp::new().await;
    let (board_id, board_cookie) = create_board(&app, "Test Board", false, "owner@example.com").await;
    let (package_id, _) = create_package(&app, "Package", true, "owner@example.com").await;

    // Remove package that was never added (should be idempotent)
    let resp = helpers::delete(&app.router, &format!("/api/board/{}/packages/{}", board_id, package_id), Some(&board_cookie)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<Package>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 0);
}

#[tokio::test]
async fn test_package_removal_doesnt_affect_other_boards() {
    let app = test_app::TestApp::new().await;
    let owner_cookie = helpers::auth_cookie_for(&app.router, "owner@example.com").await;

    // Create 2 boards
    let payload = r#"{"name":"Board A","is_public":false}"#;
    let resp = helpers::post_json(&app.router, "/api/board", payload, Some(&owner_cookie)).await;
    let (_status, board_a): (StatusCode, Board) = helpers::read_json(resp).await;

    let payload = r#"{"name":"Board B","is_public":false}"#;
    let resp = helpers::post_json(&app.router, "/api/board", payload, Some(&owner_cookie)).await;
    let (_status, board_b): (StatusCode, Board) = helpers::read_json(resp).await;

    // Create package
    let (package_id, _) = create_package(&app, "Shared Package", true, "owner@example.com").await;

    // Add package to both boards
    add_package_to_board(&app, board_a.id, package_id, &owner_cookie).await;
    add_package_to_board(&app, board_b.id, package_id, &owner_cookie).await;

    // Remove from board A
    let resp = helpers::delete(&app.router, &format!("/api/board/{}/packages/{}", board_a.id, package_id), Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Check board A has no packages
    let resp = helpers::get(&app.router, &format!("/api/board/{}/packages", board_a.id), Some(&owner_cookie)).await;
    let (_status, list): (StatusCode, Vec<Package>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 0);

    // Check board B still has the package
    let resp = helpers::get(&app.router, &format!("/api/board/{}/packages", board_b.id), Some(&owner_cookie)).await;
    let (_status, list): (StatusCode, Vec<Package>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, package_id);
}
