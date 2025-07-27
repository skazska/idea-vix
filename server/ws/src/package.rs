use std::sync::{Arc};

use axum::{ extract::{ State, Path }, http::StatusCode, routing::{delete, get, post, put}, Json, Router };

use crate::{api::{validation::ValidatedJson}, package::package_types::{NewPackageItem, Package, PatchPackageItem}};

pub mod package_types;
mod package_store;
mod package_service;

struct RouteState {
    service: package_service::PackageService,
}

pub async fn get_router<'a>(connection: Arc<sqlx::Pool<sqlx::Sqlite>>) -> axum::Router {
    let items_store = package_store::PackageStore::new(connection);
    let package_service = package_service::PackageService::new(items_store);

    let state= Arc::new(RouteState {
        service: package_service,
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
async fn get_items(State(state): State<Arc<RouteState>>) -> Result<Json<Vec<Package>>, (StatusCode, String)> {
    let result = state.service.get_items().await.map_err(|e| e.into())?;

    Ok(Json(result))
}

// #[axum::debug_handler]
async fn add_item(State(state): State<Arc<RouteState>>, ValidatedJson(item): ValidatedJson<NewPackageItem>) -> Result<Json<Package>, (StatusCode, String)> {
    let result = state.service.add_item(&item).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

// #[axum::debug_handler]
async fn get_item(State(state): State<Arc<RouteState>>, axum::extract::Path(id): Path<i32>) -> Result<Json<Package>, (StatusCode, String)> {
    let result = state.service.get_item(id).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

// #[axum::debug_handler]
async fn update_item(State(state): State<Arc<RouteState>>, axum::extract::Path(id): Path<i32>, ValidatedJson(item): ValidatedJson<PatchPackageItem>) -> Result<Json<Package>, (StatusCode, String)> {
    let result = state.service.update_item(id, &item).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

// #[axum::debug_handler]
async fn delete_item(State(state): State<Arc<RouteState>>, axum::extract::Path(id): Path<i32>) -> Result<Json<Package>, (StatusCode, String)> {
    let result = state.service.delete_item(id).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

