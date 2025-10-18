# Research: Board Add Package Feature

## Analysis of Definition Content

### Core Requirement
Allow board owners to add packages to a board, establishing a many-to-many relationship through the existing `board_package` junction table.

### Existing Infrastructure

#### Database Schema
The `board_package` junction table already exists (created in migration `20250727044835_create_boards.up.sql`):
```sql
CREATE TABLE `board_package` (
    `board_id` INTEGER NOT NULL,
    `package_id` INTEGER NOT NULL,
    PRIMARY KEY (board_id, package_id),
    FOREIGN KEY (board_id) REFERENCES board(id),
    FOREIGN KEY (package_id) REFERENCES package(id)
);
```

This table:
- Has composite primary key preventing duplicates
- Has foreign keys for referential integrity
- Requires no migration changes

#### Backend Architecture Pattern
Both `boards.rs` and `package.rs` modules follow the same 3-layer architecture:

1. **Module Router** (`boards.rs`, `package.rs`):
   - Uses macros (`resource_router!`, `access_router!`, `workshop_item_router!`) for route setup
   - Handlers extract `AuthToken`, `State`, `Path`, `ValidatedJson`
   - Handlers convert results to `Json<T>` or error tuples

2. **Service Layer** (`board_service.rs`, `package_service.rs`):
   - Implements `CrudService` trait
   - Uses `TransactionStarter` for DB transactions
   - Enforces business logic and access control
   - Converts between API models and DB models

3. **Store Layer** (`board_store.rs`, `package_store.rs`):
   - Implements `CrudQueries` trait
   - Executes SQL via `sqlx`
   - Works within transactions (`Trx`)

#### Similar Implementation: Board Access Management
The `/api/board/{id}/access` endpoints provide a reference pattern:
- `POST /api/board/{id}/access` - Add access (requires owner)
- `GET /api/board/{id}/access` - List access (requires owner)
- `DELETE /api/board/{id}/access/{address}` - Revoke access (requires owner)

Uses `CommonItemAccess` service with `SqliteItemAccessQueries` store.

#### Workshop Items Pattern
Workshop items (shapes, lines, rules, layouts) use `WorkshopEntityService`:
- Routes: `/api/board/{id}/workshop/shapes`, `/api/package/{id}/workshop/shapes`
- Operations: list, add, update, remove
- Pattern can be adapted for package associations

### Code References

#### Backend Files to Modify/Create
1. `server/ws/src/boards.rs` - Add package routes (nest under `/{id}/packages`)
2. `server/ws/src/boards/board_service.rs` - Add package management methods
3. `server/ws/src/boards/board_store.rs` - Add package association queries
4. Or create new files:
   - `server/ws/src/boards/board_package.rs` - Package association handlers
   - `server/ws/src/boards/board_package_service.rs` - Business logic
   - `server/ws/src/boards/board_package_store.rs` - DB queries

#### Frontend Files to Modify/Create
1. `ui/web/src/boards/providers/api.ts` - Add package API methods
2. `ui/web/src/boards/Board.tsx` - Add package management UI
3. `ui/web/src/boards/` - Potentially create `BoardPackages.tsx` component

### Key Patterns to Follow

#### Authorization
- Must validate board ownership (not just manage role)
- Must validate package accessibility (public OR user has access to it)
- Two-level check: board ownership AND package visibility

#### Error Handling
- 401: Not authenticated
- 403: Not board owner
- 404: Board or package not found
- 409: Package already added to board (caught by unique constraint)

#### Transaction Management
- Use `TransactionStarter::run_in_transaction` for database operations
- Ensures atomicity if multiple queries needed

#### API Models vs DB Models
- Define separate structs for API payloads (`NewBoardPackageRequest`)
- Define separate structs for DB operations (`BoardPackageDb`)
- Convert between layers

## Open Questions & Answers

### Q1: Should we return the added package or full list?
**A:** Return full list of packages associated with the board, consistent with access endpoints pattern.

### Q2: Create a new service or extend BoardService?
**A:** Create dedicated methods in BoardService for package management (like access methods), but potentially use a separate store module for board_package queries.

### Q3: What validation is needed when adding a package?
**A:**
1. Board exists and user is owner
2. Package exists and is accessible (public OR user has access)
3. Package not already associated (DB constraint will catch this)

### Q4: How to list available packages for selection?
**A:** Reuse existing `GET /api/package` which already filters by visibility. Frontend can exclude already-added packages in UI.

## Uncertainties & Challenges

### Challenge 1: Authorization Complexity
Need to check TWO permissions:
1. User is board owner (via board_access_roles)
2. Package is visible to user (public OR user has access via package_access_roles)

**Approach:** 
- Reuse `PackageService::get_item()` to validate package accessibility
- Reuse access validation patterns from `CommonItemAccess`

### Challenge 2: Consistent Error Messages
Should distinguish between:
- Package not found
- Package found but not accessible
- Package already added

**Approach:** Let package get_item throw 404 if not accessible, let DB throw conflict error for duplicates.

### Challenge 3: UI/UX Design
How to present package selection without cluttering board view?

**Approach:**
- Add "Packages" tab/section in board view (similar to workshop items)
- Show list of added packages
- "Add Package" button opens modal with searchable package list
- Filter out already-added packages in the modal

## References to Related Code

### Backend
- `server/ws/src/boards/board_service.rs` - Access control patterns
- `server/ws/src/common/access.rs` - `CommonItemAccess` implementation
- `server/ws/src/workshop/entity_service.rs` - Workshop item association pattern
- `server/ws/src/package/package_service.rs` - Package visibility logic

### Frontend
- `ui/web/src/boards/Board.tsx` - Board view component
- `ui/web/src/boards/Items.tsx` - Workshop items management (reference UI)
- `ui/web/src/boards/providers/api.ts` - Board API client
- `ui/web/src/package/providers/api.ts` - Package API client

### Tests
- `server/ws/tests/boards_smoke.rs` - Board CRUD and access tests
- `server/ws/tests/packages_smoke.rs` - Package CRUD tests
- `server/ws/tests/boards_workshop_*.rs` - Workshop item tests (pattern reference)
- `ui/web/tests-e2e/boards_flow.spec.ts` - E2E board tests

## Summary

The infrastructure is largely in place:
- Database table exists
- Backend patterns are well-established
- Frontend patterns exist for similar features

Main implementation tasks:
1. Create board-package association endpoints following workshop items pattern
2. Implement service logic with proper authorization checks
3. Create UI components similar to workshop items management
4. Write comprehensive tests (integration + E2E)
5. Update API documentation
