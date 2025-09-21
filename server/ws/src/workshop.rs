//! Workshop module: API routes for workshop elements (shapes, lines, rules, layouts).
//!
//! The workshop provides global management of diagram element blueprints.
//! These elements can be referenced by packages and used in boards.

use std::sync::Arc;
use crate::common::crud::{CrudService, ListParams};
use crate::common::workshop::shape_service::{NewShapeItem, PatchShapeItem, Shape, ShapeService};
use crate::common::workshop::shape_store::ShapeStore;
use crate::db::TransactionStarter;
use crate::api::{deserialize::AuthToken, validation::ValidatedJson};
use crate::session::session_jwt::SessionJWTService;
use axum::extract::Query;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::Deserialize;

/// Shared state for the workshop routes.
struct RouteState {
    shape_service: ShapeService,
    jwt_service: Arc<SessionJWTService>,
}

/// Create the workshop router with all shape routes.
pub fn get_router(
    transaction_starter: Arc<TransactionStarter>,
    jwt_service: Arc<SessionJWTService>,
) -> Router {
    let shape_store = Arc::new(ShapeStore::new());
    let shape_service = ShapeService::new(transaction_starter, shape_store);

    let state = Arc::new(RouteState {
        shape_service,
        jwt_service,
    });

    Router::new()
        // Global workshop shape routes
        .route("/workshop/shapes", get(|state: State<Arc<RouteState>>, params: Query<SearchParams>| async {
            list_shapes(state, params).await
        }))
        .route("/workshop/shapes", post(|state: State<Arc<RouteState>>, token: AuthToken, item: ValidatedJson<NewShapeItem>| async {
            create_shape(token, state, item).await
        }))
        .route("/workshop/shapes/{id}", get(|state: State<Arc<RouteState>>, id: Path<i64>| async {
            get_shape(state, id).await
        }))
        .route("/workshop/shapes/by-slug/{slug}", get(|state: State<Arc<RouteState>>, slug: Path<String>| async {
            get_shape_by_slug(state, slug).await
        }))
        .with_state(state)
}

#[derive(Deserialize, Debug)]
pub struct SearchParams {
    pub ids: Vec<i64>,
    // pub limit: Option<u32>,
}

/// Handler: list all shapes in the global workshop.
/// - No authentication required for reading
/// - Returns array of shapes
async fn list_shapes(State(state): State<Arc<RouteState>>, Query(params): Query<SearchParams>) -> Result<Json<Vec<Shape>>, (StatusCode, String)> {
    if params.ids.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Missing or invalid 'id' query parameter".into()));
    }

    // Create a lister with the requested IDs
    let lister = ListParams {
        filter: Some(crate::common::crud::ListFilter {
            filter: None,
            ids: Some(params.ids),
            search: None,
        }),
        pager: None,
    };

    let shapes = state.shape_service.get_items(&lister, None).await.map_err(|e| e.into())?;

    Ok(Json(shapes))
}

/// Handler: create a new shape in the global workshop.
/// - Requires authentication
/// - Validates slug format and uniqueness
/// - Returns 201 Created with the new shape
async fn create_shape(
    AuthToken(token): AuthToken,
    State(state): State<Arc<RouteState>>,
    ValidatedJson(mut item): ValidatedJson<NewShapeItem>,
) -> Result<(StatusCode, Json<Shape>), (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let shape = state.shape_service.add_item(&mut item, &session).await.map_err(|e| e.into())?;
    Ok((StatusCode::CREATED, Json(shape)))
}

/// Handler: get a shape by id.
/// - No authentication required for reading
/// - Returns single shape or 404
async fn get_shape(State(state): State<Arc<RouteState>>, Path(id): Path<i64>) -> Result<Json<Shape>, (StatusCode, String)> {
    let shape = state.shape_service.get_item(id, None).await.map_err(|e| e.into())?;
    Ok(Json(shape))
}

/// Handler: get a shape by semantic identifier (slug).
/// - No authentication required for reading
/// - Returns single shape or 404
async fn get_shape_by_slug(State(state): State<Arc<RouteState>>, Path(slug): Path<String>) -> Result<Json<Shape>, (StatusCode, String)> {
    let shape = state.shape_service.get_shape_by_slug(&slug).await.map_err(|e| e.into())?;
    Ok(Json(shape))
}

/// Handler: update an existing shape.
/// - Requires authentication
/// - Validates slug format and uniqueness if provided
/// - Returns updated shape
async fn update_shape(
    AuthToken(token): AuthToken,
    State(state): State<Arc<RouteState>>,
    Path(id): Path<i64>,
    ValidatedJson(item): ValidatedJson<PatchShapeItem>,
) -> Result<Json<Shape>, (StatusCode, String)> {
    let session = state.jwt_service.get_session_data(&token).map_err(|e| e.into())?;
    let shape = state.shape_service.update_item(id, &item, &session).await.map_err(|e| e.into())?;
    Ok(Json(shape))
}
