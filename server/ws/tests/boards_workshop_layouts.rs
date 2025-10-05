//! Integration tests for board workshop layouts functionality
//! Tests CRUD operations for layout workshop items in board context

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
async fn board_workshop_layouts_full_crud() {
    let app = test_app::TestApp::new().await;
    let owner_cookie_hdr = helpers::auth_cookie_for(&app.router, "user@example.com").await;

    // 1. Create board
    let resp = helpers::post_json(
        &app.router, 
        "/api/board", 
        r#"{"name":"Test Board","description":"Board for layout testing"}"#, 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::CREATED, "Failed to create board");
    let (_status, board): (StatusCode, serde_json::Value) = helpers::read_json(resp).await;
    let board_id = board["id"].as_i64().unwrap() as i32;

    // 2. Create layout in board workshop
    let layout_def = json!({
        "type": "hierarchical",
        "direction": "top-to-bottom",
        "spacing": {
            "horizontal": 100,
            "vertical": 80
        },
        "alignment": "center",
        "options": {
            "auto_arrange": true,
            "prevent_overlap": true
        }
    });
    // Use unique slug with board_id to avoid conflicts
    let unique_slug = format!("hierarchical-{}", board_id);
    let resp = helpers::post_json(
        &app.router, 
        &format!("/api/board/{}/workshop/layouts", board_id), 
        &json!({
            "name": "Hierarchical Layout",
            "slug": unique_slug,
            "description": "Top-to-bottom hierarchical layout",
            "definition": layout_def
        }).to_string(), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::CREATED, "Failed to create layout in board workshop");
    let (_status, created): (StatusCode, WorkshopItemResp) = helpers::read_json(resp).await;
    let layout_id = created.id;
    assert_eq!(created.name, "Hierarchical Layout");
    assert_eq!(created.slug, unique_slug);
    assert_eq!(created.description, Some("Top-to-bottom hierarchical layout".to_string()));

    // 3. List layouts in board workshop
    let resp = helpers::get(
        &app.router, 
        &format!("/api/board/{}/workshop/layouts", board_id), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::OK, "Failed to list layouts");
    let (_status, list): (StatusCode, Vec<WorkshopItemResp>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 1, "Expected 1 layout in workshop");
    assert_eq!(list[0].id, layout_id);
    assert_eq!(list[0].name, "Hierarchical Layout");

    // 4. Create another layout
    let grid_layout_def = json!({
        "type": "grid",
        "columns": 4,
        "rows": 3,
        "cell_width": 150,
        "cell_height": 100,
        "alignment": "center"
    });
    let unique_slug_2 = format!("grid-layout-{}", board_id);
    let resp = helpers::post_json(
        &app.router, 
        &format!("/api/board/{}/workshop/layouts", board_id), 
        &json!({
            "name": "Grid Layout",
            "slug": unique_slug_2,
            "description": "4x3 grid layout",
            "definition": grid_layout_def
        }).to_string(), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let (_status, created2): (StatusCode, WorkshopItemResp) = helpers::read_json(resp).await;
    let layout_id_2 = created2.id;

    // 5. List should now have 2 layouts
    let resp = helpers::get(
        &app.router, 
        &format!("/api/board/{}/workshop/layouts", board_id), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<WorkshopItemResp>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 2, "Expected 2 layouts in workshop");

    // 6. Update first layout
    let resp = helpers::put_json(
        &app.router, 
        &format!("/api/board/{}/workshop/layouts/{}", board_id, layout_id), 
        &json!({
            "name": "Updated Hierarchical Layout",
            "description": "Modified hierarchical layout with better spacing"
        }).to_string(), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::OK, "Failed to update layout");
    let (_status, updated): (StatusCode, WorkshopItemResp) = helpers::read_json(resp).await;
    assert_eq!(updated.id, layout_id);
    assert_eq!(updated.name, "Updated Hierarchical Layout");
    assert_eq!(updated.description, Some("Modified hierarchical layout with better spacing".to_string()));

    // 7. Delete second layout
    let resp = helpers::delete(
        &app.router, 
        &format!("/api/board/{}/workshop/layouts/{}", board_id, layout_id_2), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT, "Failed to delete layout");
    
    // 8. Verify only 1 layout remains
    let resp = helpers::get(
        &app.router, 
        &format!("/api/board/{}/workshop/layouts", board_id), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<WorkshopItemResp>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 1, "Expected 1 layout after deletion");
    assert_eq!(list[0].id, layout_id, "Wrong layout remaining");
    assert_eq!(list[0].name, "Updated Hierarchical Layout");

    // 9. Delete remaining layout
    let resp = helpers::delete(
        &app.router, 
        &format!("/api/board/{}/workshop/layouts/{}", board_id, layout_id), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    
    // 10. Verify workshop is empty
    let resp = helpers::get(
        &app.router, 
        &format!("/api/board/{}/workshop/layouts", board_id), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<WorkshopItemResp>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 0, "Workshop should be empty after deleting all layouts");
}

#[tokio::test]
async fn board_workshop_layouts_requires_authentication() {
    let app = test_app::TestApp::new().await;
    let owner_cookie_hdr = helpers::auth_cookie_for(&app.router, "user@example.com").await;

    // Create board
    let resp = helpers::post_json(
        &app.router, 
        "/api/board", 
        r#"{"name":"Auth Test Board"}"#, 
        Some(&owner_cookie_hdr)
    ).await;
    let (_status, board): (StatusCode, serde_json::Value) = helpers::read_json(resp).await;
    let board_id = board["id"].as_i64().unwrap() as i32;

    // Try to create layout without authentication
    let layout_def = json!({"type": "grid"});
    let resp = helpers::post_json(
        &app.router, 
        &format!("/api/board/{}/workshop/layouts", board_id), 
        &json!({
            "name": "Unauthorized Layout",
            "slug": "unauth-layout",
            "definition": layout_def
        }).to_string(), 
        None // No authentication
    ).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED, "Should require authentication for create");

    // List should work without authentication (read-only)
    let resp = helpers::get(
        &app.router, 
        &format!("/api/board/{}/workshop/layouts", board_id), 
        None
    ).await;
    assert_eq!(resp.status(), StatusCode::OK, "List should work without auth for public board");
}
