use std::sync::Arc;

use axum::{
    routing::{get}, Router
};
use clap::Parser;

use ws::{boards, common::workshop_store::{WorkshopItemType, WorkshopStore, WorkshopStores}, config::Config, db, package, session, workshop};

/// WebSocket server application
#[derive(Parser, Debug)]
#[command(name = "ws")]
#[command(about = "WebSocket server application", long_about = None)]
struct Args {
    /// Initialize database and run migrations
    #[arg(long)]
    init_db: bool,
}

/// Run database migrations
async fn run_migrations(pool: Arc<sqlx::Pool<sqlx::Sqlite>>) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations")
        .run(&*pool)
        .await
}

// #[tokio::main]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // witt? 
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let config = Config::new();

    println!("Starting server on {}:{}", config.host, config.port);
    println!("Using database: {} pool {}", config.database_url, config.database_pool);

    let connection = db::SqlitePool::new(
        &config.database_url,
        config.database_pool,
    ).await;

    // Run migrations if --init-db flag is provided
    if args.init_db {
        println!("Running database migrations...");
        if let Err(e) = run_migrations(connection.get()).await {
            eprintln!("Failed to run migrations: {}", e);
            std::process::exit(1);
        }
        println!("Database migrations completed successfully");
    }

    let jwt_adapter = ws::jwt_adapter::JwtAdapter::new(
        &config.app_jwt_secret,
        config.app_jwt_expiration_secs,
    );

    let jwt_service = Arc::new(session::session_jwt::SessionJWTService::new(jwt_adapter));

    let workshop_stores = Arc::new(WorkshopStores {
        shape_store: Arc::new(WorkshopStore::new(WorkshopItemType::Shape)),
        line_store: Arc::new(WorkshopStore::new(WorkshopItemType::Line)),
        rule_store: Arc::new(WorkshopStore::new(WorkshopItemType::Rule)),
        layout_store: Arc::new(WorkshopStore::new(WorkshopItemType::Layout)),
    });

    let package_router = package::get_router(connection.get_transaction_starter(), jwt_service.clone(), workshop_stores.clone());
    let boards_router = boards::get_router(connection.get_transaction_starter(), jwt_service.clone(), workshop_stores.clone());
    let session_router = session::get_router(connection.get(), jwt_service.clone());
    let workshop_router = workshop::get_router(connection.get_transaction_starter(), jwt_service.clone(), workshop_stores.clone());

    let app = Router::new()
        .route("/", get(root))
        .route("/wait_async", get(wait_async))
        .route("/wait_sync", get(wait_sync))
        .nest("/api/board", boards_router)
        .nest("/api/package", package_router)
        .nest("/api/session", session_router)
        .nest("/api/workshop", workshop_router);

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
