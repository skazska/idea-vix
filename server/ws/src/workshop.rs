//! Workshop module: API routes for workshop elements (shapes, lines, rules, layouts).
//!
//! The workshop provides global management of diagram element blueprints.

use std::sync::Arc;
use crate::common::crud::{CrudService, ListParams};
use crate::db::TransactionStarter;
use crate::api::{deserialize::AuthToken, validation::ValidatedJson};
use crate::session::session_jwt::SessionJWTService;
use crate::workshop::generic_service::{
    WorkshopService,
    Shape, NewShapeItem, PatchShapeItem,
    Line, NewLineItem, PatchLineItem,
    Rule, NewRuleItem, PatchRuleItem,
    Layout, NewLayoutItem, PatchLayoutItem,
};
use crate::common::workshop_store::{WorkshopItemType, WorkshopStore};
use axum::extract::Query;
use axum::routing::{put, post, delete};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get},
    Router,
};
use serde::Deserialize;

pub mod service;
pub mod generic_service;

/// Shared state for the workshop routes.
struct RouteState {
    shape_service: WorkshopService,
    line_service: WorkshopService,
    rule_service: WorkshopService,
    layout_service: WorkshopService,
    jwt_service: Arc<SessionJWTService>,
}

/// Create the workshop router with all workshop item type routes.
pub fn get_router(
    transaction_starter: Arc<TransactionStarter>,
    jwt_service: Arc<SessionJWTService>,
) -> Router {
    let shape_store = Arc::new(WorkshopStore::new(WorkshopItemType::Shape));
    let line_store = Arc::new(WorkshopStore::new(WorkshopItemType::Line));
    let rule_store = Arc::new(WorkshopStore::new(WorkshopItemType::Rule));
    let layout_store = Arc::new(WorkshopStore::new(WorkshopItemType::Layout));
    
    let shape_service = WorkshopService::new(transaction_starter.clone(), shape_store);
    let line_service = WorkshopService::new(transaction_starter.clone(), line_store);
    let rule_service = WorkshopService::new(transaction_starter.clone(), rule_store);
    let layout_service = WorkshopService::new(transaction_starter.clone(), layout_store);

    let state = Arc::new(RouteState {
        shape_service,
        line_service,
        rule_service,
        layout_service,
        jwt_service,
    });

    Router::new()
        // Global workshop shape routes
        .route("/shapes", get(list_shapes))
        .route("/shapes", post(create_shape))
        .route("/shapes/{id}", get(get_shape))
        .route("/shapes/{id}", put(update_shape))
        .route("/shapes/{id}", delete(delete_shape))
        .route("/shapes/by-slug/{slug}", get(get_shape_by_slug))
        
        // Global workshop line routes
        .route("/lines", get(list_lines))
        .route("/lines", post(create_line))
        .route("/lines/{id}", get(get_line))
        .route("/lines/{id}", put(update_line))
        .route("/lines/{id}", delete(delete_line))
        .route("/lines/by-slug/{slug}", get(get_line_by_slug))
        
        // Global workshop rule routes
        .route("/rules", get(list_rules))
        .route("/rules", post(create_rule))
        .route("/rules/{id}", get(get_rule))
        .route("/rules/{id}", put(update_rule))
        .route("/rules/{id}", delete(delete_rule))
        .route("/rules/by-slug/{slug}", get(get_rule_by_slug))
        
        // Global workshop layout routes
        .route("/layouts", get(list_layouts))
        .route("/layouts", post(create_layout))
        .route("/layouts/{id}", get(get_layout))
        .route("/layouts/{id}", put(update_layout))
        .route("/layouts/{id}", delete(delete_layout))
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

/// Handler: create a new shape.
async fn create_shape(
    State(state): State<Arc<RouteState>>, 
    AuthToken(token): AuthToken,
    ValidatedJson(item): ValidatedJson<NewShapeItem>
) -> Result<(StatusCode, Json<Shape>), (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let mut item = item;
    let shape = state.shape_service.add_item(&mut item, &session).await.map_err(|e| e.into())?;
    Ok((StatusCode::CREATED, Json(shape)))
}

/// Handler: get a shape by id.
async fn get_shape(State(state): State<Arc<RouteState>>, Path(id): Path<i64>) -> Result<Json<Shape>, (StatusCode, String)> {
    let shape = state.shape_service.get_item(id, None).await.map_err(|e| e.into())?;
    Ok(Json(shape))
}

/// Handler: update a shape.
async fn update_shape(
    State(state): State<Arc<RouteState>>,
    AuthToken(token): AuthToken,
    Path(id): Path<i64>,
    ValidatedJson(item): ValidatedJson<PatchShapeItem>,
) -> Result<Json<Shape>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let shape = state.shape_service.update_item(id, &item, &session).await.map_err(|e| e.into())?;
    Ok(Json(shape))
}

/// Handler: delete a shape.
async fn delete_shape(
    State(state): State<Arc<RouteState>>,
    AuthToken(token): AuthToken,
    Path(id): Path<i64>,
) -> Result<Json<Shape>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let shape = state.shape_service.delete_item(id, &session).await.map_err(|e| e.into())?;
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

/// Handler: create a new line.
async fn create_line(
    State(state): State<Arc<RouteState>>, 
    AuthToken(token): AuthToken,
    ValidatedJson(item): ValidatedJson<NewLineItem>
) -> Result<(StatusCode, Json<Line>), (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let mut item = item;
    let line = state.line_service.add_item(&mut item, &session).await.map_err(|e| e.into())?;
    Ok((StatusCode::CREATED, Json(line)))
}

/// Handler: get a line by id.
async fn get_line(State(state): State<Arc<RouteState>>, Path(id): Path<i64>) -> Result<Json<Line>, (StatusCode, String)> {
    let line = state.line_service.get_item(id, None).await.map_err(|e| e.into())?;
    Ok(Json(line))
}

/// Handler: update a line.
async fn update_line(
    State(state): State<Arc<RouteState>>,
    AuthToken(token): AuthToken,
    Path(id): Path<i64>,
    ValidatedJson(item): ValidatedJson<PatchLineItem>,
) -> Result<Json<Line>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let line = state.line_service.update_item(id, &item, &session).await.map_err(|e| e.into())?;
    Ok(Json(line))
}

/// Handler: delete a line.
async fn delete_line(
    State(state): State<Arc<RouteState>>,
    AuthToken(token): AuthToken,
    Path(id): Path<i64>,
) -> Result<Json<Line>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let line = state.line_service.delete_item(id, &session).await.map_err(|e| e.into())?;
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
            limit: params.limit.unwrap_or(50) as u32,
            offset: params.offset.map(|o| o as i64),
        }),
    };

    let rules = state.rule_service.get_items(&lister, None).await.map_err(|e| e.into())?;
    Ok(Json(rules))
}

/// Handler: create a new rule.
async fn create_rule(
    State(state): State<Arc<RouteState>>, 
    AuthToken(token): AuthToken,
    ValidatedJson(item): ValidatedJson<NewRuleItem>
) -> Result<(StatusCode, Json<Rule>), (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let mut item = item;
    let rule = state.rule_service.add_item(&mut item, &session).await.map_err(|e| e.into())?;
    Ok((StatusCode::CREATED, Json(rule)))
}

/// Handler: get a rule by id.
async fn get_rule(State(state): State<Arc<RouteState>>, Path(id): Path<i64>) -> Result<Json<Rule>, (StatusCode, String)> {
    let rule = state.rule_service.get_item(id, None).await.map_err(|e| e.into())?;
    Ok(Json(rule))
}

/// Handler: update a rule.
async fn update_rule(
    State(state): State<Arc<RouteState>>,
    AuthToken(token): AuthToken,
    Path(id): Path<i64>,
    ValidatedJson(item): ValidatedJson<PatchRuleItem>,
) -> Result<Json<Rule>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let rule = state.rule_service.update_item(id, &item, &session).await.map_err(|e| e.into())?;
    Ok(Json(rule))
}

/// Handler: delete a rule.
async fn delete_rule(
    State(state): State<Arc<RouteState>>,
    AuthToken(token): AuthToken,
    Path(id): Path<i64>,
) -> Result<Json<Rule>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let rule = state.rule_service.delete_item(id, &session).await.map_err(|e| e.into())?;
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
            limit: params.limit.unwrap_or(50) as u32,
            offset: params.offset.map(|o| o as i64),
        }),
    };

    let layouts = state.layout_service.get_items(&lister, None).await.map_err(|e| e.into())?;
    Ok(Json(layouts))
}

/// Handler: create a new layout.
async fn create_layout(
    State(state): State<Arc<RouteState>>, 
    AuthToken(token): AuthToken,
    ValidatedJson(item): ValidatedJson<NewLayoutItem>
) -> Result<(StatusCode, Json<Layout>), (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let mut item = item;
    let layout = state.layout_service.add_item(&mut item, &session).await.map_err(|e| e.into())?;
    Ok((StatusCode::CREATED, Json(layout)))
}

/// Handler: get a layout by id.
async fn get_layout(State(state): State<Arc<RouteState>>, Path(id): Path<i64>) -> Result<Json<Layout>, (StatusCode, String)> {
    let layout = state.layout_service.get_item(id, None).await.map_err(|e| e.into())?;
    Ok(Json(layout))
}

/// Handler: update a layout.
async fn update_layout(
    State(state): State<Arc<RouteState>>,
    AuthToken(token): AuthToken,
    Path(id): Path<i64>,
    ValidatedJson(item): ValidatedJson<PatchLayoutItem>,
) -> Result<Json<Layout>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let layout = state.layout_service.update_item(id, &item, &session).await.map_err(|e| e.into())?;
    Ok(Json(layout))
}

/// Handler: delete a layout.
async fn delete_layout(
    State(state): State<Arc<RouteState>>,
    AuthToken(token): AuthToken,
    Path(id): Path<i64>,
) -> Result<Json<Layout>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let layout = state.layout_service.delete_item(id, &session).await.map_err(|e| e.into())?;
    Ok(Json(layout))
}

/// Handler: get a layout by semantic identifier (slug).
async fn get_layout_by_slug(State(state): State<Arc<RouteState>>, Path(slug): Path<String>) -> Result<Json<Layout>, (StatusCode, String)> {
    let layout = state.layout_service.get_item_by_slug(&slug).await.map_err(|e| e.into())?;
    Ok(Json(layout))
}
