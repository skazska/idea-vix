# Testing Plan: Board Add Package Feature

## Test Strategy

Following `r&d/dev-docs/testing/testing_overview.md`, we will implement tests at multiple levels:

1. **Backend Integration Tests** - Test API endpoints with database
2. **Frontend E2E Tests** - Test complete user flows with Playwright

## Backend Integration Tests

### File: `server/ws/tests/boards_packages.rs`

#### Test Cases

##### 1. Happy Path Tests

**`test_list_board_packages_empty`**
- Setup: Create board
- Action: GET /api/board/{id}/packages
- Assert: Returns empty array

**`test_add_package_to_board_ok`**
- Setup: Create board (as user A), create public package (as user B)
- Action: POST /api/board/{board_id}/packages with package_id
- Assert: Returns array with one package, package details are correct

**`test_add_own_private_package`**
- Setup: Create board and private package (same user)
- Action: POST /api/board/{board_id}/packages
- Assert: Successfully adds package

**`test_list_board_packages_ok`**
- Setup: Board with 2 packages added
- Action: GET /api/board/{id}/packages
- Assert: Returns array with both packages

**`test_remove_package_ok`**
- Setup: Board with package added
- Action: DELETE /api/board/{id}/packages/{package_id}
- Assert: Returns empty array, package removed

##### 2. Authorization Tests

**`test_add_package_requires_auth`**
- Setup: Create board and package
- Action: POST /api/board/{id}/packages (no auth token)
- Assert: 401 Unauthorized

**`test_add_package_requires_owner_role`**
- Setup: Create board (user A), create package (user B), grant user B 'manage' role on board
- Action: POST /api/board/{id}/packages (as user B)
- Assert: 403 Forbidden (only owner can add packages)

**`test_list_packages_requires_access`**
- Setup: Create private board with packages (user A)
- Action: GET /api/board/{id}/packages (as user B, no access)
- Assert: 404 Not Found (board not accessible)

**`test_remove_package_requires_owner`**
- Setup: Board with package, user B has 'manage' role
- Action: DELETE /api/board/{id}/packages/{package_id} (as user B)
- Assert: 403 Forbidden

##### 3. Validation Tests

**`test_add_package_invalid_package_id`**
- Action: POST with package_id = 0 or negative
- Assert: 400 Bad Request

**`test_add_package_board_not_found`**
- Action: POST /api/board/99999/packages
- Assert: 404 Not Found

**`test_add_package_package_not_found`**
- Setup: Create board
- Action: POST with non-existent package_id
- Assert: 404 Not Found

**`test_add_package_not_accessible`**
- Setup: Create board (user A), create private package (user B)
- Action: POST /api/board/{id}/packages (as user A, trying to add B's private package)
- Assert: 404 Not Found (package not accessible)

**`test_add_package_duplicate`**
- Setup: Board with package already added
- Action: POST /api/board/{id}/packages with same package_id
- Assert: 409 Conflict

**`test_remove_package_not_in_board`**
- Setup: Board with no packages, valid package exists
- Action: DELETE /api/board/{id}/packages/{package_id}
- Assert: 200 OK, returns empty array (idempotent)

##### 4. Isolation Tests

**`test_package_removal_doesnt_affect_other_boards`**
- Setup: Package added to board A and board B
- Action: Remove package from board A
- Assert: Package still in board B

**`test_board_deletion_removes_package_associations`**
- Setup: Board with packages
- Action: DELETE /api/board/{id}
- Assert: board_package rows deleted (CASCADE)

## Frontend E2E Tests

### File: `ui/web/tests-e2e/boards_packages_flow.spec.ts`

#### Test Flows

**`Board owner can view packages section`**
- Login as user
- Create board
- Navigate to board view
- Assert: "Packages" section visible
- Assert: "Add Package" button visible (owner only)

**`Board owner can add public package`**
- Setup: Public package exists
- Login, create board, navigate to board view
- Click "Add Package"
- Assert: Package selection modal opens
- Select package, click Add
- Assert: Package appears in board's package list
- Assert: Success feedback shown

**`Board owner can add own private package`**
- Login, create private package
- Create board, navigate to board view
- Add own private package
- Assert: Package added successfully

**`Board owner cannot add inaccessible package`**
- Setup: Another user's private package exists
- Login, create board
- Open package selection
- Assert: Inaccessible package NOT in available list

**`Non-owner cannot add packages`**
- Setup: User A creates board, grants user B 'view' role
- Login as user B, navigate to board
- Assert: "Add Package" button NOT visible

**`Board owner can remove package`**
- Setup: Board with package
- Navigate to board view
- Click remove button on package
- Assert: Confirmation dialog shown
- Confirm removal
- Assert: Package removed from list

**`Package search and filtering works`**
- Setup: Multiple packages exist
- Open package selection modal
- Type in search box
- Assert: List filters by package name

**`Already-added packages are indicated`**
- Setup: Board with 1 package, other packages exist
- Open package selection modal
- Assert: Added package shows as "Added" or disabled

**`Package list updates after add`**
- Add package
- Assert: Package list refreshes and shows new package

## Test Data Setup Patterns

### Helper Functions

```rust
// Backend test helpers
async fn create_board_with_packages(
    app: &TestApp,
    owner_token: &str,
    num_packages: usize
) -> (Board, Vec<Package>)

async fn grant_package_access(
    app: &TestApp,
    package_id: i64,
    address: &str,
    role: &str,
    owner_token: &str
)
```

```typescript
// Frontend test helpers
async function createBoardWithPackages(
    page: Page,
    numPackages: number
): Promise<{ board: Board, packages: Package[] }>

async function addPackageToBoard(
    page: Page,
    boardSlug: string,
    packageName: string
)
```

## Success Criteria

### Backend
- All integration tests pass
- Test coverage > 90% for new code
- All error cases handled with appropriate status codes

### Frontend
- All E2E tests pass
- UI responsive and accessible
- Loading and error states properly displayed
- Works across different screen sizes

## Test Execution

### Local Development
```bash
# Backend integration tests
cd server/ws
cargo test boards_packages

# Frontend E2E tests
cd ui/web
npm run test:e2e -- boards_packages_flow
```

### CI Pipeline
- Both test suites run on every commit
- Tests must pass before merge

## Notes

- Use consistent test naming convention
- Include both positive and negative test cases
- Test edge cases (empty lists, duplicates, etc.)
- Test authorization thoroughly (owner vs. other roles)
- Ensure tests are isolated and can run in any order
- Mock external dependencies if needed
- Use descriptive assertion messages
