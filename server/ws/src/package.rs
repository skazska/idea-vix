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
    extract::{ Path, State }, http::StatusCode, Json
 };

use crate::{
    api::{deserialize::AuthToken, validation::ValidatedJson},
    common::{
        access::{self, ItemAccess, ItemRole, ItemRoleDto, SqliteItemAccessQueries},
        crud::CrudService,
        workshop::{
            shape_service::{ShapeService},
            shape_store::ShapeStore,
        }
    },
    db::TransactionStarter,
    package::package_service::{
        NewPackageAccessItem, NewPackageItem, Package, PatchPackageItem
    },
    session::session_jwt::SessionJWTService
};

mod package_store;
mod package_service;

/// Shared state for the package routes.
///
/// Holds the service layer and JWT service required by handlers.
struct RouteState {
    access_service: access::CommonItemAccess,
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
/// - `POST /{id}/access` – Grant access to an address (owner)
/// - `DELETE /{id}/access/{address}` – Revoke access for an address (owner)
/// - `GET /{id}/access` – List all access mappings (owner)
///
/// Parameters:
/// - `connection`: SQLx Sqlite pool wrapped in `Arc`
/// - `jwt_service`: JWT service wrapped in `Arc`
///
/// Returns an Axum `Router` ready to be nested under a path like `/api/package`.
pub fn get_router<'a>(
    transaction_starter: Arc<TransactionStarter>,
    jwt_service: Arc<SessionJWTService>,
) -> axum::Router {

    let shape_store = Arc::new(ShapeStore::new());
    let access_store = Arc::new(SqliteItemAccessQueries::new(
        "package_access_roles",
        "package_id",
    ));
    let access_service = access::CommonItemAccess::new(transaction_starter.clone(), access_store.clone());

    let items_store = Arc::new(package_store::PackageStore::new());
    let shape_service = Arc::new(ShapeService::new(transaction_starter.clone(), shape_store));
    let package_service = package_service::PackageService::new(transaction_starter, items_store, shape_service, access_store);

    let state= Arc::new(RouteState {
        access_service,
        service: package_service,
        jwt_service
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
    // .route("/{id}/shapes", axum::routing::get(list_package_shapes))
    // .route("/{id}/shapes", axum::routing::post(add_package_shape))
    // .route("/{id}/shapes/{shape_id}", axum::routing::delete(remove_package_shape))
    .with_state(state)
}

// #[axum::debug_handler]
/// Handler: list packages visible to the caller.
/// - Returns only public packages when no valid JWT is provided
/// - With a valid JWT, also returns packages accessible to the user
/// - Returns a JSON array of `Package`
async fn get_items(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>) -> Result<Json<Vec<Package>>, (StatusCode, String)> {
    let session = state.jwt_service.get_optional_session_data(&token).map_err(|e| e.into())?;
    
    // Create default list parameters
    let lister = package_service::PackageLister {
        filter: None,
        pager: None,
    };

    let result = state.service.get_items(&lister, session.as_ref()).await.map_err(|e| e.into())?;

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
async fn get_item(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>, axum::extract::Path(id): Path<i64>) -> Result<Json<Package>, (StatusCode, String)> {
    let session = state.jwt_service.get_optional_session_data(&token).map_err(|e| e.into())?;
    let result = state.service.get_item(id, session.as_ref()).await.map_err(|e| e.into())?;
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
async fn update_item(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>, axum::extract::Path(id): Path<i64>, ValidatedJson(item): ValidatedJson<PatchPackageItem>) -> Result<Json<Package>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let result = state.service.update_item(id, &item, &session).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

// #[axum::debug_handler]
/// Handler: delete a package by id.
/// - Authenticates via JWT
/// - Requires `owner` or `manage` role
/// - Returns `204 No Content` on success
async fn delete_item(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>, axum::extract::Path(id): Path<i64>) -> Result<StatusCode, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    // perform deletion, ignore returned entity for API contract
    let _ = state.service.delete_item(id, &session).await.map_err(|e| e.into())?;
    Ok(StatusCode::NO_CONTENT)
}

// #[axum::debug_handler]
/// Handler: grant access to a package (owner only).
/// - Authenticates via JWT
/// - Requires `owner` role on the package
/// - Body: { address, role: "view"|"edit"|"manage" }
/// - Returns created mapping
async fn add_access(
    AuthToken(token): AuthToken,
    State(state): State<Arc<RouteState>>,
    axum::extract::Path(id): Path<i64>,
    ValidatedJson(item): ValidatedJson<NewPackageAccessItem<i64>>,
) -> Result<Json<ItemRole<i64>>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let role_dto = ItemRoleDto {
        address: item.address,
        role: item.role,
        item_id: id,
    };
    let result = state.access_service.add_access_role(role_dto, &session).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

// #[axum::debug_handler]
/// Handler: revoke access to a package (owner only).
/// - Authenticates via JWT
/// - Requires `owner` role on the package
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
/// Handler: list access mappings for a package (owner only).
/// - Authenticates via JWT
/// - Requires `owner` role on the package
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
            let roles = state.access_service.list_access_roles(id, &s).await.map_err(|e| e.into())?;

            roles.into_iter().map(|r| r.role.into()).collect::<Vec<String>>()
        },
        None => vec![],
    };
    Ok(Json(result))
}

// // Package-Shape Association Handlers

// /// Handler: list shapes associated with a package.
// /// - Requires read access to the package
// /// - Returns array of `PackageShape`
// async fn list_package_shapes(
//     AuthToken(token): AuthToken,
//     State(state): State<Arc<RouteState>>,
//     Path(package_id): Path<i32>,
// ) -> Result<Json<Vec<PackageShape>>, (StatusCode, String)> {
//     let session = state.jwt_service.get_optional_session_data(&token).map_err(|e| e.into())?;
//     let result = state.service.list_shapes(package_id, &session).await.map_err(|e| e.into())?;
//     Ok(Json(result))
// }

// /// Handler: add a shape to a package.
// /// - Requires edit access to the package
// /// - Body: { "shape_id": 123, "name": "optional display name" }
// /// - Returns the created `PackageShape`
// async fn add_package_shape(
//     AuthToken(token): AuthToken,
//     State(state): State<Arc<RouteState>>,
//     Path(package_id): Path<i32>,
//     ValidatedJson(item): ValidatedJson<AddShapeToPackageItem>,
// ) -> Result<(StatusCode, Json<PackageShape>), (StatusCode, String)> {
//     let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
//     let result = state.service.add_shape(package_id, &item, &session).await.map_err(|e| e.into())?;
//     Ok((StatusCode::CREATED, Json(result)))
// }


// /// Handler: remove a shape from a package.
// /// - Requires edit access to the package
// /// - Returns `204 No Content` on success
// async fn remove_package_shape(
//     AuthToken(token): AuthToken,
//     State(state): State<Arc<RouteState>>,
//     Path((package_id, slug)): Path<(i32, String)>,
// ) -> Result<StatusCode, (StatusCode, String)> {
//     let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
//     state.service.remove_shape(package_id, &slug, &session).await.map_err(|e| e.into())?;
//     Ok(StatusCode::NO_CONTENT)
// }
