//! Workshop module: API routes for workshop elements (shapes, lines, rules, layouts).
//!
//! The workshop provides global read-only access to diagram element blueprints for discovery.
//! Workshop items are created and managed within the context of packages and boards.

use std::sync::Arc;
use crate::common::crud::{CrudService, ListParams};
use crate::db::TransactionStarter;
use crate::session::session_jwt::SessionJWTService;
use crate::workshop::workshop_service::{
    WorkshopService,
    Shape, Line, Rule, Layout,
};
use crate::common::workshop_store::{WorkshopStores};
use axum::extract::Query;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::Deserialize;

pub mod workshop_service;
pub mod entity_service;
/// Shared state for the workshop routes.
struct RouteState {
    shape_service: WorkshopService,
    line_service: WorkshopService,
    rule_service: WorkshopService,
    layout_service: WorkshopService,
}

/// Create the workshop router with read-only routes for global discovery.
pub fn get_router(
    transaction_starter: Arc<TransactionStarter>,
    _jwt_service: Arc<SessionJWTService>,
    workshop_stores: Arc<WorkshopStores>,
) -> Router {
    let shape_service = WorkshopService::new(transaction_starter.clone(), workshop_stores.shape_store.clone());
    let line_service = WorkshopService::new(transaction_starter.clone(), workshop_stores.line_store.clone());
    let rule_service = WorkshopService::new(transaction_starter.clone(), workshop_stores.rule_store.clone());
    let layout_service = WorkshopService::new(transaction_starter.clone(), workshop_stores.layout_store.clone());

    let state = Arc::new(RouteState {
        shape_service,
        line_service,
        rule_service,
        layout_service,
    });

    Router::new()
        // Global workshop shape routes - read-only for discovery
        .route("/shapes", get(list_shapes))
        .route("/shapes/{id}", get(get_shape))
        .route("/shapes/by-slug/{slug}", get(get_shape_by_slug))
        
        // Global workshop line routes - read-only for discovery
        .route("/lines", get(list_lines))
        .route("/lines/{id}", get(get_line))
        .route("/lines/by-slug/{slug}", get(get_line_by_slug))
        
        // Global workshop rule routes - read-only for discovery
        .route("/rules", get(list_rules))
        .route("/rules/{id}", get(get_rule))
        .route("/rules/by-slug/{slug}", get(get_rule_by_slug))
        
        // Global workshop layout routes - read-only for discovery
        .route("/layouts", get(list_layouts))
        .route("/layouts/{id}", get(get_layout))
        .route("/layouts/by-slug/{slug}", get(get_layout_by_slug))
        .with_state(state)
}

#[derive(Deserialize, Debug)]
pub struct SearchParams {
    pub ids: Option<Vec<i64>>,
    pub search: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<i64>,
}

// Shape handlers

/// Handler: list all shapes in the global workshop.
async fn list_shapes(State(state): State<Arc<RouteState>>, Query(params): Query<SearchParams>) -> Result<Json<Vec<Shape>>, (StatusCode, String)> {
    let lister = ListParams {
        filter: Some(crate::common::crud::ListFilter {
            filter: None,
            ids: params.ids,
            search: params.search,
        }),
        pager: Some(crate::common::crud::QueryPager {
            limit: params.limit.unwrap_or(50),
            offset: params.offset,
        }),
    };

    let shapes = state.shape_service.get_items(&lister, None).await.map_err(|e| e.into())?;
    Ok(Json(shapes))
}

/// Handler: get a shape by id.
async fn get_shape(State(state): State<Arc<RouteState>>, Path(id): Path<i64>) -> Result<Json<Shape>, (StatusCode, String)> {
    let shape = state.shape_service.get_item(id, None).await.map_err(|e| e.into())?;
    Ok(Json(shape))
}

/// Handler: get a shape by semantic identifier (slug).
async fn get_shape_by_slug(State(state): State<Arc<RouteState>>, Path(slug): Path<String>) -> Result<Json<Shape>, (StatusCode, String)> {
    let shape = state.shape_service.get_item_by_slug(&slug).await.map_err(|e| e.into())?;
    Ok(Json(shape))
}

// Line handlers

/// Handler: list all lines in the global workshop.
async fn list_lines(State(state): State<Arc<RouteState>>, Query(params): Query<SearchParams>) -> Result<Json<Vec<Line>>, (StatusCode, String)> {
    let lister = ListParams {
        filter: Some(crate::common::crud::ListFilter {
            filter: None,
            ids: params.ids,
            search: params.search,
        }),
        pager: Some(crate::common::crud::QueryPager {
            limit: params.limit.unwrap_or(50),
            offset: params.offset,
        }),
    };

    let lines = state.line_service.get_items(&lister, None).await.map_err(|e| e.into())?;
    Ok(Json(lines))
}

/// Handler: get a line by id.
async fn get_line(State(state): State<Arc<RouteState>>, Path(id): Path<i64>) -> Result<Json<Line>, (StatusCode, String)> {
    let line = state.line_service.get_item(id, None).await.map_err(|e| e.into())?;
    Ok(Json(line))
}

/// Handler: get a line by semantic identifier (slug).
async fn get_line_by_slug(State(state): State<Arc<RouteState>>, Path(slug): Path<String>) -> Result<Json<Line>, (StatusCode, String)> {
    let line = state.line_service.get_item_by_slug(&slug).await.map_err(|e| e.into())?;
    Ok(Json(line))
}

// Rule handlers

/// Handler: list all rules in the global workshop.
async fn list_rules(State(state): State<Arc<RouteState>>, Query(params): Query<SearchParams>) -> Result<Json<Vec<Rule>>, (StatusCode, String)> {
    let lister = ListParams {
        filter: Some(crate::common::crud::ListFilter {
            filter: None,
            ids: params.ids,
            search: params.search,
        }),
        pager: Some(crate::common::crud::QueryPager {
            limit: params.limit.unwrap_or(50),
            offset: params.offset,
        }),
    };

    let rules = state.rule_service.get_items(&lister, None).await.map_err(|e| e.into())?;
    Ok(Json(rules))
}

/// Handler: get a rule by id.
async fn get_rule(State(state): State<Arc<RouteState>>, Path(id): Path<i64>) -> Result<Json<Rule>, (StatusCode, String)> {
    let rule = state.rule_service.get_item(id, None).await.map_err(|e| e.into())?;
    Ok(Json(rule))
}

/// Handler: get a rule by semantic identifier (slug).
async fn get_rule_by_slug(State(state): State<Arc<RouteState>>, Path(slug): Path<String>) -> Result<Json<Rule>, (StatusCode, String)> {
    let rule = state.rule_service.get_item_by_slug(&slug).await.map_err(|e| e.into())?;
    Ok(Json(rule))
}

// Layout handlers

/// Handler: list all layouts in the global workshop.
async fn list_layouts(State(state): State<Arc<RouteState>>, Query(params): Query<SearchParams>) -> Result<Json<Vec<Layout>>, (StatusCode, String)> {
    let lister = ListParams {
        filter: Some(crate::common::crud::ListFilter {
            filter: None,
            ids: params.ids,
            search: params.search,
        }),
        pager: Some(crate::common::crud::QueryPager {
            limit: params.limit.unwrap_or(50),
            offset: params.offset,
        }),
    };

    let layouts = state.layout_service.get_items(&lister, None).await.map_err(|e| e.into())?;
    Ok(Json(layouts))
}

/// Handler: get a layout by id.
async fn get_layout(State(state): State<Arc<RouteState>>, Path(id): Path<i64>) -> Result<Json<Layout>, (StatusCode, String)> {
    let layout = state.layout_service.get_item(id, None).await.map_err(|e| e.into())?;
    Ok(Json(layout))
}

/// Handler: get a layout by semantic identifier (slug).
async fn get_layout_by_slug(State(state): State<Arc<RouteState>>, Path(slug): Path<String>) -> Result<Json<Layout>, (StatusCode, String)> {
    let layout = state.layout_service.get_item_by_slug(&slug).await.map_err(|e| e.into())?;
    Ok(Json(layout))
}