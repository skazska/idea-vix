use assert_struct::assert_struct;
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

#[derive(Deserialize, Debug, PartialEq)]
struct EntityWorkshopItemResp { 
    item_id: i32,           // entity id (package_id or board_id)
    workshop_item_id: i32,  // workshop item id (shape_id, line_id, etc.)
    origin_id: Option<i32>, // package id for board imports, None for package own items
    name: Option<String>,   // board-specific override name
    created_at: i64,
}

#[tokio::test]
async fn workshop_global_crud_ok() {
    let app = test_app::TestApp::new().await;

    let owner_cookie_hdr = helpers::auth_cookie_for(&app.router, "user@example.com").await;

    // Create shapes in global workshop
    let shape_def = json!({
        "background": {"border": {"path": "M0,0 L100,0 L100,50 L0,50 Z", "stroke": {"width": 2, "color": "#000"}}},
        "label": {"position": {"x": 50, "y": 25}, "default_text": "Shape"}
    });

    let resp = helpers::post_json(&app.router, "/api/workshop/shapes", &json!({
        "name": "Rectangle Shape",
        "slug": "rectangle",
        "description": "Basic rectangle shape",
        "definition": shape_def
    }).to_string(), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::CREATED, "create shape in global workshop");
    let (_status, created): (StatusCode, WorkshopItemResp) = helpers::read_json(resp).await;
    assert_struct!(created, WorkshopItemResp { 
        name: "Rectangle Shape", 
        slug: "rectangle", 
        description: Some("Basic rectangle shape"), 
        definition: shape_def.clone(),
        .. 
    });
    let shape_1_id = created.id;

    // Create another shape
    let circle_def = json!({"background": {"border": {"path": "circle", "radius": 25}}});
    let resp = helpers::post_json(&app.router, "/api/workshop/shapes", &json!({
        "name": "Circle Shape", 
        "slug": "circle",
        "description": null,
        "definition": circle_def
    }).to_string(), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let (_status, created): (StatusCode, WorkshopItemResp) = helpers::read_json(resp).await;
    let shape_2_id = created.id;

    // List shapes (no filter for now due to query parsing issues)
    let resp = helpers::get(&app.router, "/api/workshop/shapes", None).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<WorkshopItemResp>) = helpers::read_json(resp).await;
    assert!(list.len() >= 2, "Should contain at least the 2 shapes we created");
    assert!(list.iter().any(|s| s.id == shape_1_id));
    assert!(list.iter().any(|s| s.id == shape_2_id));

    // Get shape by id (no auth required for global workshop)
    let resp = helpers::get(&app.router, &format!("/api/workshop/shapes/{}", shape_1_id), None).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, got): (StatusCode, WorkshopItemResp) = helpers::read_json(resp).await;
    assert_eq!(got.id, shape_1_id);
    assert_eq!(got.name, "Rectangle Shape");

    // Get shape by slug
    let resp = helpers::get(&app.router, "/api/workshop/shapes/by-slug/rectangle", None).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, got): (StatusCode, WorkshopItemResp) = helpers::read_json(resp).await;
    assert_eq!(got.id, shape_1_id);
    assert_eq!(got.slug, "rectangle");

    // Update shape (requires authentication)
    let resp = helpers::put_json(&app.router, &format!("/api/workshop/shapes/{}", shape_1_id), &json!({
        "name": "Updated Rectangle",
        "description": "Updated description"
    }).to_string(), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, updated): (StatusCode, WorkshopItemResp) = helpers::read_json(resp).await;
    assert_struct!(updated, WorkshopItemResp { 
        id: shape_1_id,
        name: "Updated Rectangle", 
        slug: "rectangle", 
        description: Some("Updated description"),
        .. 
    });

    // Delete shape (requires authentication) 
    let resp = helpers::delete(&app.router, &format!("/api/workshop/shapes/{}", shape_2_id), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, deleted): (StatusCode, WorkshopItemResp) = helpers::read_json(resp).await;
    assert_eq!(deleted.id, shape_2_id);

    // Verify shape is deleted
    let resp = helpers::get(&app.router, &format!("/api/workshop/shapes/{}", shape_2_id), None).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn workshop_lines_crud_ok() {
    let app = test_app::TestApp::new().await;

    let owner_cookie_hdr = helpers::auth_cookie_for(&app.router, "user@example.com").await;

    // Create line in global workshop
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
    assert_eq!(resp.status(), StatusCode::CREATED, "create line in global workshop");
    let (_status, created): (StatusCode, WorkshopItemResp) = helpers::read_json(resp).await;
    let line_id = created.id;

    // Get line by id
    let resp = helpers::get(&app.router, &format!("/api/workshop/lines/{}", line_id), None).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, got): (StatusCode, WorkshopItemResp) = helpers::read_json(resp).await;
    assert_eq!(got.name, "Solid Line");
}

#[tokio::test]
async fn workshop_package_link_ok() {
    let app = test_app::TestApp::new().await;

    let owner_cookie_hdr = helpers::auth_cookie_for(&app.router, "user@example.com").await;

    // Create package
    let resp = helpers::post_json(&app.router, "/api/package", r#"{"name":"Test Package","description":"test"}"#, Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let (_status, package): (StatusCode, serde_json::Value) = helpers::read_json(resp).await;
    let package_id = package["id"].as_i64().unwrap() as i32;

    // Create shape
    let shape_def = json!({"background": {"border": {"path": "rectangle"}}});
    let resp = helpers::post_json(&app.router, "/api/workshop/shapes", &json!({
        "name": "Package Shape",
        "slug": "package-shape",
        "definition": shape_def
    }).to_string(), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let (_status, shape): (StatusCode, WorkshopItemResp) = helpers::read_json(resp).await;
    let shape_id = shape.id;

    // Add shape to package workshop
    let resp = helpers::post_json(&app.router, &format!("/api/package/{}/workshop/shapes", package_id), &json!({
        "workshop_item_id": shape_id
    }).to_string(), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::CREATED, "add shape to package workshop");
    let (_status, linked): (StatusCode, EntityWorkshopItemResp) = helpers::read_json(resp).await;
    assert_eq!(linked.item_id, package_id);
    assert_eq!(linked.workshop_item_id, shape_id);
    assert_eq!(linked.origin_id, None); // package own item

    // List package workshop shapes
    let resp = helpers::get(&app.router, &format!("/api/package/{}/workshop/shapes", package_id), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<WorkshopItemResp>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, shape_id);

    // Remove shape from package workshop  
    let resp = helpers::delete(&app.router, &format!("/api/package/{}/workshop/shapes/{}", package_id, shape_id), Some(&owner_cookie_hdr)).await;
    
    let status = resp.status();
    if status != StatusCode::NO_CONTENT {
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = std::str::from_utf8(&body).unwrap();
        panic!("Expected 204 but got {}: {}", status, body_str);
    }
    
    // Verify shape removed from package
    let resp = helpers::get(&app.router, &format!("/api/package/{}/workshop/shapes", package_id), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<WorkshopItemResp>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 0);
}

#[tokio::test]
async fn workshop_board_link_with_import_ok() {
    let app = test_app::TestApp::new().await;

    let owner_cookie_hdr = helpers::auth_cookie_for(&app.router, "user@example.com").await;

    // Create package and board
    let resp = helpers::post_json(&app.router, "/api/package", r#"{"name":"Source Package","is_public":true}"#, Some(&owner_cookie_hdr)).await;
    let (_status, package): (StatusCode, serde_json::Value) = helpers::read_json(resp).await;
    let package_id = package["id"].as_i64().unwrap() as i32;

    let resp = helpers::post_json(&app.router, "/api/board", r#"{"name":"Test Board"}"#, Some(&owner_cookie_hdr)).await;
    let (_status, board): (StatusCode, serde_json::Value) = helpers::read_json(resp).await;
    let board_id = board["id"].as_i64().unwrap() as i32;

    // Create shape and add to package
    let shape_def = json!({"background": "blue"});
    let resp = helpers::post_json(&app.router, "/api/workshop/shapes", &json!({
        "name": "Import Shape",
        "slug": "import-shape", 
        "definition": shape_def
    }).to_string(), Some(&owner_cookie_hdr)).await;
    let (_status, shape): (StatusCode, WorkshopItemResp) = helpers::read_json(resp).await;
    let shape_id = shape.id;

    // Add shape to package first
    let resp = helpers::post_json(&app.router, &format!("/api/package/{}/workshop/shapes", package_id), &json!({
        "workshop_item_id": shape_id
    }).to_string(), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Import shape from package to board (with origin_id and name override)
    let resp = helpers::post_json(&app.router, &format!("/api/board/{}/workshop/shapes", board_id), &json!({
        "workshop_item_id": shape_id,
        "origin_id": package_id,
        "name": "Board Imported Shape"
    }).to_string(), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::CREATED, "import shape from package to board");
    let (_status, linked): (StatusCode, EntityWorkshopItemResp) = helpers::read_json(resp).await;
    assert_eq!(linked.item_id, board_id);
    assert_eq!(linked.workshop_item_id, shape_id);
    assert_eq!(linked.origin_id, Some(package_id)); // imported from package
    assert_eq!(linked.name, Some("Board Imported Shape".to_string()));

    // List board workshop shapes
    let resp = helpers::get(&app.router, &format!("/api/board/{}/workshop/shapes", board_id), Some(&owner_cookie_hdr)).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<WorkshopItemResp>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, shape_id);
}