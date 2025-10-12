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
//! async fn build_app(pool: Arc<sqlx::Pool<sqlx::Sqlite>>, jwt: Arc<SessionJWTService>) -> Router {
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
        access::{CommonItemAccess, ItemAccess, ItemAccessGrantDto, ItemRole, SqliteItemAccessQueries},
        crud::{CrudService, ListParams},
    },
    db::TransactionStarter,
    package::package_service::{
        NewPackageItem, Package, PackageService, PatchPackageItem
    },
    session::session_jwt::SessionJWTService,
    workshop::{
        entity_service::WorkshopEntityService,
        workshop_service::{ NewWorkshopItem, PatchWorkshopItem, WorkshopItem }, workshop_store::WorkshopStores
    }
};

mod package_store;
mod package_service;

const ENTITY: &str = "package";

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
pub fn get_router(
    transaction_starter: Arc<TransactionStarter>,
    jwt_service: Arc<SessionJWTService>,
    workshop_stores: Arc<WorkshopStores>,
) -> axum::Router {

    let access_store = Arc::new(SqliteItemAccessQueries::new(
        "package_access_roles",
        "package_id",
    ));

    let items_store = Arc::new(package_store::PackageStore::new());

    crate::resource_router!(
        get_items,
        add_item,
        get_item,
        update_item,
        delete_item,
        Arc::new((jwt_service.clone(), PackageService::new(
            transaction_starter.clone(),
            items_store,
            access_store.clone(),
        )))
    )
    .nest("/{id}", crate::access_router!(
        add_access,
        list_access,
        revoke_access,
        check_access,
        Arc::new((jwt_service.clone(), CommonItemAccess::new(transaction_starter.clone(), access_store.clone())))
    ))
    .nest("/{id}/workshop/shapes", crate::workshop_item_router!(
        list_package_items,
        add_package_item,
        update_package_item,
        remove_package_item,
        Arc::new((jwt_service.clone(), WorkshopEntityService::new(
            ENTITY,
            transaction_starter.clone(),
            workshop_stores.shape_store.clone(),
        )))
    ))
    .nest("/{id}/workshop/lines", crate::workshop_item_router!(
        list_package_items,
        add_package_item,
        update_package_item,
        remove_package_item,
        Arc::new((jwt_service.clone(), WorkshopEntityService::new(
            ENTITY,
            transaction_starter.clone(),
            workshop_stores.line_store.clone(),
        )))
    ))
    .nest("/{id}/workshop/rules", crate::workshop_item_router!(
        list_package_items,
        add_package_item,
        update_package_item,
        remove_package_item,
        Arc::new((jwt_service.clone(), WorkshopEntityService::new(
            ENTITY,
            transaction_starter.clone(),
            workshop_stores.rule_store.clone(),
        )))
    ))
    .nest("/{id}/workshop/layouts", crate::workshop_item_router!(
        list_package_items,
        add_package_item,
        update_package_item,
        remove_package_item,
        Arc::new((jwt_service.clone(), WorkshopEntityService::new(
            ENTITY,
            transaction_starter.clone(),
            workshop_stores.layout_store.clone(),
        )))
    ))
}

// #[axum::debug_handler]
/// Handler: list packages visible to the caller.
/// - Returns only public packages when no valid JWT is provided
/// - With a valid JWT, also returns packages accessible to the user
/// - Returns a JSON array of `Package`
async fn get_items(AuthToken(token): AuthToken, State(state): State<Arc<(Arc<SessionJWTService>, PackageService)>>) -> Result<Json<Vec<Package>>, (StatusCode, String)> {
    let (jwt_service, package_service) = state.as_ref();
    let session = jwt_service.get_optional_session_data(&token).map_err(|e| e.into())?;

    // Create default list parameters
    let lister = package_service::PackageLister {
        filter: None,
        pager: None,
    };

    let result = package_service.get_items(&lister, session.as_ref()).await.map_err(|e| e.into())?;

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
async fn add_item(AuthToken(token): AuthToken, State(state): State<Arc<(Arc<SessionJWTService>, PackageService)>>, ValidatedJson(mut item): ValidatedJson<NewPackageItem>) -> Result<(StatusCode, Json<Package>), (StatusCode, String)> {
    let (jwt_service, service) = state.as_ref();
    let session = jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let result = service.add_item(&mut item, &session).await.map_err(|e| e.into())?;
    Ok((StatusCode::CREATED, Json(result)))
}

// #[axum::debug_handler]
/// Handler: get a package by id.
/// - Returns public packages without authentication
/// - Private packages require access for the caller
/// - Returns `404` if the package is not accessible or does not exist
async fn get_item(AuthToken(token): AuthToken, State(state): State<Arc<(Arc<SessionJWTService>, PackageService)>>, axum::extract::Path(id): Path<i64>) -> Result<Json<Package>, (StatusCode, String)> {
    let (jwt_service, service) = state.as_ref();
    let session = jwt_service.get_optional_session_data(&token).map_err(|e| e.into())?;
    let result = service.get_item(id, session.as_ref()).await.map_err(|e| e.into())?;
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
async fn update_item(AuthToken(token): AuthToken, State(state): State<Arc<(Arc<SessionJWTService>, PackageService)>>, axum::extract::Path(id): Path<i64>, ValidatedJson(item): ValidatedJson<PatchPackageItem>) -> Result<Json<Package>, (StatusCode, String)> {
    let (jwt_service, service) = state.as_ref();
    let session = jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let result = service.update_item(id, &item, &session).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

// #[axum::debug_handler]
/// Handler: delete a package by id.
/// - Authenticates via JWT
/// - Requires `owner` or `manage` role
/// - Returns `200 Ok` and deleted package on success
async fn delete_item(AuthToken(token): AuthToken, State(state): State<Arc<(Arc<SessionJWTService>, PackageService)>>, axum::extract::Path(id): Path<i64>) -> Result<Json<Package>, (StatusCode, String)> {
    let (jwt_service, service) = state.as_ref();
    let session = jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    // perform deletion, ignore returned entity for API contract
    let result = service.delete_item(id, &session).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

// #[axum::debug_handler]
/// Handler: grant access to a package (owner only).
/// - Authenticates via JWT
/// - Requires `owner` role on the package
/// - Body: { address, role: "view"|"edit"|"manage" }
/// - Returns created mapping
async fn add_access(
    AuthToken(token): AuthToken,
    State(state): State<Arc<(Arc<SessionJWTService>, CommonItemAccess)>>,
    axum::extract::Path(id): Path<i64>,
    ValidatedJson(item): ValidatedJson<ItemAccessGrantDto>,
) -> Result<Json<ItemRole<i64>>, (StatusCode, String)> {
    let (jwt_service, service) = state.as_ref();
    let session = jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let role_dto = ItemRole {
        address: item.address,
        role: item.role,
        item_id: id,
    };
    let result = service.add_access_role(role_dto, &session).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

// #[axum::debug_handler]
/// Handler: revoke access to a package (owner only).
/// - Authenticates via JWT
/// - Requires `owner` role on the package
/// - Path: /{id}/access/{address}
async fn revoke_access(
    AuthToken(token): AuthToken,
    State(state): State<Arc<(Arc<SessionJWTService>, CommonItemAccess)>>,
    Path((id, address)): Path<(i64, String)>,
) -> Result<Json<Vec<ItemRole<i64>>>, (StatusCode, String)> {
    let (jwt_service, access_service) = state.as_ref();
    let session = jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let result = access_service.revoke_access_roles(id, address, &session).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

// #[axum::debug_handler]
/// Handler: list access mappings for a package (owner only).
/// - Authenticates via JWT
/// - Requires `owner` role on the package
/// - Returns array of mappings
async fn list_access(
    AuthToken(token): AuthToken,
    State(state): State<Arc<(Arc<SessionJWTService>, CommonItemAccess)>>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<ItemRole<i64>>>, (StatusCode, String)> {
    let (jwt_service, access_service) = state.as_ref();
    let session = jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let result = access_service.list_access_roles(id, &session).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

// #[axum::debug_handler]
/// Handler: returns access to package for address
/// - Authenticates via JWT
/// - Returns roles of invitations of `address`
/// - Returns `404` if the package is not accessible or does not exist
async fn check_access(
    AuthToken(token): AuthToken,
    State(state): State<Arc<(Arc<SessionJWTService>, CommonItemAccess)>>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<String>>, (StatusCode, String)> {
    let (jwt_service, access_service) = state.as_ref();
    let session = jwt_service.get_optional_session_data(&token).map_err(|e| e.into())?;
    let result = match &session {
        Some(s) => {
            let roles = access_service.get_my_access_roles(id, &s).await.map_err(|e| e.into())?;
            roles.into_iter().map(|r| r.into()).collect::<Vec<String>>()
        },
        None => vec![],
    };
    Ok(Json(result))
}

/// Handler: list items associated with a package.
/// - Requires read access to the package
/// - Returns array of items linked to the package
async fn list_package_items(
    AuthToken(token): AuthToken,
    State(state): State<Arc<(Arc<SessionJWTService>, WorkshopEntityService)>>,
    Path(package_id): Path<i32>,
) -> Result<Json<Vec<WorkshopItem>>, (StatusCode, String)> {
    let (jwt_service, service) = state.as_ref();
    let session = jwt_service.get_optional_session_data(&token).map_err(|e| e.into())?;
    let list_params = ListParams { filter: None, pager: None };
    let result = service.list_entity_items( package_id as i64, &list_params, session.as_ref()).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

/// Handler: add a shape to a package.
/// - Requires edit access to the package
/// - Body: { "workshop_item_id": 123 }
async fn add_package_item(
    AuthToken(token): AuthToken,
    State(state): State<Arc<(Arc<SessionJWTService>, WorkshopEntityService)>>,
    Path(package_id): Path<i64>,
    ValidatedJson(request): ValidatedJson<NewWorkshopItem>,
) -> Result<(StatusCode, Json<WorkshopItem>), (StatusCode, String)> {
    let (jwt_service, service) = state.as_ref();
    let session = jwt_service.get_session_data(&token).map_err(|e| e.into())?;

    let mut new_item = request;
    let created = service.create_entity_item(package_id, &mut new_item, &session).await.map_err(|e| e.into())?;
    Ok((StatusCode::CREATED, Json(created)))
}

/// Handler: update a shape in a package.
/// - Requires edit access to the package
/// - Updates the workshop item globally (not entity-specific)
/// - Returns the updated WorkshopItem
async fn update_package_item(
    AuthToken(token): AuthToken,
    State(state): State<Arc<(Arc<SessionJWTService>, WorkshopEntityService)>>,
    Path((package_id, item_id)): Path<(i64, i64)>,
    ValidatedJson(request): ValidatedJson<PatchWorkshopItem>,
) -> Result<Json<WorkshopItem>, (StatusCode, String)> {
    let (jwt_service, service) = state.as_ref();
    let session = jwt_service.get_session_data(&token).map_err(|e| e.into())?;

    // TODO: Validate that the item is linked to this package and user has access
    let updated = service.update_entity_item(package_id, item_id, &request, &session).await.map_err(|e| e.into())?;
    Ok(Json(updated))
}

/// Handler: remove a shape from a package.
/// - Requires edit access to the package  
/// - Deletes the workshop item globally (since it was created in package context)
/// - Returns `204 No Content` on success
async fn remove_package_item(
    AuthToken(token): AuthToken,
    State(state): State<Arc<(Arc<SessionJWTService>, WorkshopEntityService)>>,
    Path((package_id, item_id)): Path<(i64, i64)>,
) -> Result<StatusCode, (StatusCode, String)> {
    let (jwt_service, service) = state.as_ref();
    let session = jwt_service.get_session_data(&token).map_err(|e| e.into())?;

    // TODO: Validate that the item is owned by this package and user has access
    let _deleted = service.delete_entity_item(package_id, item_id, &session).await.map_err(|e| e.into())?;
    Ok(StatusCode::NO_CONTENT)
}
