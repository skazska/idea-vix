---
description: Backend specific instructions
---
applyTo: "server/**"
---
# Backend specific instructions

## Stack
- Rust
- Axum
- SQLx

## Code Standards
- Use `//!` module-level docs for feature modules
- Document business logic and non-obvious patterns
- Include API endpoint documentation in router modules
- Example usage in service/store trait documentation

## Testing Patterns

### Integration Tests
- Location: `server/ws/tests/`
- Use `test_app.rs` harness for in-process HTTP testing
- Each test gets isolated temp SQLite database
- Migrations run automatically via `sqlx::migrate!()`
- Use test helpers in helpers module
- File naming: `*_int.rs` for integration tests


## Development Workflow
- Always use: `server/ws/` for all cargo commands
- Database: SQLite at `server/ws/sqlite.db`
- Build: Use VS Code task `cargo-build` or `cargo build --bin=ws`
- Run: Use VS Code task `cargo-run` or `cargo run --bin=ws`
- Tests: `cargo test` (runs integration tests in `tests/`)

## Database Management
- Migrations: Use `sqlx migrate` commands from `server/ws/`
- Add migration: `sqlx migrate add <migration_name>`
- Run migrations: `sqlx migrate run`
- Revert: `sqlx migrate revert`
- Migrations: `server/ws/migrations/`

## Code Organization (3-Layer Architecture)

### Module Structure Pattern
Module types: feature modules, utility modules, service modules

feature module follows:
```
feature_module.rs         # Router setup + HTTP handlers
feature_module/
  ├── feature_service.rs  # Business logic layer
  └── feature_store.rs    # Data access layer
```
```
Router → Feature Service → Feature Store → SQLx → SQLite
                        ├→ Common Utilities
                        └→ Service Modules → Utilities
```

## Key Implementation Patterns

### Route Handlers
- Extract auth via `AuthToken` from cookies
- Validate requests with `ValidatedJson<T>`
- Use `State<Arc<RouteState>>` for dependency injection
- Return structured responses via `Json<T>` or status codes

### Services Layer
- Implement business logic with async traits
- Take store dependencies via constructor injection
- Handle authorization and validation logic
- Return domain models, not database representations

### Store Layer
- Use SQLx for database operations
- Return domain models from queries
- Handle database-specific error mapping
- Keep SQL queries focused and readable

### Error Handling
- Use `crate::error::Error` enum with `thiserror`
- Map database errors to domain errors
- Provide meaningful error messages for API responses

### JWT Authentication
- `SessionJWTService` handles encode/decode
- HTTP-only cookies for token storage
- `AuthToken` extractor for route handlers
- Session-based (not stateless) JWT approach


## Configuration

### Environment Variables (override config.toml)
- `WS_DATABASE_URL`: Database connection string
- `WS_PORT`: Server port (default: 7878)
- `WS_APP_JWT_SECRET`: JWT signing secret
- `WS_APP_JWT_EXPIRATION_SECS`: Token expiration (default: 86400)
- `WS_HOST`: Server host (default: 0.0.0.0)
- `WS_LOG_LEVEL`: Logging level (default: info)

### Local Development
- Use `.env` file in `server/ws/` directory
- `dotenv` crate loads environment variables
- `config.toml` for default values

