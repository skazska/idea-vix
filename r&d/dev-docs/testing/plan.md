# Testing Plan

Purpose: Define a practical approach to integration and end-to-end (E2E) testing for this project, with minimal friction for local runs and CI.

## Goals

- Verify API behavior and contracts across router -> service -> store -> DB.
- Verify critical user flows in the UI against a running backend.
- Keep tests deterministic, isolated, and fast to run locally and in CI.

## Tiered execution strategy

To keep feedback loops short while still guarding against regressions, the suite is split into two execution tiers:

- **Smoke (Tier 1)** — Happy-path coverage for session auth plus board/package CRUD on backend and UI. Runs in under five minutes and is appropriate for every commit or pre-PR hook. Failing smoke indicates the build is unsafe to merge.
- **Regression (Tier 2)** — Full matrix spanning role/permission permutations, destructive flows, and invite lifecycle. Runs the entire backend `tests/` tree and all Playwright specs. Schedule nightly in CI and before tagged releases.

Implementation roadmap:

1. Tag Playwright smoke scenarios with `@smoke` and extend `npm run test:e2e` with a `test:e2e:smoke` script.
2. Introduce a Rust test runner wrapper (e.g., `cargo test --test smoke_suite`) that re-exports the three smoke integration tests (`sessions_smoke`, `boards_smoke`, `packages_smoke`) for a single command.
3. Update CI to execute smoke on every PR and regression on a nightly cron and release branches.
4. Track coverage drift in `traceability-matrix.md`; if a gap is smoke-critical, promote its test into Tier 1.

## Scope

- Integration tests (backend, Rust): Axum routers + SQLx + SQLite + JWT flow.
  - Smoke coverage lives in `sessions_smoke.rs`, `boards_smoke.rs`, `packages_smoke.rs`.
  - Regression coverage adds `boards_regression.rs`, `packages_regression.rs` for validation, access control, and visibility edge cases.
- E2E tests (frontend, Playwright): UI flows for auth and boards/packages CRUD.
- Out of scope (initially): performance, load, fuzz, security scans.

## Test matrix (initial)

- Sessions
  - signin -> verify -> me -> signout
  - cookie handling, expiry boundaries (basic)
- Boards
  - create -> list -> get -> update -> delete
  - visibility with/without session
- Packages
  - create -> list -> get -> update -> delete
  - visibility with/without session
- Error handling
  - invalid requests (400s)
  - unauthorized access (401s)

---

## Backend Integration Tests (Rust)

### Tooling & dependencies

- tokio (async runtime)
- reqwest (HTTP client) or tower::ServiceExt for in-process requests
- serde_json for payloads
- sqlx (with `migrate` feature) and sqlite
- in-memory SQLite for fast isolated tests (or tempfile for persistence. why?  for isolation?)

### Harness design

Two viable strategies; choose one per test module for clarity:

1. In-process (faster)

   - Build Router directly (mirror `main.rs` wiring).
   - Use `tower::ServiceExt::oneshot` to send requests.

1. Spawned server (closer to prod)

   - Bind to random free port, `tokio::spawn` Axum server.
   - Use `reqwest` to call endpoints.

Recommendation: start with in-process for CRUD coverage; add 1–2 spawned tests to cover cookie handling end-to-end.

### Database & migrations

- Create a unique temp SQLite file per test (tempfile crate) for isolation. (use in-memory SQLite for speed  - not allows parrallel runs of tests)
- Instantiate `db::SqlitePool::new(path, size)`.
- Run migrations: `sqlx::migrate!("./migrations").run(&pool).await`.

### JWT/session setup

- Initialize `JwtAdapter` with a fixed test secret and short expiration.
- Build `SessionJWTService` from the adapter.
- For auth-requiring routes, acquire cookie by running the signin/verify flow in test, or inject a known-good token.

### Helper(s)

- `test_app.rs` to construct:
  - DB pool (temp file) + run migrations
  - JWT service
  - Routers: boards, packages, sessions
  - Return either the Router (in-process) or a started server URL

### Example test cases

- `sessions_flow_ok`
  - POST /api/session/signin with address
  - POST /api/session/verify with code "some_code"
  - Assert Set-Cookie contains Authorization=Bearer ...
  - GET /api/session/me with cookie -> 200 and session json
  - POST /api/session/signout -> cookie cleared

- `boards_crud_ok`
  - Create board (auth cookie)
  - List boards -> contains created
  - Get by id -> fields match
  - Update -> fields updated
  - Delete -> 204; list no longer contains

- `boards_visibility`
  - Create public and private boards
  - List without cookie -> only public
  - List with cookie -> both

### Running integration tests locally

- `cargo test -p ws` (or workspace-wide `cargo test`) after enabling sqlx `migrate` feature in the crate.

---

## Frontend E2E Tests (Playwright)

### Tooling

- @playwright/test
- Browsers via `npx playwright install`

### Project setup

- Add dev dependency and `test:e2e` script in `ui/web/package.json`.
- Add `playwright.config.ts` with:
  - `webServer` to start UI (and optionally backend)
  - `use.baseURL` pointing to the UI URL
  - `timeout`/`retries` pragmatic defaults

- Option A (preferred): start backend separately (VS Code task) and only start UI in webServer.
- Option B: start both backend and UI in a small shell script called from `webServer` (ensure cross-platform or provide Linux default).

### Test data & auth

- Drive the session flow via UI (enter address, then verify with "some_code").
- Alternatively, pre-seed via direct API calls using Playwright's `request` fixture, then set cookie in context.

### Example E2E scenarios

- `auth_flow.spec.ts`
  - Visit signin page, submit address, verify with code, expect signed-in indicator (e.g., SessionInfo shows address).
- `boards_crud.spec.ts`
  - Create board via UI form, see it in list, open details, update name, delete, expect removal.
- `packages_crud.spec.ts`
  - Mirror boards flow for packages UI.

### Running E2E locally

- Backend: run server (VS Code task `cargo-run`) with DB available/migrated.
- UI: `npm run dev` (or Playwright `webServer`).
- E2E: `npm run test:e2e`.

---

## Orchestration & CI

- Local convenience script (optional):
  - Start backend (cargo-run)
  - Start UI dev server
  - Run Playwright tests, then teardown

- CI (two jobs):
  1) Rust: fmt/clippy/test (runs integration tests)
  2) Node: install/build UI, start backend+UI, run Playwright

---

## Locations & structure

- Backend integration tests: `server/ws/tests/`
  - `test_app.rs` (helpers)
  - `sessions_smoke.rs`, `boards_smoke.rs`, `packages_smoke.rs`
- Frontend E2E: `ui/web/tests-e2e/`
  - `auth_flow.spec.ts`, `boards_crud.spec.ts`, `packages_crud.spec.ts`
  - `playwright.config.ts`

## Non-goals (initial)

- Performance benchmarking
- Security scanning
- Cross-browser/device matrix beyond Playwright defaults
