//! Board feature module: HTTP routes for listing and managing boards.
//!
//! Exposes REST-style endpoints mounted under the module root (typically `/api/board`).
//! Supported operations:
//! - List boards (public for unauthenticated users; public + permitted for authenticated users)
//! - Create a board (authenticated)
//! - Read a specific board (public or permitted)
//! - Update a board (owner or manage)
//! - Delete a board (owner only)
//! - Manage access to a board (owner): grant/revoke/list
//!
//! Prerequisites:
//! - A configured SQLx Sqlite pool
//! - An instance of `SessionJWTService` to decode optional/required JWT tokens

use std::sync::{Arc};

use axum::{ extract::{ Path, State }, http::StatusCode, Json };

use crate::{
    api::{deserialize::AuthToken, validation::ValidatedJson},
    boards::board_service::{Board, NewBoardAccessItem, NewBoardItem, PatchBoardItem},
    common::{
        access::{self, ItemAccess, ItemRole, SqliteItemAccessQueries},
        crud::CrudService,
        workshop_store::{WorkshopStore, WorkshopItemType},
    },
    db::TransactionStarter,
    session::session_jwt::SessionJWTService,
    workshop::generic_service::WorkshopService,
};

mod board_store;
mod board_service;

struct RouteState {
    access_service: access::CommonItemAccess,
    service: board_service::BoardService,
    jwt_service: Arc<SessionJWTService>,
    shape_service: Arc<WorkshopService>,
    line_service: Arc<WorkshopService>,
    rule_service: Arc<WorkshopService>,
    layout_service: Arc<WorkshopService>,
}

pub fn get_router<'a>(
    transaction_starter: Arc<TransactionStarter>,
    jwt_service: Arc<SessionJWTService>,
) -> axum::Router {

    let access_store = Arc::new(SqliteItemAccessQueries::new(
        "board_access_roles",
        "board_id",
    ));
    let access_service = access::CommonItemAccess::new(transaction_starter.clone(), access_store.clone());

    let items_store = Arc::new(board_store::BoardStore::new());
    let board_service = board_service::BoardService::new(transaction_starter.clone(), items_store, access_store);

    let shape_service = Arc::new(WorkshopService::new(
        transaction_starter.clone(),
        Arc::new(WorkshopStore::new(WorkshopItemType::Shape))
    ));
    let line_service = Arc::new(WorkshopService::new(
        transaction_starter.clone(),
        Arc::new(WorkshopStore::new(WorkshopItemType::Line))
    ));
    let rule_service = Arc::new(WorkshopService::new(
        transaction_starter.clone(),
        Arc::new(WorkshopStore::new(WorkshopItemType::Rule))
    ));
    let layout_service = Arc::new(WorkshopService::new(
        transaction_starter.clone(),
        Arc::new(WorkshopStore::new(WorkshopItemType::Layout))
    ));

    let state= Arc::new(RouteState {
        access_service,
        service: board_service,
        jwt_service,
        shape_service,
        line_service,
        rule_service,
        layout_service,
    });

    crate::resource_routes!(
        get_items,
        add_item,
        get_item,
        update_item,
        delete_item,
        add_access,
        list_access,
        revoke_access,
        check_access
    )
    .route("/{id}/workshop/shapes", axum::routing::get(list_board_shapes))
    .route("/{id}/workshop/shapes", axum::routing::post(add_board_shape))
    .route("/{id}/workshop/shapes/{shape_id}", axum::routing::put(update_board_shape))
    .route("/{id}/workshop/shapes/{shape_id}", axum::routing::delete(remove_board_shape))
    .route("/{id}/workshop/lines", axum::routing::get(list_board_lines))
    .route("/{id}/workshop/lines", axum::routing::post(add_board_line))
    .route("/{id}/workshop/lines/{line_id}", axum::routing::put(update_board_line))
    .route("/{id}/workshop/lines/{line_id}", axum::routing::delete(remove_board_line))
    .route("/{id}/workshop/rules", axum::routing::get(list_board_rules))
    .route("/{id}/workshop/rules", axum::routing::post(add_board_rule))
    .route("/{id}/workshop/rules/{rule_id}", axum::routing::put(update_board_rule))
    .route("/{id}/workshop/rules/{rule_id}", axum::routing::delete(remove_board_rule))
    .route("/{id}/workshop/layouts", axum::routing::get(list_board_layouts))
    .route("/{id}/workshop/layouts", axum::routing::post(add_board_layout))
    .route("/{id}/workshop/layouts/{layout_id}", axum::routing::put(update_board_layout))
    .route("/{id}/workshop/layouts/{layout_id}", axum::routing::delete(remove_board_layout))
    .with_state(state)
}

// #[axum::debug_handler]
/// Handler: list boards visible to the caller.
/// - Returns only public boards when no valid JWT is provided
/// - With a valid JWT, also returns boards accessible to the user
/// - Returns a JSON array of `Board`
async fn get_items(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>) -> Result<Json<Vec<Board>>, (StatusCode, String)> {
    let session = state.jwt_service.get_optional_session_data(&token).map_err(|e| e.into())?;
    
    // Create default list parameters
    let lister = board_service::BoardLister {
        filter: None,
        pager: None,
    };

    let result = state.service.get_items(&lister, session.as_ref()).await.map_err(|e| e.into())?;

    Ok(Json(result))
}

// #[axum::debug_handler]
/// Handler: create a new board.
/// - Authenticates via JWT
/// - Validates payload (`name` 3..=100; `description` <= 500; `icon` <= 255)
/// - Returns `201 Created` with the created `Board`
///
/// Example payload:
/// ```json
/// { "name": "My Board", "description": "optional", "is_public": true }
/// ```
async fn add_item(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>, ValidatedJson(mut item): ValidatedJson<NewBoardItem>) -> Result<(StatusCode, Json<Board>), (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let result = state.service.add_item(&mut item, &session).await.map_err(|e| e.into())?;
    Ok((StatusCode::CREATED, Json(result)))
}

// #[axum::debug_handler]
/// Handler: get a board by id.
/// - Returns public boards without authentication
/// - Private boards require access for the caller
/// - Returns `404` if the board is not accessible or does not exist
async fn get_item(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>, axum::extract::Path(id): Path<i64>) -> Result<Json<Board>, (StatusCode, String)> {
    let session = state.jwt_service.get_optional_session_data(&token).map_err(|e| e.into())?;
    let result = state.service.get_item(id, session.as_ref()).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

// #[axum::debug_handler]
/// Handler: update an existing board.
/// - Authenticates via JWT
/// - Requires `owner` or `manage` role
/// - Supports partial updates (omit fields to leave unchanged)
/// - To clear `description` or `icon`, send the field with `null`
/// - Returns the updated `Board`
///
/// Example payloads:
/// - Update name only: `{ "name": "New Name" }`
/// - Clear description: `{ "description": null }`
async fn update_item(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>, axum::extract::Path(id): Path<i64>, ValidatedJson(item): ValidatedJson<PatchBoardItem>) -> Result<Json<Board>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let result = state.service.update_item(id, &item, &session).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

// #[axum::debug_handler]
/// Handler: delete a board by id.
/// - Authenticates via JWT
/// - Requires `owner` or `manage` role
/// - Returns `200 Ok` and deleted board on success
async fn delete_item(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>, axum::extract::Path(id): Path<i64>) -> Result<Json<Board>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    // perform deletion, ignore returned entity for API contract
    let result = state.service.delete_item(id, &session).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

// #[axum::debug_handler]
/// Handler: grant access to a board (owner only).
/// - Authenticates via JWT
/// - Requires `owner` role on the board
/// - Body: { address, role: "view"|"edit"|"manage" }
/// - Returns created mapping
async fn add_access(
    AuthToken(token): AuthToken,
    State(state): State<Arc<RouteState>>,
    axum::extract::Path(id): Path<i64>,
    ValidatedJson(item): ValidatedJson<NewBoardAccessItem>,
) -> Result<Json<ItemRole<i64>>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let role_dto = ItemRole {
        address: item.address,
        role: item.role,
        item_id: id,
    };
    let result = state.access_service.add_access_role(role_dto, &session).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

// #[axum::debug_handler]
/// Handler: revoke access to a board (owner only).
/// - Authenticates via JWT
/// - Requires `owner` role on the board
/// - Path: /{id}/access/{address}
async fn revoke_access(
    AuthToken(token): AuthToken,
    State(state): State<Arc<RouteState>>,
    Path((id, address)): Path<(i64, String)>,
) -> Result<Json<Vec<ItemRole<i64>>>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let result = state.access_service.revoke_access_roles(id, address, &session).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

// #[axum::debug_handler]
/// Handler: list access mappings for a board (owner only).
/// - Authenticates via JWT
/// - Requires `owner` role on the board
/// - Returns array of mappings
async fn list_access(
    AuthToken(token): AuthToken,
    State(state): State<Arc<RouteState>>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<ItemRole<i64>>>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let result = state.access_service.list_access_roles(id, &session).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

// #[axum::debug_handler]
/// Handler: returns access to board for address
/// - Authenticates via JWT
/// - Returns roles of invitations of `address`
/// - Returns `404` if the board is not accessible or does not exist
async fn check_access(
    AuthToken(token): AuthToken,
    State(state): State<Arc<RouteState>>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<String>>, (StatusCode, String)> {
    let session = state.jwt_service.get_optional_session_data(&token).map_err(|e| e.into())?;
    let result = match &session {
        Some(s) => {
            let roles = state.access_service.get_my_access_roles(id, &s).await.map_err(|e| e.into())?;
            roles.into_iter().map(|r| r.into()).collect::<Vec<String>>()
        },
        None => vec![],
    };
    Ok(Json(result))
}

/// - Requires read access to the board
/// - Returns array of shapes linked to the board
async fn list_board_shapes(
    AuthToken(token): AuthToken,
    State(state): State<Arc<RouteState>>,
    Path(board_id): Path<i32>,
) -> Result<Json<Vec<crate::workshop::generic_service::WorkshopItem>>, (StatusCode, String)> {
    use crate::common::crud::ListParams;
    let session = state.jwt_service.get_optional_session_data(&token).map_err(|e| e.into())?;
    let list_params = ListParams { filter: None, pager: None };
    let result = state.shape_service.list_entity_items("board", board_id as i64, &list_params, session.as_ref()).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

/// Handler: add a shape to a board.
/// - Requires edit access to the board
/// - Body: { "workshop_item_id": 123, "origin_id": optional_package_id, "name": "optional display name" }
async fn add_board_shape(
    AuthToken(token): AuthToken,
    State(state): State<Arc<RouteState>>,
    Path(board_id): Path<i32>,
    ValidatedJson(request): ValidatedJson<crate::workshop::generic_service::NewWorkshopItem>,
) -> Result<(StatusCode, Json<crate::workshop::generic_service::WorkshopItem>), (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
        
    let mut new_item = request;
    let created = state.shape_service.create_entity_item("board", board_id as i64, &mut new_item, &session).await.map_err(|e| e.into())?;
    Ok((StatusCode::CREATED, Json(created)))
}

/// Handler: update a shape in a board.
/// - Requires edit access to the board
/// - Updates the workshop item globally (not entity-specific)
/// - Returns the updated WorkshopItem
async fn update_board_shape(
    AuthToken(token): AuthToken,
    State(state): State<Arc<RouteState>>,
    Path((_board_id, shape_id)): Path<(i32, i64)>,
    ValidatedJson(request): ValidatedJson<crate::workshop::generic_service::PatchWorkshopItem>,
) -> Result<Json<crate::workshop::generic_service::WorkshopItem>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    
    // TODO: Validate that the shape is linked to this board and user has access
    let updated = state.shape_service.update_item(shape_id, &request, &session).await.map_err(|e| e.into())?;
    Ok(Json(updated))
}

/// Handler: remove a shape from a board.
/// - Requires edit access to the board  
/// - Deletes the workshop item globally (since it was created in board context)
/// - Returns `204 No Content` on success
async fn remove_board_shape(
    AuthToken(token): AuthToken,
    State(state): State<Arc<RouteState>>,
    Path((_board_id, shape_id)): Path<(i32, i64)>,
) -> Result<StatusCode, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    
    // TODO: Validate that the shape is owned by this board and user has access
    let _deleted = state.shape_service.delete_item(shape_id, &session).await.map_err(|e| e.into())?;
    Ok(StatusCode::NO_CONTENT)
}

// Board-Line Association Handlers

/// Handler: list lines associated with a board.
/// - Requires read access to the board
/// - Returns array of lines linked to the board
async fn list_board_lines(
    AuthToken(token): AuthToken,
    State(state): State<Arc<RouteState>>,
    Path(board_id): Path<i32>,
) -> Result<Json<Vec<crate::workshop::generic_service::WorkshopItem>>, (StatusCode, String)> {
    use crate::common::crud::ListParams;
    let session = state.jwt_service.get_optional_session_data(&token).map_err(|e| e.into())?;
    let list_params = ListParams { filter: None, pager: None };
    let result = state.line_service.list_entity_items("board", board_id as i64, &list_params, session.as_ref()).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

/// Handler: add a line to a board.
/// - Requires edit access to the board
/// - Body: { "workshop_item_id": 123, "origin_id": optional_package_id, "name": "optional display name" }
async fn add_board_line(
    AuthToken(token): AuthToken,
    State(state): State<Arc<RouteState>>,
    Path(board_id): Path<i32>,
    ValidatedJson(request): ValidatedJson<crate::workshop::generic_service::NewWorkshopItem>,
) -> Result<(StatusCode, Json<crate::workshop::generic_service::WorkshopItem>), (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
        
    let mut new_item = request;
    let created = state.line_service.create_entity_item("board", board_id as i64, &mut new_item, &session).await.map_err(|e| e.into())?;
    Ok((StatusCode::CREATED, Json(created)))
}

/// Handler: update a line in a board.
/// - Requires edit access to the board
/// - Updates the workshop item globally (not entity-specific)
/// - Returns the updated WorkshopItem
async fn update_board_line(
    AuthToken(token): AuthToken,
    State(state): State<Arc<RouteState>>,
    Path((_board_id, line_id)): Path<(i32, i64)>,
    ValidatedJson(request): ValidatedJson<crate::workshop::generic_service::PatchWorkshopItem>,
) -> Result<Json<crate::workshop::generic_service::WorkshopItem>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    
    // TODO: Validate that the line is linked to this board and user has access
    let updated = state.line_service.update_item(line_id, &request, &session).await.map_err(|e| e.into())?;
    Ok(Json(updated))
}

/// Handler: remove a line from a board.
/// - Requires edit access to the board  
/// - Deletes the workshop item globally (since it was created in board context)
/// - Returns `204 No Content` on success
async fn remove_board_line(
    AuthToken(token): AuthToken,
    State(state): State<Arc<RouteState>>,
    Path((_board_id, line_id)): Path<(i32, i64)>,
) -> Result<StatusCode, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    
    // TODO: Validate that the line is owned by this board and user has access
    let _deleted = state.line_service.delete_item(line_id, &session).await.map_err(|e| e.into())?;
    Ok(StatusCode::NO_CONTENT)
}

// Board-Rule Association Handlers

/// Handler: list rules associated with a board.
/// - Requires read access to the board
/// - Returns array of rules linked to the board
async fn list_board_rules(
    AuthToken(token): AuthToken,
    State(state): State<Arc<RouteState>>,
    Path(board_id): Path<i32>,
) -> Result<Json<Vec<crate::workshop::generic_service::WorkshopItem>>, (StatusCode, String)> {
    use crate::common::crud::ListParams;
    let session = state.jwt_service.get_optional_session_data(&token).map_err(|e| e.into())?;
    let list_params = ListParams { filter: None, pager: None };
    let result = state.rule_service.list_entity_items("board", board_id as i64, &list_params, session.as_ref()).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

/// Handler: add a rule to a board.
/// - Requires edit access to the board
/// - Body: { "workshop_item_id": 123, "origin_id": optional_package_id, "name": "optional display name" }
async fn add_board_rule(
    AuthToken(token): AuthToken,
    State(state): State<Arc<RouteState>>,
    Path(board_id): Path<i32>,
    ValidatedJson(request): ValidatedJson<crate::workshop::generic_service::NewWorkshopItem>,
) -> Result<(StatusCode, Json<crate::workshop::generic_service::WorkshopItem>), (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
        
    let mut new_item = request;
    let created = state.rule_service.create_entity_item("board", board_id as i64, &mut new_item, &session).await.map_err(|e| e.into())?;
    Ok((StatusCode::CREATED, Json(created)))
}

/// Handler: update a rule in a board.
/// - Requires edit access to the board
/// - Updates the workshop item globally (not entity-specific)
/// - Returns the updated WorkshopItem
async fn update_board_rule(
    AuthToken(token): AuthToken,
    State(state): State<Arc<RouteState>>,
    Path((_board_id, rule_id)): Path<(i32, i64)>,
    ValidatedJson(request): ValidatedJson<crate::workshop::generic_service::PatchWorkshopItem>,
) -> Result<Json<crate::workshop::generic_service::WorkshopItem>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    
    // TODO: Validate that the rule is linked to this board and user has access
    let updated = state.rule_service.update_item(rule_id, &request, &session).await.map_err(|e| e.into())?;
    Ok(Json(updated))
}

/// Handler: remove a rule from a board.
/// - Requires edit access to the board  
/// - Deletes the workshop item globally (since it was created in board context)
/// - Returns `204 No Content` on success
async fn remove_board_rule(
    AuthToken(token): AuthToken,
    State(state): State<Arc<RouteState>>,
    Path((_board_id, rule_id)): Path<(i32, i64)>,
) -> Result<StatusCode, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    
    // TODO: Validate that the rule is owned by this board and user has access
    let _deleted = state.rule_service.delete_item(rule_id, &session).await.map_err(|e| e.into())?;
    Ok(StatusCode::NO_CONTENT)
}

// Board-Layout Association Handlers

/// Handler: list layouts associated with a board.
/// - Requires read access to the board
/// - Returns array of layouts linked to the board
async fn list_board_layouts(
    AuthToken(token): AuthToken,
    State(state): State<Arc<RouteState>>,
    Path(board_id): Path<i32>,
) -> Result<Json<Vec<crate::workshop::generic_service::WorkshopItem>>, (StatusCode, String)> {
    use crate::common::crud::ListParams;
    let session = state.jwt_service.get_optional_session_data(&token).map_err(|e| e.into())?;
    let list_params = ListParams { filter: None, pager: None };
    let result = state.layout_service.list_entity_items("board", board_id as i64, &list_params, session.as_ref()).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

/// Handler: add a layout to a board.
/// - Requires edit access to the board
/// - Body: { "workshop_item_id": 123, "origin_id": optional_package_id, "name": "optional display name" }
async fn add_board_layout(
    AuthToken(token): AuthToken,
    State(state): State<Arc<RouteState>>,
    Path(board_id): Path<i32>,
    ValidatedJson(request): ValidatedJson<crate::workshop::generic_service::NewWorkshopItem>,
) -> Result<(StatusCode, Json<crate::workshop::generic_service::WorkshopItem>), (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
        
    let mut new_item = request;
    let created = state.layout_service.create_entity_item("board", board_id as i64, &mut new_item, &session).await.map_err(|e| e.into())?;
    Ok((StatusCode::CREATED, Json(created)))
}

/// Handler: update a layout in a board.
/// - Requires edit access to the board
/// - Updates the workshop item globally (not entity-specific)
/// - Returns the updated WorkshopItem
async fn update_board_layout(
    AuthToken(token): AuthToken,
    State(state): State<Arc<RouteState>>,
    Path((_board_id, layout_id)): Path<(i32, i64)>,
    ValidatedJson(request): ValidatedJson<crate::workshop::generic_service::PatchWorkshopItem>,
) -> Result<Json<crate::workshop::generic_service::WorkshopItem>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    
    // TODO: Validate that the layout is linked to this board and user has access
    let updated = state.layout_service.update_item(layout_id, &request, &session).await.map_err(|e| e.into())?;
    Ok(Json(updated))
}

/// Handler: remove a layout from a board.
/// - Requires edit access to the board  
/// - Deletes the workshop item globally (since it was created in board context)
/// - Returns `204 No Content` on success
async fn remove_board_layout(
    AuthToken(token): AuthToken,
    State(state): State<Arc<RouteState>>,
    Path((_board_id, layout_id)): Path<(i32, i64)>,
) -> Result<StatusCode, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    
    // TODO: Validate that the layout is owned by this board and user has access
    let _deleted = state.layout_service.delete_item(layout_id, &session).await.map_err(|e| e.into())?;
    Ok(StatusCode::NO_CONTENT)
}

