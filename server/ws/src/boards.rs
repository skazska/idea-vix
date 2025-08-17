//! Board feature module: HTTP routes for listing and managing boards.
//!
//! Exposes REST-style endpoints mounted under the module root (typically `/api/board`).
//! Supported operations:
//! - List boards (public for unauthenticated users; public + permitted for authenticated users)
//! - Create a board (authenticated)
//! - Read a specific board (public or permitted)
//! - Update a board (owner or manage)
//! - Delete a board (owner or manage)
//! - Manage access to a board (owner): grant/revoke/list
//!
//! Prerequisites:
//! - A configured SQLx Sqlite pool
//! - An instance of `SessionJWTService` to decode optional/required JWT tokens

use std::sync::{Arc};

use axum::{ extract::{ Path, State }, http::StatusCode, routing::{delete, get, post, put}, Json, Router };

use crate::{
    api::{deserialize::AuthToken, validation::ValidatedJson},
    boards::board_service::{Board, NewBoardItem, PatchBoardItem, NewBoardAccessItem, BoardAccessRoles},
    session::session_jwt::SessionJWTService,
};

mod board_store;
mod board_service;

struct RouteState {
    service: board_service::BoardService,
    jwt_service: Arc<SessionJWTService>,
}

pub async fn get_router<'a>(connection: Arc<sqlx::Pool<sqlx::Sqlite>>, jwt_service: Arc<SessionJWTService>) -> axum::Router {
    let items_store = board_store::BoardStore::new(connection);
    let board_service = board_service::BoardService::new(items_store);

    let state= Arc::new(RouteState {
        service: board_service,
        jwt_service
    });

    Router::new()
        .route("/", get(get_items))
        .route("/", post(add_item))
        .route("/{id}", get(get_item))
        .route("/{id}", put(update_item))
    .route("/{id}", delete(delete_item))
    .route("/{id}/access", post(add_access))
    .route("/{id}/access", get(list_access))
    .route("/{id}/access/{address}", delete(revoke_access))
        .with_state(state)
}

// #[axum::debug_handler]
async fn get_items(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>) -> Result<Json<Vec<Board>>, (StatusCode, String)> {
    let session = state.jwt_service.get_optional_session_data(&token).map_err(|e| e.into())?;
    let result = state.service.get_items(&session).await.map_err(|e| e.into())?;

    Ok(Json(result))
}

// #[axum::debug_handler]
async fn add_item(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>, ValidatedJson(item): ValidatedJson<NewBoardItem>) -> Result<(StatusCode, Json<Board>), (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let result = state.service.add_item(&item, &session).await.map_err(|e| e.into())?;
    Ok((StatusCode::CREATED, Json(result)))
}

// #[axum::debug_handler]
async fn get_item(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>, axum::extract::Path(id): Path<i32>) -> Result<Json<Board>, (StatusCode, String)> {
    let session = state.jwt_service.get_optional_session_data(&token).map_err(|e| e.into())?;
    let result = state.service.get_item(id, &session).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

// #[axum::debug_handler]
async fn update_item(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>, axum::extract::Path(id): Path<i32>, ValidatedJson(item): ValidatedJson<PatchBoardItem>) -> Result<Json<Board>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let result = state.service.update_item(id, &item, &session).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

// #[axum::debug_handler]
async fn delete_item(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>, axum::extract::Path(id): Path<i32>) -> Result<StatusCode, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    // perform deletion, ignore returned entity for API contract
    let _ = state.service.delete_item(id, &session).await.map_err(|e| e.into())?;
    Ok(StatusCode::NO_CONTENT)
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
    axum::extract::Path(id): Path<i32>,
    ValidatedJson(item): ValidatedJson<NewBoardAccessItem>,
) -> Result<Json<BoardAccessRoles>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let result = state.service.add_access_role(id, &item, &session).await.map_err(|e| e.into())?;
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
    Path((id, address)): Path<(i32, String)>,
) -> Result<Json<BoardAccessRoles>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let result = state.service.revoke_access_role(id, &address, &session).await.map_err(|e| e.into())?;
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
    Path(id): Path<i32>,
) -> Result<Json<Vec<BoardAccessRoles>>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let result = state.service.list_access_roles(id, &session).await.map_err(|e| e.into())?;
    Ok(Json(result))
}
