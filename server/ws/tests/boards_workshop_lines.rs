//! Integration tests for board workshop lines functionality
//! Tests CRUD operations for line workshop items in board context

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
async fn board_workshop_lines_full_crud() {
    let app = test_app::TestApp::new().await;
    let owner_cookie_hdr = helpers::auth_cookie_for(&app.router, "user@example.com").await;

    // 1. Create board
    let resp = helpers::post_json(
        &app.router, 
        "/api/board", 
        r#"{"name":"Test Board","description":"Board for line testing"}"#, 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::CREATED, "Failed to create board");
    let (_status, board): (StatusCode, serde_json::Value) = helpers::read_json(resp).await;
    let board_id = board["id"].as_i64().unwrap() as i32;

    // 2. Create line in board workshop
    let line_def = json!({
        "stroke": {"width": 2, "color": "#000000", "style": "solid"},
        "source_socket": "output",
        "target_socket": "input"
    });
    let resp = helpers::post_json(
        &app.router, 
        &format!("/api/board/{}/workshop/lines", board_id), 
        &json!({
            "name": "Solid Connection Line",
            "slug": "solid-line",
            "description": "A solid black line for connections",
            "definition": line_def
        }).to_string(), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::CREATED, "Failed to create line in board workshop");
    let (_status, created): (StatusCode, WorkshopItemResp) = helpers::read_json(resp).await;
    let line_id = created.id;
    assert_eq!(created.name, "Solid Connection Line");
    assert_eq!(created.slug, "solid-line");
    assert_eq!(created.description, Some("A solid black line for connections".to_string()));

    // 3. List lines in board workshop
    let resp = helpers::get(
        &app.router, 
        &format!("/api/board/{}/workshop/lines", board_id), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::OK, "Failed to list lines");
    let (_status, list): (StatusCode, Vec<WorkshopItemResp>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 1, "Expected 1 line in workshop");
    assert_eq!(list[0].id, line_id);
    assert_eq!(list[0].name, "Solid Connection Line");

    // 4. Create another line
    let dashed_line_def = json!({
        "stroke": {"width": 1, "color": "#666666", "style": "dashed"},
        "source_socket": "output",
        "target_socket": "input"
    });
    let resp = helpers::post_json(
        &app.router, 
        &format!("/api/board/{}/workshop/lines", board_id), 
        &json!({
            "name": "Dashed Line",
            "slug": "dashed-line",
            "description": "A dashed gray line",
            "definition": dashed_line_def
        }).to_string(), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let (_status, created2): (StatusCode, WorkshopItemResp) = helpers::read_json(resp).await;
    let line_id_2 = created2.id;

    // 5. List should now have 2 lines
    let resp = helpers::get(
        &app.router, 
        &format!("/api/board/{}/workshop/lines", board_id), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<WorkshopItemResp>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 2, "Expected 2 lines in workshop");

    // 6. Update first line
    let resp = helpers::put_json(
        &app.router, 
        &format!("/api/board/{}/workshop/lines/{}", board_id, line_id), 
        &json!({
            "name": "Updated Solid Line",
            "description": "Updated description for solid line"
        }).to_string(), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::OK, "Failed to update line");
    let (_status, updated): (StatusCode, WorkshopItemResp) = helpers::read_json(resp).await;
    assert_eq!(updated.id, line_id);
    assert_eq!(updated.name, "Updated Solid Line");
    assert_eq!(updated.description, Some("Updated description for solid line".to_string()));

    // 7. Delete second line
    let resp = helpers::delete(
        &app.router, 
        &format!("/api/board/{}/workshop/lines/{}", board_id, line_id_2), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT, "Failed to delete line");
    
    // 8. Verify only 1 line remains
    let resp = helpers::get(
        &app.router, 
        &format!("/api/board/{}/workshop/lines", board_id), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<WorkshopItemResp>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 1, "Expected 1 line after deletion");
    assert_eq!(list[0].id, line_id, "Wrong line remaining");
    assert_eq!(list[0].name, "Updated Solid Line");

    // 9. Delete remaining line
    let resp = helpers::delete(
        &app.router, 
        &format!("/api/board/{}/workshop/lines/{}", board_id, line_id), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    
    // 10. Verify workshop is empty
    let resp = helpers::get(
        &app.router, 
        &format!("/api/board/{}/workshop/lines", board_id), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<WorkshopItemResp>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 0, "Workshop should be empty after deleting all lines");
}

#[tokio::test]
async fn board_workshop_lines_requires_authentication() {
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

    // Try to create line without authentication
    let line_def = json!({"stroke": {"width": 2}});
    let resp = helpers::post_json(
        &app.router, 
        &format!("/api/board/{}/workshop/lines", board_id), 
        &json!({
            "name": "Unauthorized Line",
            "slug": "unauth-line",
            "definition": line_def
        }).to_string(), 
        None // No authentication
    ).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED, "Should require authentication for create");

    // List should work without authentication (read-only)
    let resp = helpers::get(
        &app.router, 
        &format!("/api/board/{}/workshop/lines", board_id), 
        None
    ).await;
    assert_eq!(resp.status(), StatusCode::OK, "List should work without auth for public board");
}
