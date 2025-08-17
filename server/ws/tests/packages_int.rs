use axum::http::StatusCode;

mod test_app;
mod helpers;
use serde::Deserialize;

#[derive(Deserialize)]
struct PackageResp { id: i32, name: String }

#[derive(Deserialize)]
struct AccessMapping { package_id: i32, address: String, role: String }

#[tokio::test]
async fn packages_crud_ok() {
    let app = test_app::TestApp::new().await;

    let cookie_hdr = helpers::auth_cookie_for(&app.router, "user@example.com").await;

    // create package (body shape may differ; using minimal fields)
    let resp = helpers::post_json(&app.router, "/api/package", r#"{"name":"Demo Package","description":"desc"}"#, Some(&cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    // list packages
    let resp = helpers::get(&app.router, "/api/package", None).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn package_access_invite_and_permissions() {
    let app = test_app::TestApp::new().await;

    // Owner session
    let owner_cookie = helpers::auth_cookie_for(&app.router, "owner@example.com").await;

    // Create a private package
    let resp = helpers::post_json(&app.router, "/api/package", r#"{"name":"Secret","description":"hidden","is_public":false}"#, Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let (_status, created): (StatusCode, PackageResp) = helpers::read_json(resp).await;
    let pkg_id = created.id;

    // Unauthenticated cannot access
    let resp = helpers::get(&app.router, &format!("/api/package/{}", pkg_id), None).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    // Grant view to guest
    let resp = helpers::post_json(&app.router, &format!("/api/package/{}/access", pkg_id), r#"{"address":"guest@example.com","role":"view"}"#, Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Guest session
    let guest_cookie = helpers::auth_cookie_for(&app.router, "guest@example.com").await;

    // Owner can list access and should see owner + guest mapping
    let resp = helpers::get(&app.router, &format!("/api/package/{}/access", pkg_id), Some(&owner_cookie)).await;
    let (_status, list): (StatusCode, Vec<AccessMapping>) = helpers::read_json(resp).await;
    assert!(list.iter().any(|m| m.address == "owner@example.com" && m.role == "owner"));
    assert!(list.iter().any(|m| m.address == "guest@example.com" && m.role == "view"));

    // Guest cannot list access (owner-only)
    let resp = helpers::get(&app.router, &format!("/api/package/{}/access", pkg_id), Some(&guest_cookie)).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    // Unauthenticated must be 401
    let resp = helpers::get(&app.router, &format!("/api/package/{}/access", pkg_id), None).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    // Guest can GET the package now
    let resp = helpers::get(&app.router, &format!("/api/package/{}", pkg_id), Some(&guest_cookie)).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Guest cannot update with view
    let resp = helpers::put_json(&app.router, &format!("/api/package/{}", pkg_id), r#"{"name":"New name"}"#, Some(&guest_cookie)).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    // Owner revokes then grants manage
    let resp = helpers::delete(&app.router, &format!("/api/package/{}/access/{}", pkg_id, "guest@example.com"), Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let resp = helpers::post_json(&app.router, &format!("/api/package/{}/access", pkg_id), r#"{"address":"guest@example.com","role":"manage"}"#, Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Guest can update now
    let resp = helpers::put_json(&app.router, &format!("/api/package/{}", pkg_id), r#"{"name":"Managed name"}"#, Some(&guest_cookie)).await;
    assert_eq!(resp.status(), StatusCode::OK);
}
