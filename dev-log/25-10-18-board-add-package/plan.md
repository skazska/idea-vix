# Plan: Implement Board Add Package Feature

## Overview

Implement endpoints and UI for board owners to add/remove packages from boards, following established patterns from workshop items and access management.

## Tasks

### Backend Implementation

#### Task 1: Add Database Query Functions
**File:** `server/ws/src/boards/board_store.rs`

Add queries for board_package junction table:

```rust
// Add these methods to BoardStore impl

/// Get all packages associated with a board
pub async fn get_board_packages(&self, board_id: i64, trx: &mut Trx) -> Result<Vec<i64>, DbErr>

/// Add a package to a board
pub async fn add_board_package(&self, board_id: i64, package_id: i64, trx: &mut Trx) -> Result<(), DbErr>

/// Remove a package from a board
pub async fn remove_board_package(&self, board_id: i64, package_id: i64, trx: &mut Trx) -> Result<(), DbErr>

/// Check if package is associated with board
pub async fn is_package_in_board(&self, board_id: i64, package_id: i64, trx: &mut Trx) -> Result<bool, DbErr>
```

**SQL:**
- GET: `SELECT origin_id FROM board_package WHERE board_id = ?`
- ADD: `INSERT INTO board_package (board_id, origin_id) VALUES (?, ?)`
- REMOVE: `DELETE FROM board_package WHERE board_id = ? AND origin_id = ?`
- CHECK: `SELECT COUNT(*) FROM board_package WHERE board_id = ? AND origin_id = ?`

---

#### Task 2: Add Service Layer Methods
**File:** `server/ws/src/boards/board_service.rs`

Add package management methods to `BoardService`:

```rust
/// Get all packages associated with a board (uses PackageService for full package data)
pub async fn list_board_packages(&self, board_id: i64, session: Option<&SessionData>) -> Result<Vec<Package>, ModelError>

/// Add a package to a board
/// - Validates board ownership
/// - Validates package accessibility
/// - Adds association
pub async fn add_board_package(&self, board_id: i64, package_id: i64, session: &SessionData) -> Result<Vec<Package>, ModelError>

/// Remove a package from a board
/// - Validates board ownership
/// - Removes association
pub async fn remove_board_package(&self, board_id: i64, package_id: i64, session: &SessionData) -> Result<Vec<Package>, ModelError>
```

**Logic:**
1. Check board access (owner role required)
2. For add: verify package exists and is accessible (call PackageService::get_item)
3. Perform operation in transaction
4. Return updated list of board packages

---

#### Task 3: Add HTTP Handlers
**File:** `server/ws/src/boards.rs`

Add route handlers:

```rust
/// GET /api/board/{id}/packages - List packages associated with board
async fn list_board_packages(
    AuthToken(token): AuthToken,
    State(state): State<Arc<(Arc<SessionJWTService>, BoardService)>>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<Package>>, (StatusCode, String)>

/// POST /api/board/{id}/packages - Add package to board
/// Body: { "package_id": number }
async fn add_board_package(
    AuthToken(token): AuthToken,
    State(state): State<Arc<(Arc<SessionJWTService>, BoardService)>>,
    Path(board_id): Path<i64>,
    ValidatedJson(request): ValidatedJson<AddPackageRequest>,
) -> Result<Json<Vec<Package>>, (StatusCode, String)>

/// DELETE /api/board/{id}/packages/{package_id} - Remove package from board
async fn remove_board_package(
    AuthToken(token): AuthToken,
    State(state): State<Arc<(Arc<SessionJWTService>, BoardService)>>,
    Path((board_id, package_id)): Path<(i64, i64)>,
) -> Result<Json<Vec<Package>>, (StatusCode, String)>
```

**Request/Response Models:**
```rust
#[derive(Deserialize, Validate)]
pub struct AddPackageRequest {
    #[validate(range(min = 1))]
    pub package_id: i64,
}
```

---

#### Task 4: Update Router
**File:** `server/ws/src/boards.rs` in `get_router()` function

Add nested route:
```rust
.nest("/{id}/packages", Router::new()
    .route("/", get(list_board_packages).post(add_board_package))
    .route("/{package_id}", delete(remove_board_package))
    .with_state(state.clone())
)
```

---

#### Task 5: Inject PackageService Dependency
**File:** `server/ws/src/boards/board_service.rs`

Update `BoardService` to have access to packages:
```rust
pub struct BoardService {
    transaction_starter: Arc<TransactionStarter>,
    items_store: Arc<BoardStore>,
    access_store: Arc<SqliteItemAccessQueries>,
    package_service: Arc<PackageService>, // NEW
}
```

Update `get_router()` to pass PackageService.

---

### Testing Implementation

#### Task 6: Backend Integration Tests
**File:** `server/ws/tests/boards_packages.rs` (new file)

Test cases:
1. `test_add_package_to_board_ok` - Happy path
2. `test_add_package_requires_auth` - 401 when not authenticated
3. `test_add_package_requires_owner` - 403 when not owner
4. `test_add_package_not_found` - 404 when package doesn't exist
5. `test_add_package_not_accessible` - 404 when package is private and user has no access
6. `test_add_package_duplicate` - 409 when package already added
7. `test_remove_package_ok` - Remove package successfully
8. `test_remove_package_not_in_board` - Handle remove when not associated
9. `test_list_board_packages_ok` - List packages for board
10. `test_list_board_packages_empty` - List when no packages added

---

#### Task 7: E2E Tests
**File:** `ui/web/tests-e2e/boards_packages_flow.spec.ts` (new file)

Test flows:
1. Board owner can add public package to board
2. Board owner can add own private package to board
3. Board owner cannot add inaccessible private package
4. Non-owner cannot add packages
5. Added packages appear in board's package list
6. Board owner can remove package from board
7. Removing package doesn't affect other boards using same package

---

### Frontend Implementation

#### Task 8: API Client Methods
**File:** `ui/web/src/boards/providers/api.ts`

Add methods to `BoardApi` class:

```typescript
export async function listBoardPackages(backend: IBackend, boardId: string): Promise<Package[]>
export async function addBoardPackage(backend: IBackend, boardId: string, packageId: number): Promise<Package[]>
export async function removeBoardPackage(backend: IBackend, boardId: string, packageId: number): Promise<Package[]>

// In BoardApi class:
public listPackages = query((id: string) => listBoardPackages(this.backend, id), `${ENTITIES_NAME}_packages`)
public addPackage = query((id: string, packageId: number) => addBoardPackage(this.backend, id, packageId), `add_${ENTITIES_NAME}_package`)
public removePackage = query((id: string, packageId: number) => removeBoardPackage(this.backend, id, packageId), `remove_${ENTITIES_NAME}_package`)
```

---

#### Task 9: Board Packages Component
**File:** `ui/web/src/boards/BoardPackages.tsx` (new file)

Create component to:
- Display list of packages associated with board
- Show "Add Package" button (owner only)
- Show remove button for each package (owner only)
- Open modal/dialog for package selection

```typescript
interface BoardPackagesProps {
  boardId: string;
  isOwner: boolean;
}

export default function BoardPackages(props: BoardPackagesProps)
```

---

#### Task 10: Package Selection Modal
**File:** `ui/web/src/boards/PackageSelectionModal.tsx` (new file)

Create modal component to:
- List available packages (from /api/package)
- Filter out already-added packages
- Provide search/filter functionality
- Handle add action
- Show loading states and errors

---

#### Task 11: Integrate into Board View
**File:** `ui/web/src/boards/Board.tsx`

Add:
- Packages tab/section
- Render `<BoardPackages>` component
- Pass owner status from access roles

---

### Documentation

#### Task 12: Update OpenAPI Specification
**File:** `r&d/dev-docs/openapi.yaml`

Add endpoints:
- `GET /api/board/{id}/packages`
- `POST /api/board/{id}/packages`
- `DELETE /api/board/{id}/packages/{package_id}`

Define schemas:
- `AddPackageRequest`: `{ package_id: number }`
- Update `Package` schema if needed

---

#### Task 13: Update Traceability Matrix
**File:** `r&d/analysys/requirements/traceability-matrix.md`

Add new row for "add package to board" user story with:
- Acceptance criteria
- Test coverage (backend + frontend)
- Status

---

#### Task 14: Update Core Stories
**File:** `r&d/analysys/stories/core.md`

Update status from `[]` to `[v]` for:
- "add package to board" story

---

## Implementation Order

1. **Backend Foundation** (Tasks 1-4):
   - Add store queries
   - Add service methods with PackageService dependency injection
   - Add HTTP handlers
   - Update router

2. **Backend Tests** (Task 6):
   - Write integration tests
   - Ensure all tests pass

3. **Frontend API** (Task 8):
   - Add API client methods

4. **Frontend Components** (Tasks 9-11):
   - Create BoardPackages component
   - Create PackageSelectionModal
   - Integrate into Board view

5. **E2E Tests** (Task 7):
   - Write Playwright tests
   - Ensure all flows work

6. **Documentation** (Tasks 12-14):
   - Update OpenAPI spec
   - Update traceability matrix
   - Update core stories

## Dependencies Between Tasks

- Task 2 depends on Task 1 (service needs store methods)
- Task 3 depends on Task 2 (handlers need service methods)
- Task 5 needed for Task 2 (service needs PackageService)
- Task 6 depends on Tasks 1-4 (tests need implementation)
- Task 8 depends on Tasks 1-4 (API needs backend endpoints)
- Tasks 9-11 depend on Task 8 (UI needs API)
- Task 7 depends on Tasks 1-11 (E2E needs full implementation)
- Tasks 12-14 can be done anytime after design is clear

## Testing Strategy

### Unit/Integration Level (Backend)
- Test each endpoint with valid/invalid inputs
- Test authorization (authenticated, owner, non-owner)
- Test error cases (not found, conflict, forbidden)
- Test with public and private packages
- Test duplicate prevention

### E2E Level (Frontend)
- Test complete user flows
- Test UI interactions (buttons, modals, lists)
- Test visual feedback (loading, errors, success)
- Test different user roles

## Rollout Plan

1. Implement backend (Tasks 1-5)
2. Run backend tests (Task 6)
3. Implement frontend (Tasks 8-11)
4. Run E2E tests (Task 7)
5. Update documentation (Tasks 12-14)
6. Code review
7. Merge to main branch

## Notes

- Follow existing patterns from workshop items and access management
- Column name in DB is `origin_id` not `package_id`
- Return full package list after add/remove for consistency
- UI should only show to board owners (check access roles)
- No migration needed - table already exists
