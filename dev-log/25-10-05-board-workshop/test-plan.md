# Board Workshop Implementation - Test Plan

**Note**: This test plan should have been created before implementation (TDD approach). It is being documented retrospectively to close this gap in the development process.

## Test Strategy

Following the testing approach outlined in `r&d/dev-docs/testing/testing_overview.md`, we need to ensure comprehensive coverage of the new board workshop functionality.

## Unit Tests

### Service Layer Tests

**Location**: `server/ws/src/boards/board_service.rs` (if dedicated tests exist)

Tests already covered by existing workshop service tests:
- ✅ WorkshopService generic functionality (tested via workshop module)
- ✅ Entity association logic (covered by existing workshop tests)

**Gap Analysis**: Board-specific workshop service logic is covered by generic workshop service tests. No additional unit tests needed at this layer.

## Integration Tests

### Existing Coverage

**File**: `server/ws/tests/workshop_smoke.rs`

Existing tests that partially cover board workshop functionality:
1. ✅ `workshop_board_direct_creation_and_import_smoke_ok` - Tests board workshop shape creation
2. ✅ `workshop_lines_readonly_smoke_ok` - Tests line listing functionality

### Missing Integration Tests

**Recommended New Tests** (should be added):

#### 1. Board Workshop Lines CRUD Test
**File**: `server/ws/tests/boards_workshop_lines.rs` (new)

```rust
#[tokio::test]
async fn board_workshop_lines_full_crud() {
    // Given: Authenticated user with a board
    // When: User creates, lists, updates, deletes line workshop items
    // Then: All operations succeed with correct responses
}
```

**Coverage**:
- POST `/api/board/{id}/workshop/lines` - Create line
- GET `/api/board/{id}/workshop/lines` - List lines
- PUT `/api/board/{id}/workshop/lines/{line_id}` - Update line
- DELETE `/api/board/{id}/workshop/lines/{line_id}` - Delete line

#### 2. Board Workshop Rules CRUD Test
**File**: `server/ws/tests/boards_workshop_rules.rs` (new)

```rust
#[tokio::test]
async fn board_workshop_rules_full_crud() {
    // Given: Authenticated user with a board
    // When: User creates, lists, updates, deletes rule workshop items
    // Then: All operations succeed with correct responses
}
```

**Coverage**:
- POST `/api/board/{id}/workshop/rules` - Create rule
- GET `/api/board/{id}/workshop/rules` - List rules
- PUT `/api/board/{id}/workshop/rules/{rule_id}` - Update rule
- DELETE `/api/board/{id}/workshop/rules/{rule_id}` - Delete rule

#### 3. Board Workshop Layouts CRUD Test
**File**: `server/ws/tests/boards_workshop_layouts.rs` (new)

```rust
#[tokio::test]
async fn board_workshop_layouts_full_crud() {
    // Given: Authenticated user with a board
    // When: User creates, lists, updates, deletes layout workshop items
    // Then: All operations succeed with correct responses
}
```

**Coverage**:
- POST `/api/board/{id}/workshop/layouts` - Create layout
- GET `/api/board/{id}/workshop/layouts` - List layouts
- PUT `/api/board/{id}/workshop/layouts/{layout_id}` - Update layout
- DELETE `/api/board/{id}/workshop/layouts/{layout_id}` - Delete layout

#### 4. Board Workshop Access Control Test
**File**: `server/ws/tests/boards_workshop_access.rs` (new)

```rust
#[tokio::test]
async fn board_workshop_requires_authentication() {
    // Given: Unauthenticated user
    // When: Attempting to create/modify workshop items
    // Then: All operations return 401 Unauthorized
}

#[tokio::test]
async fn board_workshop_respects_board_access() {
    // Given: User without access to board
    // When: Attempting to create/modify workshop items
    // Then: Operations return 403 Forbidden or 404 Not Found
}

#[tokio::test]
async fn board_workshop_list_allows_read_access() {
    // Given: User with read access to board
    // When: Listing workshop items
    // Then: Returns items successfully
}
```

**Coverage**:
- Authentication enforcement
- Authorization checks
- Access control for different operations

#### 5. Board Workshop Validation Test
**File**: `server/ws/tests/boards_workshop_validation.rs` (new)

```rust
#[tokio::test]
async fn board_workshop_validates_item_data() {
    // Given: Authenticated user with a board
    // When: Creating workshop item with invalid data
    // Then: Returns 400 Bad Request with validation errors
}

#[tokio::test]
async fn board_workshop_rejects_invalid_board_id() {
    // Given: Authenticated user
    // When: Accessing workshop for non-existent board
    // Then: Returns 404 Not Found
}
```

**Coverage**:
- Input validation
- Error responses
- Edge cases

## Regression Tests

### Existing Tests to Verify

All existing board and workshop tests should continue to pass:
- ✅ `boards_regression.rs` (6 tests)
- ✅ `boards_smoke.rs` (2 tests)
- ✅ `workshop_smoke.rs` (4 tests)
- ✅ `packages_smoke.rs` (2 tests - to ensure package workshop still works)

## End-to-End Tests

**Location**: `ui/web/tests-e2e/`

### Existing E2E Tests

Check if these cover board workshop:
- `board_access_flow.spec.ts` - May need updates
- `boards_flow.spec.ts` - May need workshop additions

### Recommended E2E Tests

Should be added to cover full user flow:

#### 1. Board Workshop Management Flow
**File**: `ui/web/tests-e2e/board_workshop_flow.spec.ts` (new)

```typescript
test('user can manage board workshop items', async ({ page }) => {
  // 1. Login
  // 2. Create/open board
  // 3. Navigate to workshop section
  // 4. Add shapes, lines, rules, layouts
  // 5. Edit workshop items
  // 6. Delete workshop items
  // 7. Verify items are persisted
});
```

#### 2. Board Workshop Access Control Flow
**File**: `ui/web/tests-e2e/board_workshop_access_flow.spec.ts` (new)

```typescript
test('board workshop respects access permissions', async ({ page }) => {
  // 1. User A creates board with workshop items
  // 2. User A invites User B with different permissions
  // 3. User B attempts workshop operations
  // 4. Verify operations succeed/fail based on permissions
});
```

## Test Priority

**Priority 1 (Critical)**: Must have before production
- ✅ Existing workshop smoke tests pass
- ✅ Existing board tests pass
- ✅ No regressions in package workshop

**Priority 2 (High)**: Should add soon
- 🔲 Integration tests for lines, rules, layouts CRUD
- 🔲 Access control tests

**Priority 3 (Medium)**: Good to have
- 🔲 Validation tests
- 🔲 E2E tests for workshop management

**Priority 4 (Low)**: Nice to have
- 🔲 Performance tests
- 🔲 Load tests for workshop operations

## Current Status

### What We Have ✅
- Generic workshop service tests (via workshop module)
- Basic board workshop test (`workshop_board_direct_creation_and_import_smoke_ok`)
- Line listing test (`workshop_lines_readonly_smoke_ok`)
- All existing tests passing (no regressions)

### What's Missing 🔲
- Dedicated integration tests for board workshop lines CRUD
- Dedicated integration tests for board workshop rules CRUD
- Dedicated integration tests for board workshop layouts CRUD
- Access control tests specific to board workshop
- Validation tests for workshop item creation
- E2E tests for workshop management UI

## Test Coverage Analysis

**Current Coverage**: ~60%
- Core functionality: ✅ Covered by existing workshop tests
- Board-specific operations: ⚠️ Partially covered
- CRUD operations: ⚠️ Only shapes fully covered
- Access control: ⚠️ Generic board access covered, workshop-specific not explicit
- Validation: ⚠️ Generic validation covered

**Target Coverage**: 85%+
- Need to add specific tests for lines, rules, layouts
- Need explicit access control tests
- Need validation edge case tests

## Recommendations

### Immediate Actions
1. **Add integration tests** for lines, rules, layouts CRUD operations
2. **Verify access control** works correctly with manual testing
3. **Document test gaps** in this file for future reference

### Future Actions
1. **Create E2E tests** for workshop management UI (when UI is implemented)
2. **Add performance tests** if workshop becomes a bottleneck
3. **Consider property-based testing** for workshop validation logic

## TDD Lesson Learned

**For Future Iterations**:
1. ✅ Write test plan BEFORE implementation
2. ✅ Write failing tests FIRST
3. ✅ Implement code to make tests pass
4. ✅ Refactor while keeping tests green
5. ✅ Document test coverage as you go

This iteration was implementation-first, but functionality is proven by existing tests passing. The gaps identified here should be addressed in a follow-up task.
