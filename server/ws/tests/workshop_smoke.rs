use axum::http::StatusCode;
use serde_json::json;

mod test_app;
mod helpers;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
struct WorkshopItemResp { 
    id: i32, 
    name: String, 
    slug: String, 
    description: Option<String>, 
    definition: serde_json::Value,
    created_at: i64,
    updated_at: i64,
}



#[tokio::test]
async fn workshop_global_readonly_smoke_ok() {
    let app = test_app::TestApp::new().await;

    // Global workshop is read-only, should list existing items
    let resp = helpers::get(&app.router, "/api/workshop/shapes", None).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, _list): (StatusCode, Vec<WorkshopItemResp>) = helpers::read_json(resp).await;
    // List may be empty initially, that's expected

    // Test that global creation endpoints are disabled (405 Method Not Allowed)
    let shape_def = json!({
        "background": {"border": {"path": "M0,0 L100,0 L100,50 L0,50 Z", "stroke": {"width": 2, "color": "#000"}}},
        "label": {"position": {"x": 50, "y": 25}, "default_text": "Shape"}
    });

    let owner_cookie_hdr = helpers::auth_cookie_for(&app.router, "user@example.com").await;
    let resp = helpers::post_json(&app.router, "/api/workshop/shapes", &json!({
        "name": "Rectangle Shape",
        "slug": "rectangle",
        "description": "Basic rectangle shape",
        "definition": shape_def
    }).to_string(), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED, "global workshop creation should be disabled");
}

#[tokio::test]
async fn workshop_lines_readonly_smoke_ok() {
    let app = test_app::TestApp::new().await;

    let owner_cookie_hdr = helpers::auth_cookie_for(&app.router, "user@example.com").await;

    // Test that global line creation is disabled
    let line_def = json!({
        "stroke": {"width": 2, "color": "#000", "style": "solid"},
        "source_socket": "output",
        "target_socket": "input"
    });

    let resp = helpers::post_json(&app.router, "/api/workshop/lines", &json!({
        "name": "Solid Line",
        "slug": "solid-line", 
        "description": "Basic solid line",
        "definition": line_def
    }).to_string(), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED, "global workshop line creation should be disabled");

    // Test that we can still list lines
    let resp = helpers::get(&app.router, "/api/workshop/lines", None).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn workshop_package_direct_creation_smoke_ok() {
    let app = test_app::TestApp::new().await;

    let owner_cookie_hdr = helpers::auth_cookie_for(&app.router, "user@example.com").await;

    // Create package
    let resp = helpers::post_json(&app.router, "/api/package", r#"{"name":"Test Package","description":"test"}"#, Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let (_status, package): (StatusCode, serde_json::Value) = helpers::read_json(resp).await;
    let package_id = package["id"].as_i64().unwrap() as i32;

    // Create shape directly in package workshop (new direct creation API)
    let shape_def = json!({"background": {"border": {"path": "rectangle"}}});
    let resp = helpers::post_json(&app.router, &format!("/api/package/{}/workshop/shapes", package_id), &json!({
        "name": "Package Shape",
        "slug": "package-shape",
        "description": "Shape created in package context",
        "definition": shape_def
    }).to_string(), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::CREATED, "create shape directly in package workshop");
    let (_status, created): (StatusCode, WorkshopItemResp) = helpers::read_json(resp).await;
    let shape_id = created.id;

    // List package workshop shapes
    let resp = helpers::get(&app.router, &format!("/api/package/{}/workshop/shapes", package_id), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<WorkshopItemResp>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, shape_id);
    assert_eq!(list[0].name, "Package Shape");

    // Update shape in package workshop
    let resp = helpers::put_json(&app.router, &format!("/api/package/{}/workshop/shapes/{}", package_id, shape_id), &json!({
        "name": "Updated Package Shape",
        "description": "Updated description"
    }).to_string(), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, updated): (StatusCode, WorkshopItemResp) = helpers::read_json(resp).await;
    assert_eq!(updated.name, "Updated Package Shape");

    // Delete shape from package workshop  
    let resp = helpers::delete(&app.router, &format!("/api/package/{}/workshop/shapes/{}", package_id, shape_id), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    
    // Verify shape removed from package
    let resp = helpers::get(&app.router, &format!("/api/package/{}/workshop/shapes", package_id), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<WorkshopItemResp>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 0);
}

#[tokio::test]
async fn workshop_board_direct_creation_and_import_smoke_ok() {
    let app = test_app::TestApp::new().await;

    let owner_cookie_hdr = helpers::auth_cookie_for(&app.router, "user@example.com").await;

    // Create package and board
    let resp = helpers::post_json(&app.router, "/api/package", r#"{"name":"Source Package","is_public":true}"#, Some(&owner_cookie_hdr)).await;
    let (_status, package): (StatusCode, serde_json::Value) = helpers::read_json(resp).await;
    let package_id = package["id"].as_i64().unwrap() as i32;

    let resp = helpers::post_json(&app.router, "/api/board", r#"{"name":"Test Board"}"#, Some(&owner_cookie_hdr)).await;
    let (_status, board): (StatusCode, serde_json::Value) = helpers::read_json(resp).await;
    let board_id = board["id"].as_i64().unwrap() as i32;

    // Create shape directly in package workshop
    let shape_def = json!({"background": "blue"});
    let resp = helpers::post_json(&app.router, &format!("/api/package/{}/workshop/shapes", package_id), &json!({
        "name": "Package Shape",
        "slug": "package-shape", 
        "definition": shape_def
    }).to_string(), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let (_status, _shape): (StatusCode, WorkshopItemResp) = helpers::read_json(resp).await;

    // Create another shape directly in board workshop (board-specific)
    let board_shape_def = json!({"background": "red"});
    let resp = helpers::post_json(&app.router, &format!("/api/board/{}/workshop/shapes", board_id), &json!({
        "name": "Board Shape",
        "slug": "board-shape",
        "definition": board_shape_def
    }).to_string(), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::CREATED, "create shape directly in board workshop");
    let (_status, board_shape): (StatusCode, WorkshopItemResp) = helpers::read_json(resp).await;
    let board_shape_id = board_shape.id;

    // List board workshop shapes (should show both)
    let resp = helpers::get(&app.router, &format!("/api/board/{}/workshop/shapes", board_id), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<WorkshopItemResp>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 1); // Only the board-specific shape
    assert_eq!(list[0].id, board_shape_id);
    assert_eq!(list[0].name, "Board Shape");
}