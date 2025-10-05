# Project Implementation Overview

Purpose: Give a concise, structured view of how this project is built and how it works at runtime. Audience: contributors and reviewers.

## 1) High-level system

- Client: SolidJS SPA (Vite) in `ui/web/`
- API: Axum-based Rust service in `server/ws/`
- Storage: SQLite via SQLx, file `server/ws/sqlite.db`
- Auth: Passwordless sessions with JWT returned as HttpOnly cookie

ASCII map:

Browser (SolidJS) -> HTTP/JSON -> Axum Routers -> Services -> Stores -> SQLx -> SQLite
                                 |                   |
                                 |-- JWT (cookie) ---|

## 2) Wares and tiers

- Software: backend crate `ws`, frontend SPA, build configs
- Infoware: database file, SQL migrations
- Docuware: `r&d/dev-docs/` (this doc, code-overview, API spec)
- Config-ware: `config.toml` (optional), `WS_*` env vars
- Analytics: `r&d/analytics/` user-stories, requirements, and design docs

Tiers:

- Presentation: SolidJS SPA + Axum routers
- Application: service layer (business logic)
- Persistence: SQLx stores and migrations

## 3) Runtime components (backend)

Crate `server/ws/src/` modules and roles:

- `main.rs`: bootstraps config, DB pool, JWT service; mounts routers
- Routers (presentation): `boards.rs`, `package.rs`, `workshop.rs`, `session.rs`
- Services (application): `boards/board_service.rs`, `package/package_service.rs`, `workshop/generic_service.rs`, `session/session_service.rs`
- Stores (persistence): `boards/board_store.rs`, `package/package_store.rs`, `common/workshop_store.rs`, `session/session_store.rs`
- Infrastructure: `config.rs`, `db.rs` (pool + time utils), `jwt_adapter.rs`, `session/session_jwt.rs`, `ext_comm{,/email}.rs`
- Common patterns: `TransactionStarter` for database operations, `CommonItemAccess` for access control
- Unified traits: `CrudService` and `CrudQueries` implemented by boards, packages, and workshop items
- API helpers: `api/{validation,results,deserialize}.rs`
- Error type: `error.rs`

Testing (high-level): Integration tests exercise routers → services → stores against SQLite using in-process requests. See Testing section and `testing_overview.md` for details.

Mounted endpoints (see routers):

- `/api/board`: list, create, get, update, delete, access management
- `/api/board/{id}/workshop/shapes`: list, create, update, delete workshop shapes
- `/api/package`: list, create, get, update, delete, access management
- `/api/package/{id}/workshop/{shapes|lines|rules|layouts}`: list, create, update, delete workshop items
- `/api/workshop/{shapes|lines|rules|layouts}`: global read-only discovery
- `/api/session`: signin, verify, me, signout

## 4) Runtime components (frontend)

Folder `ui/web/src/`:

- Entry/app: `index.tsx`, `App.tsx`, `Routing.tsx`
- Features:
  - Boards: `boards/` (pages, forms, model, providers)
  - Packages: `package/` (pages, forms, model, providers)
  - Workshop: `common/workshop/` (components, model, providers, API for shapes, lines, rules, layouts)
  - Session: `session/` (session UI, forms, model, providers)
- Shared: `common/`, `assets/`

Key deps:

- `solid-js`, `@solidjs/router`, `@modular-forms/solid`, `valibot`, `lucide-solid`, `vite`, `vite-plugin-solid`, `tailwindcss`

Testing (high-level): Playwright E2E tests run critical UI flows against a running backend. See Testing section and `testing_overview.md` for commands/setup.

## 5) Request lifecycle

1. Browser calls API with optional Authorization cookie (Bearer JWT)
2. Router extracts token via `AuthToken` and validates request body via `ValidatedJson<T>`
3. Router delegates to Service via dependency injection (Arc&lt;T&gt;)
4. Service applies rules using TransactionStarter pattern for database operations
5. Service leverages CommonItemAccess for access control validation
6. Store (implementing CrudQueries trait) performs SQLx operations against SQLite
7. Response is mapped to API DTO and returned

## 6) Domain features

Boards

- Entity: id (i64), name, description?, icon?, is_public
- Visibility: public; private requires valid session and access control via CommonItemAccess
- Architecture: Implements CrudService and CrudQueries traits with TransactionStarter pattern
- DTOs: `Board`, `NewBoardItem`, `PatchBoardItem` (partial with explicit null handling)

Packages

- Entity: id (i64), name, description?, icon?, is_public  
- Visibility: public; private requires valid session and access control via CommonItemAccess
- Architecture: Implements CrudService and CrudQueries traits with TransactionStarter pattern
- DTOs: `Package`, `NewPackageItem`, `PatchPackageItem` (partial with explicit null handling)
- Note: Boards feature module now mirrors this structure exactly

Workshop Items

- Types: shapes, lines, rules, layouts (diagram element blueprints)
- Global: Read-only discovery via `/api/workshop/{shapes|lines|rules|layouts}`
- Entity-specific: CRUD operations within packages and boards context
  - Package routes: `/api/package/{id}/workshop/{shapes|lines|rules|layouts}`
  - Board routes: `/api/board/{id}/workshop/{shapes|lines|rules|layouts}` (shapes only currently)
- Architecture: Generic `WorkshopService` with type-specific `WorkshopStore` instances
- Association tables: `package_shape`, `package_line`, `package_rule`, `package_layout`
- DTOs: `WorkshopItem`, `NewWorkshopItem`, `PatchWorkshopItem`
- Features: slug-based semantic identifiers, JSON definition storage, board imports from packages

Sessions/Auth (passwordless)

- POST `/api/session/signin`: generates code (stubbed), sends via `ExtComm`
- POST `/api/session/verify`: validates code; returns JWT in HttpOnly; SameSite=Strict cookie
- GET `/api/session/me`: returns session data if token present
- POST `/api/session/signout`: clears cookie

## 7) Data & persistence

- SQLite file: `server/ws/sqlite.db`
- Pool: `db::SqlitePool` (WAL mode)
- Migrations: `server/ws/migrations/*.sql`

## 8) Configuration

`config.rs` loads defaults -> optional `config.toml` -> env overrides.

Relevant `WS_*` envs:

- `WS_HOST`, `WS_PORT`, `WS_LOG_LEVEL`
- `WS_DATABASE_URL`, `WS_DATABASE_POOL`
- `WS_APP_JWT_EXPIRATION_SECS`, `WS_APP_JWT_SECRET`

## 9) Source layout (essentials)

- Backend
  - `server/ws/src/main.rs`
  - `server/ws/src/lib.rs`
  - Feature folders: `boards/`, `package/`, `session/`
  - Infra: `config.rs`, `db.rs`, `jwt_adapter.rs`, `ext_comm.rs`, `api/`, `error.rs`
  - DB: `server/ws/sqlite.db`, `server/ws/migrations/`
- Frontend
  - `ui/web/src/` (feature-first folders), `vite.config.ts`, `package.json`
- Docs
  - `r&d/dev-docs/` (this file, `code-overview.md`, `openapi.yaml`)

## 10) Tooling & tasks

Backend (VS Code tasks):

- `cargo-build`: build server
- `cargo-test-build`: compile tests
- `cargo-run`: run server (env in task options)

Frontend (npm scripts):

- `npm run dev`, `npm run build`, `npm run preview`

Testing (quick commands):

- Backend integration tests: `cd server/ws && cargo test`
- Frontend E2E tests: `cd ui/web && npm install && npx playwright install && npm run test:e2e`

## 11) Security & validation

- JWT tokens issued by `SessionJWTService` using `JwtAdapter`
- Token returned as secure HttpOnly cookie; SameSite=Strict; path=/
- Input validation via `validator` + custom extractor `ValidatedJson`

## 12) Error handling

- Domain errors unified in `error::ModelError`
- Routers map model errors to `(StatusCode, String)` results

## 13) Extensibility

- Replace `ExtComm` stub with real provider
- Add role-based permissions per route/entity
- OpenAPI-first contracts and typed clients
- Integration/e2e tests

## 14) Testing

For full testing instructions and scope, see Testing Overview:

- r&d/dev-docs/testing/testing_overview.md
- Direct link: `./testing/testing_overview.md`

Summary:

- Backend integration tests live in `server/ws/tests/` and run the Axum router in-process with a temporary SQLite database and migrations.
- Frontend E2E tests live in `ui/web/tests-e2e/` and use Playwright to drive the SPA against a running backend.

