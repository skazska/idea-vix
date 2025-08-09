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
    jwt_adapter::JwtAdapter,
    package::package_service::{
        NewPackageItem, Package, PatchPackageItem
    },
    session::jwt
};

mod package_store;
mod package_service;

struct RouteState {
    service: package_service::PackageService,
    jwt_adapter: Arc<JwtAdapter>,
}

pub async fn get_router<'a>(connection: Arc<sqlx::Pool<sqlx::Sqlite>>, jwt_adapter: Arc<JwtAdapter>) -> axum::Router {
    let items_store = package_store::PackageStore::new(connection);
    let package_service = package_service::PackageService::new(items_store);

    let state= Arc::new(RouteState {
        service: package_service,
        jwt_adapter
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
    let session = jwt::optional_jwt(&state.jwt_adapter, &token).await;
    let result = state.service.get_items(&session).await.map_err(|e| e.into())?;

    Ok(Json(result))
}

// #[axum::debug_handler]
async fn add_item(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>, ValidatedJson(item): ValidatedJson<NewPackageItem>) -> Result<Json<Package>, (StatusCode, String)> {
    let session = jwt::jwt(&state.jwt_adapter, &token).await.map_err(|e| e.into())?;
    let result = state.service.add_item(&item, &session).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

// #[axum::debug_handler]
async fn get_item(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>, axum::extract::Path(id): Path<i32>) -> Result<Json<Package>, (StatusCode, String)> {
    let session = jwt::optional_jwt(&state.jwt_adapter, &token).await;
    let result = state.service.get_item(id, &session).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

// #[axum::debug_handler]
async fn update_item(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>, axum::extract::Path(id): Path<i32>, ValidatedJson(item): ValidatedJson<PatchPackageItem>) -> Result<Json<Package>, (StatusCode, String)> {
    let session = jwt::jwt(&state.jwt_adapter, &token).await.map_err(|e| e.into())?;
    let result = state.service.update_item(id, &item, &session).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

// #[axum::debug_handler]
async fn delete_item(AuthToken(token): AuthToken, State(state): State<Arc<RouteState>>, axum::extract::Path(id): Path<i32>) -> Result<Json<Package>, (StatusCode, String)> {
    let session = jwt::jwt(&state.jwt_adapter, &token).await.map_err(|e| e.into())?;
    let result = state.service.delete_item(id, &session).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

