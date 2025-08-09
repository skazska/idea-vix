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

use axum::{ 
    extract::State, 
    http::{StatusCode, header::{HeaderMap, SET_COOKIE}}, 
    response::{IntoResponse},
    routing::post, 
    Json, 
    Router 
};

use crate::{api::{results::Success, validation::ValidatedJson}, ext_comm, jwt_adapter::JwtAdapter, session::{session_jwt::SessionJWTService, session_service::{ConfirmSession, InitSession, SessionService}}};

mod session_store;
pub mod session_service;
pub mod session_jwt;

struct RouteState {
    service: SessionService,
}

/// Get the session router
pub async fn get_router<'a>(connection: Arc<sqlx::Pool<sqlx::Sqlite>>, jwt_service: Arc<SessionJWTService>) -> axum::Router {
    let session_store = session_store::SessionStore::new(connection);
    let ext_comm = ext_comm::ExtComm::new();
    let session_service = SessionService::new(session_store, ext_comm, jwt_service);

    let state = Arc::new(RouteState {
        service: session_service,
    });

    Router::new()
        .route("/signin", post(signin_session))
        .route("/verify", post(verify_session))
        .route("/signout", post(signout_session))
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
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (session_data, token) = state.service.verify_session(&item).await.map_err(|e| e.into())?;
    let max_age = session_data.expires_at - session_data.sent_at;
    // Create JSON response
    let json = Json(session_data);

    // Set the secure HttpOnly cookie with the JWT token
    let cookie = format!(
        "Authorization=Bearer {}; HttpOnly; SameSite=Strict; Path=/; Max-Age={}",
        token,
        max_age
    );

    // Create a response with headers
    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.parse().unwrap());

    Ok((headers, json))
}

/// Sign out by clearing the session cookie
async fn signout_session() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    // Set an expired cookie to clear it
    headers.insert(
        SET_COOKIE,
        "Authorization=; HttpOnly; SameSite=Strict; Path=/; Max-Age=0".parse().unwrap()
    );
    
    (headers, Json(Success { success: true }))
}


