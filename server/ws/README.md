# Backend Development

## Prerequisites

Rust toolchain (including `cargo`). You can find installation instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

SQLite database. You can find installation instructions on the [official SQLite website](https://www.sqlite.org/download.html).

sqlx-cli. You can find installation instructions on the [official sqlx website](https://crates.io/crates/sqlx).

## Getting Started

The server workspace is located in the `server/` directory. The main application crate is `ws`. All `cargo` commands might be run from the `server/` directory.

## Configurations

The application can be configured through a TOML file or environment variables. Environment variables take precedence over the configuration file.

The path to the configuration file can be specified with the `WS_CONFIG_FILE` environment variable. It defaults to `config.toml` in the `server/ws` directory.

A `.env` file can also be used to set environment variables.

Here are the available configuration options:

| Environment Variable          | TOML Property               | Description                                  | Default Value                  |
| ----------------------------- | --------------------------- | -------------------------------------------- | ------------------------------ |
| `WS_HOST`                     | `host`                      | Server host address.                         | `0.0.0.0`                      |
| `WS_PORT`                     | `port`                      | Server port.                                 | `7878`                         |
| `WS_LOG_LEVEL`                | `log_level`                 | Logging level.                               | `info`                         |
| `WS_DATABASE_URL`             | `database_url`              | Database connection URL.                     | `sqlite.db`                    |
| `WS_DATABASE_POOL`            | `database_pool`             | Database connection pool size.               | `5`                            |
| `WS_APP_JWT_EXPIRATION_SECS`  | -                           | JWT expiration time in seconds.              | `86400` (24 hours)             |
| `WS_APP_JWT_SECRET`           | -                           | Secret key for signing JWTs.                 | `supersecretkey_for_dev_only`  |
| `WS_CONFIG_FILE`              | -                           | Path to the configuration file.              | `config.toml`                  |

## Migrations

using sqlx-cli
`bash sqlx migrate add <migration_name>` adds a new migration.
`bash sqlx migrate run` applies the migrations.
`bash sqlx migrate revert` reverts the last migration.

migration files are located in `server/ws/migrations/`.

default db files located at `server/ws/sqlite.db`.

## Build and run

`bash cargo build --bin=ws` builds the application with debug information.
`bash cargo build --release --bin=ws` builds the application with optimizations.

The output will be in `server/target/debug/ws` or `server/target/release/ws`.

`bash cargo run --bin=ws` starts the server.

The server will start on `http://localhost:7878` by default.

The application uses the following environment variables:

## Running tests

To run all tests (unit and integration), use:

`bash cargo test`

You can also run specific tests:

- Compile tests without running: `cargo test --no-run`
- Run only library unit tests: `cargo test --lib`
- Run only binary unit tests: `cargo test --bin ws`
- Run all integration tests: `cargo test --tests`
- Run a single integration test file: `cargo test --test <test_name>`, e.g., `cargo test --test sessions_int`

Note: Integration tests automatically create a temporary database and run migrations.

## Code standards

### Modularity and Extensibility

Code should be modular and extensible, allowing for easy addition of new features or modifications to existing ones without significant refactoring.

Logical separation of features and functionality is achieved through units and modules.

Units are usually devoted to specific functionality or features, such as package management, boards, or user sessions. Each module should encapsulate its own logic and data structures.

Internal unit structure consists of layers.

Layers may include modules such as `api`, `service`, and `storage` and use common modules like `error` and `db`.

#### Typical layer structure

- **api**: Contains the API endpoints and request handlers. Converts payload/params to model and dispatch call to service layer. converts service results to API response.
- **service**: Service layer implements business logic, which may include:
  - Validating input data
  - Performing calculations or transformations
  - Interacting with the storage layer to retrieve or modify data
  - Interacting with other services
  - Interacting with the external APIs
  - Returning results or errors
- **storage**: Contains the database models and data access logic.

#### Typical execution flow

1. API layer receives and validates request. Converts payload/params to model and dispatch call to service layer.
2. Service layer implements business logic including access to storage layer.
3. Storage layer interacts with the database to retrieve or modify data.
4. Service layer returns the result to the API layer.
5. API layer sends the final response to the client.

#### Dependency sanity

There should be no circular dependencies between modules. Each module should depend only on the modules that are necessary for its functionality.
There could be common units of modules that are used by multiple modules, such as `error`, `db`, or `config`. These common modules should not depend on any specific feature modules, and better not depend on other common units of modules if possible.
Feature modules should not depend on each other directly. Instead, they may interact through the API layer or service layer, which acts as a mediator, or there should be integral units of modules that provide functionality involving multiple feature modules.

## Unit Testing Standards

Unit tests should provide comprehensive coverage for individual functions and methods while maintaining readability and ease of maintenance.

### Test Organization

- Tests should be placed in a `#[cfg(test)]` module within the same file as the code being tested
- Use descriptive test function names that clearly indicate what is being tested
- Group related tests with clear section comments (e.g., `// --- AuthToken tests ---`)

### Helper Functions

Use helper functions to reduce boilerplate and improve test readability:

```rust
// Helper for testing conversions with expected results
fn conversion_probe(input: InputType, expected: OutputType) {
    let result = input.convert();
    assert_eq!(result, expected);
}

// Helper for testing responses with expected status codes
fn response_probe(error: ErrorType, expected_status: StatusCode) {
    let response = error.into_response();
    assert_eq!(response.status(), expected_status);
}
```

### Test Coverage Principles

1. **Pure functions**: Test all branches and edge cases
2. **Conversion methods**: Test all variants/cases with expected outputs
3. **Error handling**: Test all error types map to correct responses
4. **Integration points**: Test actual usage patterns where possible

### Test Structure

Each test should follow a clear structure:

```rust
#[test]
fn descriptive_test_name() {
    // Arrange - set up test data
    let input = create_test_data();
    
    // Act & Assert - use helper functions when possible
    helper_probe(input, expected_result);
}
```

### Async Testing

For async functions, use `#[tokio::test]`:

```rust
#[tokio::test]
async fn async_function_test() {
    let result = async_function().await;
    assert_eq!(result.unwrap(), expected_value);
}
```

### Testing Guidelines

- **Focus on behavior**: Test what the function does, not how it does it
- **Use realistic data**: Test with data that resembles actual usage
- **Test edge cases**: Include boundary conditions and error scenarios
- **Keep tests simple**: Each test should verify one specific behavior
- **Avoid mocking when possible**: Use real data structures for unit tests
- **Regression protection**: Tests should catch breaking changes in behavior

### Examples

Good test naming:

- `deserialize_some_missing_field()`
- `auth_token_from_authorization_header()`
- `model_error_forbidden_conversion()`

Good helper usage:

- `deserialize_some_probe(json, expected_struct)`
- `auth_token_probe(headers, expected_token)`
- `model_error_to_status_probe(error, expected_status, expected_message)`

### Test Maintenance

- Update tests when changing function behavior
- Remove obsolete tests when removing functionality  
- Keep test helpers aligned with production code patterns
- Ensure tests remain fast and deterministic

