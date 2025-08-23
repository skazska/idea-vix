---
description: project GitHub Copilot instructions
---
applyTo: "**"
---
# General Coding Standards
- Modularity and Extensibility.
- Type Safety and Consistency.
- Error Handling and Resilience.
- Dependency Flow, No Circular Dependencies.
- Layered Architecture.
- Code Clarity and Readability.
- Consistent Naming Conventions.
- Code Comments and Annotations.


## Testing
- Comprehensive Testing.
- Unit Testing for Utilities.


## Requirements
- use `r&d/analysis/stories/core.md` for core feature requirements and implementation status. 


## Documentation
- Clear and Concise Documentation.
- Use `r&d/dev-docs/openapi.yaml` for API documentation.
- Keep API documentation up-to-date with code changes.
- Use `r&d/dev-docs/storage.dbml` for database schema documentation.
- Keep database documentation in sync with migrations.

## Architecture
- Backend: See `.github/instructions/server.instructions.md`
- Frontend: See `.github/instructions/webapp.instructions.md`
- Auth Flow: Email-based sessions via JWT cookies (no passwords)

## System Overview

```
Browser (SolidJS) -> HTTP/JSON -> Axum Routers -> Services ...
                                 |                   |
                                 |-- JWT (cookie) ---|
```

## Cross-Tier Patterns
- use RESTful API docs: `r&d/dev-docs/openapi.yaml`
- JSON payloads
- HTTP-only cookies: JWT tokens for session management
- Error responses: Consistent error format across all endpoints

## Configuration & Environment
- **Frontend**: `VITE_*` prefixed for client-side variables
-
## Testing Strategy

### Integration Focus
- **Frontend**: End-to-end Playwright tests for critical user journeys
- **Test isolation**: Each test gets clean database state

### Test Data
- **Auth**: Use test helper `auth_cookie_for(router, "email")` for authenticated requests
- **Database**: Temp SQLite files with automatic migrations for each test
- **E2E**: Requires backend server running during frontend tests

## Code Standards

### Modularity Principles
- **No circular dependencies** between feature modules
- **Clear layer boundaries**: Presentation → Application → Persistence
- **Feature isolation**: Boards, packages, sessions don't directly depend on each other
- **Common utilities**: Shared modules (`error`, `db`, `config`) stay dependency-free

### Documentation Standards
- **Architecture docs**: High-level patterns in `r&d/dev-docs/`
- **Code docs**: Module-level documentation for business logic
- **API docs**: Endpoint documentation in router modules
- **Examples**: Include usage examples in complex utilities

