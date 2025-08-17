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
