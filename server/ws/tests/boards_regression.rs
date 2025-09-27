use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::{self, Body};
use axum::http::StatusCode;
use serde::Deserialize;
use serde_json::json;

mod test_app;
mod helpers;

#[derive(Deserialize, Debug, PartialEq)]
struct BoardResp {
    id: i32,
    name: String,
    slug: String,
    description: Option<String>,
    icon: Option<String>,
    is_public: bool,
}

async fn body_text(resp: axum::http::Response<Body>) -> String {
    let bytes = body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    String::from_utf8_lossy(&bytes).to_string()
}

fn unique_slug(prefix: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock went backwards")
        .as_nanos();
    format!("{prefix}-{nanos}")
}

#[tokio::test]
async fn board_create_requires_authentication() {
    let app = test_app::TestApp::new().await;

    let resp = helpers::post_json(
        &app.router,
        "/api/board",
        r#"{"name":"Unauth Board","description":"no auth"}"#,
        None,
    )
    .await;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    let body = body_text(resp).await;
    assert!(body.contains("No token"), "unexpected body: {body}");
}

#[tokio::test]
async fn board_create_rejects_short_name() {
    let app = test_app::TestApp::new().await;
    let owner_cookie = helpers::auth_cookie_for(&app.router, "owner@example.com").await;

    let resp = helpers::post_json(&app.router, "/api/board", r#"{"name":"ab"}"#, Some(&owner_cookie)).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let body = body_text(resp).await;
    assert!(body.contains("Input validation error"), "unexpected body: {body}");
    assert!(body.contains("name"), "expected field name in body: {body}");
}

#[tokio::test]
async fn board_slug_conflict_returns_conflict() {
    let app = test_app::TestApp::new().await;
    let owner_cookie = helpers::auth_cookie_for(&app.router, "owner@example.com").await;
    let slug = unique_slug("slug-conflict");

    let create_payload = json!({
        "name": "First Board",
        "slug": slug.clone(),
        "description": "primary"
    });

    let resp = helpers::post_json(&app.router, "/api/board", &create_payload.to_string(), Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let (_status, created): (StatusCode, BoardResp) = helpers::read_json(resp).await;
    assert_eq!(created.slug, slug);

    let conflict_payload = json!({
        "name": "Second Board",
        "slug": slug,
        "description": "duplicate"
    });
    let resp = helpers::post_json(&app.router, "/api/board", &conflict_payload.to_string(), Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::CONFLICT);
    let body = body_text(resp).await;
    assert!(body.contains("Unique"), "expected unique violation message, got: {body}");
}

#[tokio::test]
async fn board_update_requires_elevated_role() {
    let app = test_app::TestApp::new().await;
    let owner_cookie = helpers::auth_cookie_for(&app.router, "owner@example.com").await;

    let create_payload = json!({
        "name": "Private Board",
        "description": "initial",
        "is_public": false
    });
    let resp = helpers::post_json(&app.router, "/api/board", &create_payload.to_string(), Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let (_status, created): (StatusCode, BoardResp) = helpers::read_json(resp).await;

    let grant_payload = json!({
        "address": "viewer@example.com",
        "role": "view"
    });
    let resp = helpers::post_json(&app.router, &format!("/api/board/{}/access", created.id), &grant_payload.to_string(), Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let viewer_cookie = helpers::auth_cookie_for(&app.router, "viewer@example.com").await;
    let resp = helpers::put_json(
        &app.router,
        &format!("/api/board/{}", created.id),
        r#"{"name":"Hacked"}"#,
        Some(&viewer_cookie),
    )
    .await;

    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    let body = body_text(resp).await;
    assert!(body.contains("not allowed"), "unexpected body: {body}");
}

#[tokio::test]
async fn board_delete_allows_manage_role() {
    let app = test_app::TestApp::new().await;
    let owner_cookie = helpers::auth_cookie_for(&app.router, "owner@example.com").await;

    let create_payload = json!({
        "name": "Managed Board",
        "description": "to be deleted",
        "is_public": false
    });
    let resp = helpers::post_json(&app.router, "/api/board", &create_payload.to_string(), Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let (_status, created): (StatusCode, BoardResp) = helpers::read_json(resp).await;

    let grant_payload = json!({
        "address": "manager@example.com",
        "role": "manage"
    });
    let resp = helpers::post_json(&app.router, &format!("/api/board/{}/access", created.id), &grant_payload.to_string(), Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let manager_cookie = helpers::auth_cookie_for(&app.router, "manager@example.com").await;
    let resp = helpers::delete(&app.router, &format!("/api/board/{}", created.id), Some(&manager_cookie)).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let resp = helpers::get(&app.router, &format!("/api/board/{}", created.id), Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn duplicate_board_access_invite_is_conflict() {
    let app = test_app::TestApp::new().await;
    let owner_cookie = helpers::auth_cookie_for(&app.router, "owner@example.com").await;

    let create_payload = json!({
        "name": "Invite Board",
        "is_public": false
    });
    let resp = helpers::post_json(&app.router, "/api/board", &create_payload.to_string(), Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let (_status, created): (StatusCode, BoardResp) = helpers::read_json(resp).await;

    let grant_payload = json!({
        "address": "guest@example.com",
        "role": "view"
    });
    let resp = helpers::post_json(&app.router, &format!("/api/board/{}/access", created.id), &grant_payload.to_string(), Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let resp = helpers::post_json(&app.router, &format!("/api/board/{}/access", created.id), &grant_payload.to_string(), Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::CONFLICT);
    let body = body_text(resp).await;
    assert!(body.contains("Unique"), "expected uniqueness error: {body}");
}
