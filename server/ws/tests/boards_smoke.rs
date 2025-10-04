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
async fn boards_crud_smoke_ok() {
    let app = test_app::TestApp::new().await;

    let owner_cookie_hdr = helpers::auth_cookie_for(&app.router, "user@example.com").await;

    // create first board (private, icon, no slug)
    let resp = helpers::post_json(&app.router, "/api/board", r#"{"name":"Demo Board 1","description":"desc", "icon":"📋"}"#, Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::CREATED, "create first board (private, icon, no slug)");
    let (_status, created): (StatusCode, CommonCrudResp) = helpers::read_json(resp).await;
    assert_struct!(created, CommonCrudResp { name: "Demo Board 1", slug: "demo-board-1", description: Some("desc"), icon: Some("📋"), is_public: false, .. });
    let board_1_id = created.id;

    // create second board (public, with slug)
    let resp = helpers::post_json(&app.router, "/api/board", r#"{"name":"Demo Board 2","description":"desc", "is_public":true, "slug":"d-b-2"}"#, Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let (_status, created): (StatusCode, CommonCrudResp) = helpers::read_json(resp).await;
    let board_2_id = created.id;

    // owner gets list of all boards
    let resp = helpers::get(&app.router, "/api/board", Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<CommonCrudResp>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 2);
    assert!(list.iter().any(|b| b.id == board_1_id));
    assert!(list.iter().any(|b| b.id == board_2_id));

    // unauthenticated gets only public board
    let resp = helpers::get(&app.router, "/api/board", None).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<CommonCrudResp>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, board_2_id);

    // fail to get first board by id unauthenticated (private)
    let resp = helpers::get(&app.router, &format!("/api/board/{}", board_1_id), None).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    // get second board by id unauthenticated (public)
    let resp = helpers::get(&app.router, &format!("/api/board/{}", board_2_id), None).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, got): (StatusCode, CommonCrudResp) = helpers::read_json(resp).await;
    assert_struct!(got, CommonCrudResp { id: board_2_id, name: "Demo Board 2", slug: "d-b-2", description: Some("desc"), icon: None, is_public: true });
    assert_eq!(got.is_public, true);

    // update first board by owner (with making it public)
    let resp = helpers::put_json(&app.router, &format!("/api/board/{}", board_1_id), r#"{"name":"Updated Name","description":"new desc","icon":"📋","is_public":true}"#, Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, updated): (StatusCode, CommonCrudResp) = helpers::read_json(resp).await;
    assert_struct!(updated, CommonCrudResp { id: board_1_id, name: "Updated Name", slug: "demo-board-1", description: Some("new desc"), icon: Some("📋"), is_public: true });

    // delete second board by owner
    let resp = helpers::delete(&app.router, &format!("/api/board/{}", board_2_id), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, deleted): (StatusCode, CommonCrudResp) = helpers::read_json(resp).await;
    assert_struct!(deleted, CommonCrudResp { id: board_2_id, name: "Demo Board 2", slug: "d-b-2", description: Some("desc"), icon: None, is_public: true });

    // get first board by id by owner
    let resp = helpers::get(&app.router, &format!("/api/board/{}", board_1_id), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, got): (StatusCode, CommonCrudResp) = helpers::read_json(resp).await;
    assert_eq!(got.id, board_1_id);

    // get second board by id by owner (deleted)
    let resp = helpers::get(&app.router, &format!("/api/board/{}", board_2_id), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    // unauthenticated gets first board (now public) by id
    let resp = helpers::get(&app.router, &format!("/api/board/{}", board_1_id), None).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, got): (StatusCode, CommonCrudResp) = helpers::read_json(resp).await;
    assert_eq!(got.id, board_1_id);
}

#[tokio::test]
async fn board_access_invite_and_permissions_smoke() {
    let app = test_app::TestApp::new().await;

    // Owner session
    let owner_cookie = helpers::auth_cookie_for(&app.router, "owner@example.com").await;

    // Create a private board
    let resp = helpers::post_json(&app.router, "/api/board", r#"{"name":"Secret","description":"hidden","is_public":false}"#, Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let (_status, created): (StatusCode, CommonCrudResp) = helpers::read_json(resp).await;
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
    let (_status, list): (StatusCode, Vec<AccessMapping>) = helpers::read_json(resp).await;
 
    println!("Access list: {:?}", list);
 
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

    let resp = helpers::post_json(&app.router, &format!("/api/board/{}/access", board_id), r#"{"address":"guest@example.com","role":"manage"}"#, Some(&owner_cookie)).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Guest can update now
    let resp = helpers::put_json(&app.router, &format!("/api/board/{}", board_id), r#"{"name":"Managed name"}"#, Some(&guest_cookie)).await;
    assert_eq!(resp.status(), StatusCode::OK);
}
