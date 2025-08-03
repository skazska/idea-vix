/// Feature unit of modules for user sessions in the application.
/// Identification:
/// - init session by address:
///   - generate verification code
///   - send code to address
///   - store address, code and sent time in database
/// - confirm session by address and code:
///   - error if code is not valid
///   - return jwt token if code is valid 

use std::sync::Arc;

use axum::{ extract::State, http::StatusCode, routing::post, Json, Router };

use crate::{api::{results::Success, validation::ValidatedJson}, ext_comm, jwt_adapter::JwtAdapter, session::session_service::{ConfirmSession, InitSession, SessionData, SessionService}};

mod session_store;
mod session_service;
mod jwt;

struct RouteState {
    service: SessionService,
}

/// Get the session router
pub async fn get_router<'a>(connection: Arc<sqlx::Pool<sqlx::Sqlite>>, jwt_adapter: Arc<JwtAdapter>) -> axum::Router {
    let session_store = session_store::SessionStore::new(connection);
    let ext_comm = ext_comm::ExtComm::new();
    let session_service = SessionService::new(session_store, ext_comm, jwt_adapter);

    let state = Arc::new(RouteState {
        service: session_service,
    });

    Router::new()
        .route("/signin", post(signin_session))
        .route("/verify", post(verify_session))
        .with_state(state)
}

/// Initialize a session with the given address
async fn signin_session(
    State(state): State<Arc<RouteState>>, 
    ValidatedJson(item): ValidatedJson<InitSession>
) -> Result<Json<Success>, (StatusCode, String)> {
    let result = state.service.signin_session(item).await.map_err(|e| e.into())?;
    Ok(Json(result))
}

/// Verify a session with the given address and code
async fn verify_session(
    State(state): State<Arc<RouteState>>, 
    Json(item): Json<ConfirmSession>
) -> Result<Json<SessionData>, (StatusCode, String)> {
    let result = state.service.verify_session(&item).await.map_err(|e| e.into())?;
    Ok(Json(result))
}


