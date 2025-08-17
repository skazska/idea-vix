//! Package feature module: HTTP routes for listing and managing packages.
//!
//! Exposes REST-style endpoints mounted under the module root (typically `/api/package`).
//! Supported operations:
//! - List packages (public for unauthenticated users; public + permitted for authenticated users)
//! - Create a package (authenticated)
//! - Read a specific package (public or permitted)
//! - Update a package (owner or manage)
//! - Delete a package (owner or manage)
//!
//! Prerequisites:
//! - A configured SQLx Sqlite pool
//! - An instance of `SessionJWTService` to decode optional/required JWT tokens
//!
//! Example (mounting in the main router):
//! ```ignore
//! use std::sync::Arc;
//! use axum::Router;
//! use crate::package; // adjust path to where this module is located
//! 
//! async fn build_app(pool: Arc<sqlx::Pool<sqlx::Sqlite>>, jwt: Arc<crate::session::session_jwt::SessionJWTService>) -> Router {
//!     Router::new()
//!         .nest("/api/package", package::get_router(pool, jwt))
//! }
//! ```

use std::sync::{Arc};

use axum::{
    extract::{ Path, State }, http::StatusCode, routing::{delete, get, post, put}, Json, Router
 };

use crate::{
    api::{deserialize::AuthToken, validation::ValidatedJson},
    package::package_service::{
        NewPackageItem, Package, PatchPackageItem
    },
    session::session_jwt::SessionJWTService
};

mod package_store;
mod package_service;

/// Shared state for the package routes.
///
/// Holds the service layer and JWT service required by handlers.
struct RouteState {
    service: package_service::PackageService,
    jwt_service: Arc<SessionJWTService>,
}

/// Build a router with all package endpoints.
///
/// Routes provided (relative to the mounted prefix):
/// - `GET /` – List packages visible to the current (optional) session
/// - `POST /` – Create a new package (requires authenticated session)
/// - `GET /{id}` – Get a package by id; accessible if public or user has access
/// - `PUT /{id}` – Update a package (owner/manage)
/// - `DELETE /{id}` – Delete a package (owner/manage)
///
/// Parameters:
/// - `connection`: SQLx Sqlite pool wrapped in `Arc`
/// - `jwt_service`: JWT service wrapped in `Arc`
///
/// Returns an Axum `Router` ready to be nested under a path like `/api/package`.
pub async fn get_router<'a>(connection: Arc<sqlx::Pool<sqlx::Sqlite>>, jwt_service: Arc<SessionJWTService>) -> axum::Router {
    let items_store = package_store::PackageStore::new(connection);
    let package_service = package_service::PackageService::new(items_store);

    let state= Arc::new(RouteState {
        service: package_service,
        jwt_service
    });

    Router::new()
        .route("/", get(get_items))
        .route("/", post(add_item))
        .route("/{id}", get(get_item))
        .route("/{id}", put(update_item))
        .route("/{id}", delete(delete_item))
        .with_state(state)
}

// #[axum::debug_handler]
/// Handler: list packages visible to the caller.
/// - Returns only public packages when no valid JWT is provided
/// - With a valid JWT, also returns packages accessible to the user
/// - Returns a JSON array of `Package`
async fn get_items(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>) -> Result<Json<Vec<Package>>, (StatusCode, String)> {
    let session = state.jwt_service.get_optional_session_data(&token).map_err(|e| e.into())?;
    let result = state.service.get_items(&session).await.map_err(|e| e.into())?;

    Ok(Json(result))
}

// #[axum::debug_handler]
/// Handler: create a new package.
/// - Authenticates via JWT
/// - Validates payload (`name` 3..=100; `description` <= 500; `icon` <= 255)
/// - Returns `201 Created` with the created `Package`
///
/// Example payload:
/// ```json
/// { "name": "My Package", "description": "optional", "is_public": true }
/// ```
async fn add_item(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>, ValidatedJson(item): ValidatedJson<NewPackageItem>) -> Result<(StatusCode, Json<Package>), (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let result = state.service.add_item(&item, &session).await.map_err(|e| e.into())?;
    Ok((StatusCode::CREATED, Json(result)))
}

// #[axum::debug_handler]
/// Handler: get a package by id.
/// - Returns public packages without authentication
/// - Private packages require access for the caller
/// - Returns `404` if the package is not accessible or does not exist
async fn get_item(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>, axum::extract::Path(id): Path<i32>) -> Result<Json<Package>, (StatusCode, String)> {
    let session = state.jwt_service.get_optional_session_data(&token).map_err(|e| e.into())?;
    let result = state.service.get_item(id, &session).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

// #[axum::debug_handler]
/// Handler: update an existing package.
/// - Authenticates via JWT
/// - Requires `owner` or `manage` role
/// - Supports partial updates (omit fields to leave unchanged)
/// - To clear `description` or `icon`, send the field with `null`
/// - Returns the updated `Package`
///
/// Example payloads:
/// - Update name only: `{ "name": "New Name" }`
/// - Clear description: `{ "description": null }`
async fn update_item(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>, axum::extract::Path(id): Path<i32>, ValidatedJson(item): ValidatedJson<PatchPackageItem>) -> Result<Json<Package>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let result = state.service.update_item(id, &item, &session).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

// #[axum::debug_handler]
/// Handler: delete a package by id.
/// - Authenticates via JWT
/// - Requires `owner` or `manage` role
/// - Returns `204 No Content` on success
async fn delete_item(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>, axum::extract::Path(id): Path<i32>) -> Result<StatusCode, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    // perform deletion, ignore returned entity for API contract
    let _ = state.service.delete_item(id, &session).await.map_err(|e| e.into())?;
    Ok(StatusCode::NO_CONTENT)
}

