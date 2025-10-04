use assert_struct::assert_struct;
use axum::http::StatusCode;

mod test_app;
mod helpers;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
struct CommonCrudResp { id: i32, name: String, slug: String, description: Option<String>, icon: Option<String>, is_public: bool }

#[derive(Deserialize, Debug, PartialEq)]
struct AccessMapping { item_id: i32, address: String, role: String }


#[tokio::test]
async fn packages_crud_smoke_ok() {
    let app = test_app::TestApp::new().await;

    let owner_cookie_hdr = helpers::auth_cookie_for(&app.router, "user@example.com").await;

    // create first package (private, icon, no slug)
    let resp = helpers::post_json(&app.router, "/api/package", r#"{"name":"Demo Package 1","description":"desc", "icon":"📦"}"#, Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::CREATED, "create first package (private, icon, no slug)");
    let (_status, created): (StatusCode, CommonCrudResp) = helpers::read_json(resp).await;
    // assert_eq!(status, StatusCode::CREATED, "create first package (private, icon, no slug)");
    assert_struct!(created, CommonCrudResp { name: "Demo Package 1", slug: "demo-package-1", description: Some("desc"), icon: Some("📦"), is_public: false, .. });
    let pkg_1_id = created.id;

    // create second package (public, with slug)
    let resp = helpers::post_json(&app.router, "/api/package", r#"{"name":"Demo Package 2","description":"desc", "is_public":true, "slug":"d-p-2"}"#, Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let (_status, created): (StatusCode, CommonCrudResp) = helpers::read_json(resp).await;
    let pkg_2_id = created.id;

    // owner gets list of all packages
    let resp = helpers::get(&app.router, "/api/package", Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<CommonCrudResp>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 2);
    assert!(list.iter().any(|p| p.id == pkg_1_id));
    assert!(list.iter().any(|p| p.id == pkg_2_id));

    // unauthenticated gets only public package
    let resp = helpers::get(&app.router, "/api/package", None).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<CommonCrudResp>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, pkg_2_id);

    // fail to get first package by id unauthenticated (private)
    let resp = helpers::get(&app.router, &format!("/api/package/{}", pkg_1_id), None).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    // get second package by id unauthenticated (public)
    let resp = helpers::get(&app.router, &format!("/api/package/{}", pkg_2_id), None).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, got): (StatusCode, CommonCrudResp) = helpers::read_json(resp).await;
    assert_struct!(got, CommonCrudResp { id: pkg_2_id, name: "Demo Package 2", slug: "d-p-2", description: Some("desc"), icon: None, is_public: true });
    assert_eq!(got.is_public, true);

    // update first package by owner (with making it public)
    let resp = helpers::put_json(&app.router, &format!("/api/package/{}", pkg_1_id), r#"{"name":"Updated Name","description":"new desc","icon":"📦","is_public":true}"#, Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, updated): (StatusCode, CommonCrudResp) = helpers::read_json(resp).await;
    assert_struct!(updated, CommonCrudResp { id: pkg_1_id, name: "Updated Name", slug: "demo-package-1", description: Some("new desc"), icon: Some("📦"), is_public: true });

    // delete second package by owner
    let resp = helpers::delete(&app.router, &format!("/api/package/{}", pkg_2_id), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, deleted): (StatusCode, CommonCrudResp) = helpers::read_json(resp).await;
    assert_struct!(deleted, CommonCrudResp { id: pkg_2_id, name: "Demo Package 2", slug: "d-p-2", description: Some("desc"), icon: None, is_public: true });

    // get first package by id by owner
    let resp = helpers::get(&app.router, &format!("/api/package/{}", pkg_1_id), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, got): (StatusCode, CommonCrudResp) = helpers::read_json(resp).await;
    assert_eq!(got.id, pkg_1_id);

    // get second package by id by owner (deleted)
    let resp = helpers::get(&app.router, &format!("/api/package/{}", pkg_2_id), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    // unauthenticated gets first package (now public) by id
    let resp = helpers::get(&app.router, &format!("/api/package/{}", pkg_1_id), None).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, got): (StatusCode, CommonCrudResp) = helpers::read_json(resp).await;
    assert_eq!(got.id, pkg_1_id);
}

#[tokio::test]
async fn package_access_invite_and_permissions_cmoke() {
    let app = test_app::TestApp::new().await;

    // Owner session
    let owner_cookie = helpers::auth_cookie_for(&app.router, "owner@example.com").await;

    // Create a private package
    let resp = helpers::post_json(&app.router, "/api/package", r#"{"name":"Secret","description":"hidden","is_public":false}"#, Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let (_status, created): (StatusCode, CommonCrudResp) = helpers::read_json(resp).await;
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
 
    println!("Access list: {:?}", list);
 
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
