# CRUD

This document captures the implemented CRUD contract for boards and packages.

Legend:

- `[v]` implemented and verified by the current service + OpenAPI contract.
- `[~]` partially implemented or planned follow-up.
- `[FUTURE]` not yet implemented but identified as desirable.

## Shared CRUD Contract

As Crud operations for package is similar to board crud operations, shared CRUD contract may apply.
So implementations also follow idea of shared CRUD functionality.
Specifics may be implemented per entity as needed.


- REST resources expose list, create, read, update, delete operations for both `board` and `package` entities under `/api/board` and `/api/package`. See `r&d/dev-docs/openapi.yaml#/paths`. [v]
- `DELETE` handlers return the deleted entity payload with `200 OK`, enabling callers to update caches without second lookup. [v]
- Public visibility applies to unauthenticated callers: only items with `is_public=true` are returned when no valid JWT is present; authenticated callers get public + invited items. [v]
- Creation defaults `is_public` to `false` unless explicitly overridden in the request body. [v]
- Creation automatically assigns the caller the `owner` access role for the new entity. [v]
- Update endpoints support sparse payloads; omitted fields remain unchanged, while `description`/`icon` accept explicit `null` to clear stored values. [v]

## Shared Validation Rules

As Crud operations for package is similar to board crud operations, shared validation rules may apply.
So implementations also follow idea of shared validation functionality.
Specifics may be implemented per entity as needed.

- `name` must be 3–100 characters for both new and patch payloads. [v]
- `slug`, when provided on create, must match the pattern `^[a-z][a-z0-9-]*[a-z0-9]$|^[a-z]$` (documented in `openapi.yaml` and implemented in `server/ws/src/common/slug.rs`). [v]
- `description` is optional and capped at 500 characters. [v]
- `icon` is optional and capped at 255 characters. [v]
- Patch payloads use nested options (`Option<Option<String>>`) to differentiate “not provided” vs. “clear stored value”, enforced via `deserialize_some`. [v]

## Slug Behaviour

Both board and package entities implement workshop management and items-templates needs to be referenced by slug and be distinguishable among all packages and boards.
So all entities need to have slugs.

- Slugs are generated with `generate_slug` when the client omits a value, ensuring lowercase, hyphenated identifiers that always start with a letter. [v]
- Slugs are immutable post-creation; update DTOs exclude the `slug` field. [v]
- Slugs are unique per entity type, backed by `idx_board_slug` and `idx_package_slug` unique indexes; attempts to reuse a slug surface as HTTP `409 Conflict`. [v]
- Slugs serve as semantic identifiers for diagram references alongside package/board identifiers (see `../stories/diagrams.md#drawing`). [v]

## Shared Access Control & Errors

Unified access control and error handling applies to both boards and packages.
So implementations also follow idea of shared access control and error handling functionality.

- List/read endpoints accept optional JWT cookies; private items require the caller to hold a role via invitation. [v]
- Create requires an authenticated session (`401` otherwise). [v]
- Update requires `owner`, `manage`, or `edit` role; failure yields `403`. [v]
- Delete endpoints reuse `CommonItemAccess` role checks; packages require the `owner` role, boards accept `owner` or `manage` per current service logic. [v]
- Access management (grant/list/revoke) is restricted to owners and returns `409` on duplicate grant attempts. [v]
- Access role persistence happens in the same transaction as the CRUD write to keep invitation tables (`*_access_roles`) consistent. [v]
- Error mapping follows the shared API responses declared in `openapi.yaml` (`400` validation, `401` authentication, `403` authorization, `404` not found/inaccessible, `409` uniqueness conflicts, `500` unexpected failures). [v]

## Package-Specific Notes

## Board-Specific Notes
