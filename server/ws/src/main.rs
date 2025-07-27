use axum::{
    routing::{get}, Router
};

use ws::{boards, config::Config, package, db};

// #[tokio::main]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // witt? 
    tracing_subscriber::fmt::init();

    let config = Config::new();

    println!("Starting server on {}:{}", config.host, config.port);
    println!("Using database: {} pool {}", config.database_url, config.database_pool);

    let connection = db::pool::Pool::new(
        &config.database_url,
        config.database_pool,
    ).await;

    let package_router = package::get_router(connection.get()).await;
    let boards_router = boards::get_router(connection.get()).await;

    let app = Router::new()
        .route("/", get(root))
        .route("/wait_async", get(wait_async))
        .route("/wait_sync", get(wait_sync))
        .nest("/api/board", boards_router)
        .nest("/api/package", package_router);

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
