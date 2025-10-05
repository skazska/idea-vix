//! Integration tests for board workshop rules functionality
//! Tests CRUD operations for rule workshop items in board context

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
async fn board_workshop_rules_full_crud() {
    let app = test_app::TestApp::new().await;
    let owner_cookie_hdr = helpers::auth_cookie_for(&app.router, "user@example.com").await;

    // 1. Create board
    let resp = helpers::post_json(
        &app.router, 
        "/api/board", 
        r#"{"name":"Test Board","description":"Board for rule testing"}"#, 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::CREATED, "Failed to create board");
    let (_status, board): (StatusCode, serde_json::Value) = helpers::read_json(resp).await;
    let board_id = board["id"].as_i64().unwrap() as i32;

    // 2. Create rule in board workshop
    let rule_def = json!({
        "source_shape": "rectangle",
        "target_shape": "circle",
        "line_type": "solid-line",
        "allowed": true,
        "validation": {
            "max_connections": 5,
            "bidirectional": false
        }
    });
    // Use unique slug with board_id to avoid conflicts
    let unique_slug = format!("rect-to-circle-{}", board_id);
    let resp = helpers::post_json(
        &app.router, 
        &format!("/api/board/{}/workshop/rules", board_id), 
        &json!({
            "name": "Rectangle to Circle Rule",
            "slug": unique_slug,
            "description": "Allows connection from rectangle to circle",
            "definition": rule_def
        }).to_string(), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::CREATED, "Failed to create rule in board workshop");
    let (_status, created): (StatusCode, WorkshopItemResp) = helpers::read_json(resp).await;
    let rule_id = created.id;
    assert_eq!(created.name, "Rectangle to Circle Rule");
    assert_eq!(created.slug, unique_slug);
    assert_eq!(created.description, Some("Allows connection from rectangle to circle".to_string()));

    // 3. List rules in board workshop
    let resp = helpers::get(
        &app.router, 
        &format!("/api/board/{}/workshop/rules", board_id), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::OK, "Failed to list rules");
    let (_status, list): (StatusCode, Vec<WorkshopItemResp>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 1, "Expected 1 rule in workshop");
    assert_eq!(list[0].id, rule_id);
    assert_eq!(list[0].name, "Rectangle to Circle Rule");

    // 4. Create another rule
    let universal_rule_def = json!({
        "source_shape": "*",
        "target_shape": "*",
        "line_type": "*",
        "allowed": true,
        "validation": {
            "max_connections": null,
            "bidirectional": true
        }
    });
    let unique_slug_2 = format!("universal-rule-{}", board_id);
    let resp = helpers::post_json(
        &app.router, 
        &format!("/api/board/{}/workshop/rules", board_id), 
        &json!({
            "name": "Universal Connection Rule",
            "slug": unique_slug_2,
            "description": "Allows any connection",
            "definition": universal_rule_def
        }).to_string(), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let (_status, created2): (StatusCode, WorkshopItemResp) = helpers::read_json(resp).await;
    let rule_id_2 = created2.id;

    // 5. List should now have 2 rules
    let resp = helpers::get(
        &app.router, 
        &format!("/api/board/{}/workshop/rules", board_id), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<WorkshopItemResp>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 2, "Expected 2 rules in workshop");

    // 6. Update first rule
    let resp = helpers::put_json(
        &app.router, 
        &format!("/api/board/{}/workshop/rules/{}", board_id, rule_id), 
        &json!({
            "name": "Updated Rectangle Rule",
            "description": "Modified connection rule"
        }).to_string(), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::OK, "Failed to update rule");
    let (_status, updated): (StatusCode, WorkshopItemResp) = helpers::read_json(resp).await;
    assert_eq!(updated.id, rule_id);
    assert_eq!(updated.name, "Updated Rectangle Rule");
    assert_eq!(updated.description, Some("Modified connection rule".to_string()));

    // 7. Delete second rule
    let resp = helpers::delete(
        &app.router, 
        &format!("/api/board/{}/workshop/rules/{}", board_id, rule_id_2), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT, "Failed to delete rule");
    
    // 8. Verify only 1 rule remains
    let resp = helpers::get(
        &app.router, 
        &format!("/api/board/{}/workshop/rules", board_id), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<WorkshopItemResp>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 1, "Expected 1 rule after deletion");
    assert_eq!(list[0].id, rule_id, "Wrong rule remaining");
    assert_eq!(list[0].name, "Updated Rectangle Rule");

    // 9. Delete remaining rule
    let resp = helpers::delete(
        &app.router, 
        &format!("/api/board/{}/workshop/rules/{}", board_id, rule_id), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    
    // 10. Verify workshop is empty
    let resp = helpers::get(
        &app.router, 
        &format!("/api/board/{}/workshop/rules", board_id), 
        Some(&owner_cookie_hdr)
    ).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let (_status, list): (StatusCode, Vec<WorkshopItemResp>) = helpers::read_json(resp).await;
    assert_eq!(list.len(), 0, "Workshop should be empty after deleting all rules");
}

#[tokio::test]
async fn board_workshop_rules_requires_authentication() {
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

    // Try to create rule without authentication
    let rule_def = json!({"allowed": true});
    let resp = helpers::post_json(
        &app.router, 
        &format!("/api/board/{}/workshop/rules", board_id), 
        &json!({
            "name": "Unauthorized Rule",
            "slug": "unauth-rule",
            "definition": rule_def
        }).to_string(), 
        None // No authentication
    ).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED, "Should require authentication for create");

    // List should work without authentication (read-only)
    let resp = helpers::get(
        &app.router, 
        &format!("/api/board/{}/workshop/rules", board_id), 
        None
    ).await;
    assert_eq!(resp.status(), StatusCode::OK, "List should work without auth for public board");
}
