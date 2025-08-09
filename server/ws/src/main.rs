use std::sync::Arc;

use axum::{
    routing::{get}, Router
};

use ws::{boards, config::Config, package, db, session};

// #[tokio::main]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // witt? 
    tracing_subscriber::fmt::init();

    let config = Config::new();

    println!("Starting server on {}:{}", config.host, config.port);
    println!("Using database: {} pool {}", config.database_url, config.database_pool);

    let connection = db::SqlitePool::new(
        &config.database_url,
        config.database_pool,
    ).await;

    let jwt_arc = Arc::new(ws::jwt_adapter::JwtAdapter::new(
        &config.app_jwt_secret,
        config.app_jwt_expiration_secs,
    ));

    let package_router = package::get_router(connection.get(), jwt_arc.clone()).await;
    let boards_router = boards::get_router(connection.get(), jwt_arc.clone()).await;
    let session_router = session::get_router(connection.get(), jwt_arc.clone()).await;

    let app = Router::new()
        .route("/", get(root))
        .route("/wait_async", get(wait_async))
        .route("/wait_sync", get(wait_sync))
        .nest("/api/board", boards_router)
        .nest("/api/package", package_router)
        .nest("/api/session", session_router);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port))
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn wait_async() -> &'static str {
    tokio::time::sleep(std::time::Duration::from_secs(10)).await; // Simulate some processing delay
    "Wait async endpoint"
}

async fn wait_sync() -> &'static str {
    std::thread::sleep(std::time::Duration::from_secs(10)); // Simulate some processing delay
    "Wait sync endpoint"
}
