use std::sync::Arc;

use axum::{routing::get, Router};
use ws::{db, session};

pub struct TestApp {
    pub router: Router,
}

impl TestApp {
    pub async fn new() -> Self {
        // temp sqlite file per test
        let tmp = tempfile::NamedTempFile::new().expect("tmp db");
        let db_path = tmp.path().to_string_lossy().to_string();
        // keep file alive
        std::mem::forget(tmp);

        let pool = db::SqlitePool::new(&db_path, 5).await;

        // run migrations
        let pool_arc: Arc<sqlx::Pool<sqlx::Sqlite>> = pool.get();
        sqlx::migrate!("./migrations").run(&*pool_arc).await.expect("migrations");

        // jwt
        let jwt_adapter = ws::jwt_adapter::JwtAdapter::new("test_secret", 3600);
        let jwt_service = Arc::new(session::session_jwt::SessionJWTService::new(jwt_adapter));

        // routers
        let package_router = ws::package::get_router(pool_arc.clone(), jwt_service.clone()).await;
        let boards_router = ws::boards::get_router(pool_arc.clone(), jwt_service.clone()).await;
        let session_router = ws::session::get_router(pool_arc.clone(), jwt_service.clone()).await;

        let router = Router::new()
            .route("/", get(|| async { "ok" }))
            .nest("/api/board", boards_router)
            .nest("/api/package", package_router)
            .nest("/api/session", session_router);

        Self { router }
    }
}
