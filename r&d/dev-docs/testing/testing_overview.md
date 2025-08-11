# Testing Overview

Purpose: Summarize test layers and provide step-by-step instructions to run them locally.

## What is covered

- Backend integration tests (Rust): Axum Router + Services + Stores + SQLx + SQLite + JWT flow
- Frontend end-to-end (E2E) tests (Playwright): critical UI flows against a running backend

## Prerequisites

- Rust toolchain (cargo)
- Node.js and npm
- For E2E: Playwright browsers (`npx playwright install`)

Notes:

- Backend tests create a temp SQLite database and run migrations via `sqlx::migrate!`, no manual DB setup needed.
- Cargo.toml already enables `sqlx` features: `macros`, `migrate`.

---

## Backend Tests (Rust)

### Integration Tests

Location:

- `server/ws/tests/`
  - `test_app.rs` (harness)
  - `sessions_int.rs`, `boards_int.rs`, `packages_int.rs`

How it works:

- In-process testing via `tower::ServiceExt::oneshot` against the Axum Router
- Temp SQLite file per test for isolation; migrations run automatically
- JWT service is initialized with a test secret; session flow uses stub code `some_code`

### Unit Tests

Location:
  in source files, e.g. `server/ws/src/config.rs`

### Run Tests

Run all tests:

```bash
cd server
cargo test
```

Compile tests without running: `bash cargo test --no-run`
Run lib unit tests: `bash cargo test --lib`
Run bin unit tests: `bash cargo test --bin ws`
Run a single integration test: `bash cargo test --test sessions_int`
Run all integration tests: `bash cargo test --tests`
Run tests filtered by name: `bash cargo test <test_name>`

## Frontend E2E Tests (Playwright)

Location:

- `ui/web/tests-e2e/`
  - `playwright.config.ts`
  - `auth_flow.spec.ts`

Assumptions:

- Backend runs on `http://localhost:7878` (use VS Code task `cargo-run`)
- UI dev server runs on `http://localhost:5173` (started by Playwright `webServer`)

Install and run:

```bash
cd ui/web
npm install
npx playwright install
npm run test:e2e
```

Headed mode (for debugging): `bash npm run test:e2e:headed`

Optional:

- Override UI server URL via `UI_BASE_URL` env if needed; defaults to `http://localhost:5173` in `playwright.config.ts`.

---

## Troubleshooting

- If E2E tests fail to connect, ensure the backend is running on port 7878.
- If TypeScript can’t resolve `@playwright/test`, run `npm install` in `ui/web`.
- If migrations fail in integration tests, verify `server/ws/migrations/` exists and is readable.

---

## CI (hint)

- Job 1 (Rust): format, clippy, `cargo test` in `server/ws`
- Job 2 (Node): install, build UI, start backend+UI, run Playwright E2E
