---
description: project GitHub Copilot instructions
---
applyTo: "**"
---

# Code change iterations must follow dev log process:

Prompt starts with line:
 `fix`|`feat`|`refactor` - New change iteration
 `go {some name}` - Continue iteration with name if it exists and is not completed

[dev log index file](../dev-log/index.md) contains:
name,date(YY-MM-DD),purpose,status(draft,in-progress,completed),ref detail files  -  required to be added when new change iteration requested.

Iteration details are in folder named `YY-MM-DD-{name}`.

1. `definition.md` - to define the iteration:

- purpose and goals or problem (MUST)
- definition of done (SHOULD)
- description of the feature or change (SHOULD)
- references to focus documents, code, data, documentation (SHOULD)
- requirements and specifications (SHOULD)
- any problems, open questions, uncertainties, challenges anticipated (COULD)
- any relevant diagrams or illustrations (COULD)

2. `research.md` - To register results of research or analysis related to the iteration (should pass an overview):

- analysis of definition content (considering also related code, data, project documentation)
- refs to: code, data, project documentation parts, stressed or referenced in definition.md
- ask questions, note uncertainties, challenges
- refs to any spikes, prototypes, experiments done

3. `plan.md` - To outline and sync up on an approach, tasks, and description of what, how and why to be done: changing, adding, removing, actualizing tests, code, data, documentation, other artifacts (should pass overview). 

for code change, it should look like:
- tests related tasks
- code related tasks
- testing related tasks
- documentation related tasks

4. `testing.md` - To plan and log the testing process and decisions on done according to the definition of done or goals.

5. `implementation.md` - To log the actual implementation details and progress according to the plan (should be actualized per each task done):

- tasks with status
- any deviations from the plan with reasons
- any problems, issues, blockers encountered
- references to code changes, commits, pull requests

6. `status.md` - To log  any problems, issues encountered which is not related to iteration and next steps.


Backend instructions details: See `.github/instructions/server.instructions.md`
Frontend instructions details: See `.github/instructions/webapp.instructions.md`

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
- 'server/ws/README.md'/'ui/web/README.md' - backend/frontend setup and implementation overview.
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
