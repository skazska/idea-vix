# Testing Overview

- Follow coding standards. Use helper functions.

## Prerequisites

- Rust toolchain (cargo)
- SQLite database, migrations, sqlx CLI
- Node.js and npm for web UI (npm run dev)
- E2E on Playwright with browsers (npm run test:e2e)

## Coverage

- Requirements coverage: see `r&d/analysys/requirements/traceability-matrix.md`
- Backend unit tests (Rust): utility functions.
- Backend integration tests (Rust): Axum Router + Services + Stores + SQLx + SQLite + JWT flow
- Frontend end-to-end (E2E) tests (Playwright)

## Test tiers

from project root:

1. unit: `./ci/test_unit.sh`
2. integration: `./ci/test_integration.sh --smoke`, `./ci/test_integration.sh`
3. e2e: `./ci/test_e2e.sh --smoke`, `./ci/test_e2e.sh` - starts backend on :7879, frontend preview on :4173
4. all: `./ci/test_all.sh --smoke`, `./ci/test_all.sh`

## Backend Tests (Rust)

### Units

run: `cargo test --lib`
source: `#[cfg(test)]` in source files, e.g. `server/ws/src/config.rs`

### Integration

run: `cargo test smoke`/`cargo test` in `server/ws`
source: `server/ws/tests/`, querying `test_app` using `helpers`.
db: in-memory SQLite prepared in `test_app.rs`.
JWT service is initialized with a test secret; session flow uses stub code `some_code`

## E2E Tests (Playwright)

source: `ui/web/tests-e2e/`
config - `playwright.config.ts`
requirements: backend server running on :7878
run: `npm run test:e2e`, `bash npm run test:e2e:headed` - on :5173
