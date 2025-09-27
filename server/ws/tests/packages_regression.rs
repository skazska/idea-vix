use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::{self, Body};
use axum::http::StatusCode;
use serde::Deserialize;
use serde_json::json;

mod test_app;
mod helpers;

#[derive(Deserialize, Debug, PartialEq)]
struct PackageResp {
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
async fn package_slug_conflict_returns_conflict() {
    let app = test_app::TestApp::new().await;
    let owner_cookie = helpers::auth_cookie_for(&app.router, "owner@example.com").await;
    let slug = unique_slug("pkg-conflict");

    let create_payload = json!({
        "name": "First Package",
        "slug": slug.clone(),
        "description": "primary"
    });
    let resp = helpers::post_json(&app.router, "/api/package", &create_payload.to_string(), Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let (_status, created): (StatusCode, PackageResp) = helpers::read_json(resp).await;
    assert_eq!(created.slug, slug);

    let conflict_payload = json!({
        "name": "Second Package",
        "slug": created.slug,
        "description": "duplicate"
    });
    let resp = helpers::post_json(&app.router, "/api/package", &conflict_payload.to_string(), Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::CONFLICT);
    let body = body_text(resp).await;
    assert!(body.contains("Unique"), "expected uniqueness error: {body}");
}

#[tokio::test]
async fn package_update_forbidden_for_view_role() {
    let app = test_app::TestApp::new().await;
    let owner_cookie = helpers::auth_cookie_for(&app.router, "owner@example.com").await;

    let create_payload = json!({
        "name": "Private Package",
        "description": "initial",
        "is_public": false
    });
    let resp = helpers::post_json(&app.router, "/api/package", &create_payload.to_string(), Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let (_status, created): (StatusCode, PackageResp) = helpers::read_json(resp).await;

    let grant_payload = json!({
        "address": "viewer@example.com",
        "role": "view"
    });
    let resp = helpers::post_json(&app.router, &format!("/api/package/{}/access", created.id), &grant_payload.to_string(), Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let viewer_cookie = helpers::auth_cookie_for(&app.router, "viewer@example.com").await;
    let resp = helpers::put_json(
        &app.router,
        &format!("/api/package/{}", created.id),
        r#"{"name":"Unauthorized"}"#,
        Some(&viewer_cookie),
    )
    .await;

    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    let body = body_text(resp).await;
    assert!(body.contains("not allowed"), "unexpected body: {body}");
}

#[tokio::test]
async fn package_delete_requires_owner_role() {
    let app = test_app::TestApp::new().await;
    let owner_cookie = helpers::auth_cookie_for(&app.router, "owner@example.com").await;

    let create_payload = json!({
        "name": "Managed Package",
        "is_public": false
    });
    let resp = helpers::post_json(&app.router, "/api/package", &create_payload.to_string(), Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let (_status, created): (StatusCode, PackageResp) = helpers::read_json(resp).await;

    let grant_payload = json!({
        "address": "manager@example.com",
        "role": "manage"
    });
    let resp = helpers::post_json(&app.router, &format!("/api/package/{}/access", created.id), &grant_payload.to_string(), Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let manager_cookie = helpers::auth_cookie_for(&app.router, "manager@example.com").await;
    let resp = helpers::delete(&app.router, &format!("/api/package/{}", created.id), Some(&manager_cookie)).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    let body = body_text(resp).await;
    assert!(body.contains("not allowed"), "unexpected body: {body}");

    let resp = helpers::delete(&app.router, &format!("/api/package/{}", created.id), Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn package_update_clears_optional_fields() {
    let app = test_app::TestApp::new().await;
    let owner_cookie = helpers::auth_cookie_for(&app.router, "owner@example.com").await;

    let create_payload = json!({
        "name": "Package With Details",
        "description": "keep me",
        "icon": "📦",
        "is_public": false
    });
    let resp = helpers::post_json(&app.router, "/api/package", &create_payload.to_string(), Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let (_status, created): (StatusCode, PackageResp) = helpers::read_json(resp).await;
    assert_eq!(created.description, Some("keep me".to_string()));
    assert_eq!(created.icon, Some("📦".to_string()));

    let update_payload = json!({
        "description": null,
        "icon": null,
        "is_public": true
    });
    let resp = helpers::put_json(
        &app.router,
        &format!("/api/package/{}", created.id),
        &update_payload.to_string(),
        Some(&owner_cookie),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, updated): (StatusCode, PackageResp) = helpers::read_json(resp).await;
    assert_eq!(updated.description, None);
    assert_eq!(updated.icon, None);
    assert!(updated.is_public);
}

#[tokio::test]
async fn private_package_hidden_from_unauthenticated_list() {
    let app = test_app::TestApp::new().await;
    let owner_cookie = helpers::auth_cookie_for(&app.router, "owner@example.com").await;

    let create_payload = json!({
        "name": "Private Package",
        "is_public": false
    });
    let resp = helpers::post_json(&app.router, "/api/package", &create_payload.to_string(), Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let (_status, created): (StatusCode, PackageResp) = helpers::read_json(resp).await;

    let resp = helpers::get(&app.router, "/api/package", Some(&owner_cookie)).await;
    let (_status, authed_list): (StatusCode, Vec<PackageResp>) = helpers::read_json(resp).await;
    assert!(authed_list.iter().any(|pkg| pkg.id == created.id));

    let resp = helpers::get(&app.router, "/api/package", None).await;
    let (_status, unauth_list): (StatusCode, Vec<PackageResp>) = helpers::read_json(resp).await;
    assert!(unauth_list.is_empty(), "expected no private packages, got: {unauth_list:?}");
}
