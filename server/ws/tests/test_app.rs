use std::sync::Arc;

use axum::{routing::get, Router};
use ws::{
    workshop::workshop_store::{WorkshopItemType, WorkshopStore, WorkshopStores},
    db,
    session
};

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

        let workshop_stores = Arc::new(WorkshopStores {
            shape_store: Arc::new(WorkshopStore::new(WorkshopItemType::Shape)),
            line_store: Arc::new(WorkshopStore::new(WorkshopItemType::Line)),
            rule_store: Arc::new(WorkshopStore::new(WorkshopItemType::Rule)),
            layout_store: Arc::new(WorkshopStore::new(WorkshopItemType::Layout)),
        });

        // routers
        let package_router = ws::package::get_router(pool.get_transaction_starter(), jwt_service.clone(), workshop_stores.clone());
        let boards_router = ws::boards::get_router(pool.get_transaction_starter(), jwt_service.clone(), workshop_stores.clone());
        let session_router = ws::session::get_router(pool.get(), jwt_service.clone());
        let workshop_router = ws::workshop::get_router(pool.get_transaction_starter(), jwt_service.clone(), workshop_stores.clone());

        let router = Router::new()
            .route("/", get(|| async { "ok" }))
            .nest("/api/board", boards_router)
            .nest("/api/package", package_router)
            .nest("/api/session", session_router)
            .nest("/api/workshop", workshop_router);

        Self { router }
    }
}
