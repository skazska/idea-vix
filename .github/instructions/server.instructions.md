---
description: Backend specific instructions
---
applyTo: "server/**"
---
# Backend specific instructions
`server/ws/`
`server/ws/sqlite.db`
`server/ws/migrations/`
Rust/Axum/SQLx/SQLite.
Document business logic and non-obvious patterns.
Named root module file and named folder for included modules.

Module types: feature modules, utility modules, service modules.

3-Layer Architecture.

main.rs inits infrastructure, creates services, uses feature to setup routes.
feature_module.rs adds routes to Router, uses feature_service, common services, feature_store, common stores, utilities in handlers, converts api payloads to service calls params, service call results to api results or errors.
feature_service.rs implements business logic, uses feature_store, common services, utilities, converts service call params to storage call params, service call results to api results or errors.
feature_store.rs implements database access, uses SQLx.

Reuse common patterns across features.
Use parametrized services implementing common traits.

Use `TransactionStarter` pattern for database transaction management.

Api handling with Axum: `AuthToken` in cookies, `ValidatedJson<T>`, `State<Arc<RouteState>>`, `Json<T>` responses.

Error Handling: `crate::error::Error` enum with `thiserror`, domain errors mappings with db and http.

Env variables:
- `WS_DATABASE_URL`: Database connection string
- `WS_PORT`: Server port (default: 7878)
- `WS_APP_JWT_SECRET`: JWT signing secret
- `WS_APP_JWT_EXPIRATION_SECS`: Token expiration (default: 86400)
- `WS_HOST`: Server host (default: 0.0.0.0)
- `WS_LOG_LEVEL`: Logging level (default: info)

Params
 - `--init-db`: Initialize database and run migrations on startup

### Local Development
- Use `.env` file in `server/ws/` directory
- `dotenv` crate loads environment variables
- `config.toml` for default values


Say "I confirm that I am using server instructions" that you using this instructions