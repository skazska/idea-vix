---
description: project GitHub Copilot instructions
---
applyTo: "**"
---

Backend details: See `.github/instructions/server.instructions.md`
Frontend details: See `.github/instructions/webapp.instructions.md`

# General Coding Standards Focus
- Modularity, Extensibility, Reusability.
- Type Safety and Consistency.
- Error Handling and Resilience.
- Dependency Flow, No Circular Dependencies.
- Layered Architecture.
- Code Clarity and Readability.
- Consistent Naming Conventions.
- Code Comments and Annotations.

# Testing
Testing as important as coding.
When plan, implement, run tests, consider: `r&d/dev-docs/testing/testing_overview.md`

# Project Documentation
- `r&d/dev-docs/openapi.yaml` - API - is a contract between frontend and backend.

- `r&d/analysis/stories/*` - core feature concept, user stories, status.
- `r&d/analysis/requirements/*` - requirements. 
- `r&d/dev-docs/storage.dbml` - database schema.
- `r&d/dev-docs/project-implementation-overview.md` - project implementation overview.
- Keep documents up-to-date with code changes.

# System Overview

```
Browser (SolidJS) -> HTTP/JSON -> Axum Routers -> Services ...
                                 |-- JWT (cookie) ---|
```

- Auth Flow: Email-based sessions via JWT cookies (no passwords)
- RESTful API: `r&d/dev-docs/openapi.yaml`
- JSON payloads
- HTTP-only cookies: JWT tokens for session management
- Error responses: Consistent error format across all endpoints

## Dev log
Code change iterations must follow dev log process: `r&d/dev-log/README.md` - dev log structure and usage


