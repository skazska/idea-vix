# Board Workshop Implementation - Plan

## Overview

Add workshop functionality for lines, rules, and layouts to boards by following the package implementation pattern.

## ⚠️ Process Note

**This plan was created correctly, but missed a critical step**: Test planning should have been done BEFORE implementation coding. See `test-plan.md` for the retrospective test plan that documents this gap.

**Correct Order for Future Iterations**:
1. Definition
2. Research
3. Plan (including test plan)
4. **Write tests** (TDD)
5. Implementation
6. Testing/Validation
7. Status

## Tasks

### 1. Update RouteState Structure

**File**: `server/ws/src/boards.rs` (lines ~37-41)

**What**: Add three new service fields to RouteState

**How**: Add fields for `line_service`, `rule_service`, `layout_service` matching the pattern of `shape_service`

**Why**: Services need to be accessible to handlers

### 2. Initialize Workshop Services

**File**: `server/ws/src/boards.rs` (lines ~43-62)

**What**: Initialize three new `WorkshopService` instances in `get_router` function

**How**: Create services for Line, Rule, and Layout workshop item types, similar to existing shape_service initialization (lines 61-62)

**Why**: Services need to be instantiated before use

### 3. Update RouteState Initialization

**File**: `server/ws/src/boards.rs` (lines ~64-69)

**What**: Add new services to RouteState construction

**How**: Add `line_service`, `rule_service`, `layout_service` fields to the Arc::new(RouteState { ... })

**Why**: Services must be stored in state to be accessible to handlers

### 4. Register Workshop Routes

**File**: `server/ws/src/boards.rs` (after line ~80)

**What**: Add 12 new route registrations (4 routes for each of 3 item types)

**How**: Add routes following the pattern:
- Lines: GET/POST `/workshop/lines`, PUT/DELETE `/workshop/lines/{line_id}`
- Rules: GET/POST `/workshop/rules`, PUT/DELETE `/workshop/rules/{rule_id}`  
- Layouts: GET/POST `/workshop/layouts`, PUT/DELETE `/workshop/layouts/{layout_id}`

**Why**: Routes expose the API endpoints for workshop functionality

### 5. Implement Line Handlers

**File**: `server/ws/src/boards.rs` (after shape handlers)

**What**: Add four handler functions for lines
- `list_board_lines`
- `add_board_line`
- `update_board_line`
- `remove_board_line`

**How**: Copy and adapt the shape handlers, changing:
- Handler names (shape → line)
- Service reference (state.shape_service → state.line_service)
- Path parameter names (shape_id → line_id)

**Why**: Handlers implement the business logic for line operations

### 6. Implement Rule Handlers

**File**: `server/ws/src/boards.rs` (after line handlers)

**What**: Add four handler functions for rules
- `list_board_rules`
- `add_board_rule`
- `update_board_rule`
- `remove_board_rule`

**How**: Same pattern as line handlers, adapted for rules

**Why**: Handlers implement the business logic for rule operations

### 7. Implement Layout Handlers

**File**: `server/ws/src/boards.rs` (after rule handlers)

**What**: Add four handler functions for layouts
- `list_board_layouts`
- `add_board_layout`
- `update_board_layout`
- `remove_board_layout`

**How**: Same pattern as line handlers, adapted for layouts

**Why**: Handlers implement the business logic for layout operations

### 8. Build and Test

**What**: Verify the implementation compiles and passes tests

**How**: 
- Run `cargo build` to check compilation
- Run existing tests to ensure no regressions
- Consider adding new integration tests

**Why**: Ensure quality and catch any issues early

## Dependencies

- Tasks 1-3 must be completed before task 4 (routes need services in state)
- Task 4 must be completed before tasks 5-7 (handlers need routes)
- Tasks 5-7 can be done in any order (independent item types)
- Task 8 should be done after all code changes

## Validation Approach

1. Code compiles without errors
2. All existing tests pass
3. New endpoints are accessible
4. Authorization is enforced
5. Data is persisted correctly

## Estimated Changes

- Lines added: ~200 (4 handlers × 3 types × ~17 lines per handler)
- Lines modified: ~20 (RouteState, initialization, route registration)
- Files changed: 1 (boards.rs)
