/// Feature unit of modules for packages in the application.
/// List of packages
/// CRUD operations for packages:
/// - create package
/// - read package
/// - update package
/// - delete package
/// - manage package items [TODO]

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

struct RouteState {
    service: package_service::PackageService,
    jwt_service: Arc<SessionJWTService>,
}

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
async fn get_items(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>) -> Result<Json<Vec<Package>>, (StatusCode, String)> {
    let session = state.jwt_service.get_optional_session_data(&token).map_err(|e| e.into())?;
    let result = state.service.get_items(&session).await.map_err(|e| e.into())?;

    Ok(Json(result))
}

// #[axum::debug_handler]
async fn add_item(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>, ValidatedJson(item): ValidatedJson<NewPackageItem>) -> Result<(StatusCode, Json<Package>), (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let result = state.service.add_item(&item, &session).await.map_err(|e| e.into())?;
    Ok((StatusCode::CREATED, Json(result)))
}

// #[axum::debug_handler]
async fn get_item(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>, axum::extract::Path(id): Path<i32>) -> Result<Json<Package>, (StatusCode, String)> {
    let session = state.jwt_service.get_optional_session_data(&token).map_err(|e| e.into())?;
    let result = state.service.get_item(id, &session).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

// #[axum::debug_handler]
async fn update_item(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>, axum::extract::Path(id): Path<i32>, ValidatedJson(item): ValidatedJson<PatchPackageItem>) -> Result<Json<Package>, (StatusCode, String)> {
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

