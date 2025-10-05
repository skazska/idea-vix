# Board Workshop Implementation - Definition

## Purpose and Goals

Implement full workshop functionality for boards, mirroring the implementation already done for packages. This includes support for all four workshop item types: shapes, lines, rules, and layouts.

## Problem Statement

Currently, boards only have partial workshop support (shapes only). According to `r&d/analysys/stories/core.md`, boards need complete workshop functionality to:
- Manage node shapes, links, rules and layouts
- Define and manage drawing primitives for diagrams
- Allow board owners and users with "manage" permission to add/edit/remove workshop items

## Definition of Done

1. Board workshop supports all four item types: shapes, lines, rules, layouts
2. API endpoints implemented for each item type:
   - GET `/{board_id}/workshop/{item_type}` - list items
   - POST `/{board_id}/workshop/{item_type}` - add item
   - PUT `/{board_id}/workshop/{item_type}/{item_id}` - update item
   - DELETE `/{board_id}/workshop/{item_type}/{item_id}` - remove item
3. Implementation matches the pattern used in packages (server/ws/src/package.rs)
4. Access control enforced (board owner or manage permission)
5. All existing tests pass
6. New tests added for the new functionality

## References

- **Core Feature**: `r&d/analysys/stories/core.md` - Workshop management for boards
- **Package Implementation**: `server/ws/src/package.rs` - lines 86-563 (reference implementation)
- **Current Board Implementation**: `server/ws/src/boards.rs` - lines 233-295 (shapes only)
- **Workshop Service**: `server/ws/src/workshop/generic_service.rs`
- **Workshop Store**: `server/ws/src/common/workshop_store.rs`

## Requirements

1. **Functional**:
   - Support lines, rules, and layouts in addition to existing shapes
   - Maintain same authorization model as shapes
   - Follow RESTful API conventions
   - Ensure data consistency

2. **Non-Functional**:
   - Code consistency with package implementation
   - Maintain type safety
   - Follow project coding standards
   - Proper error handling

## Challenges Anticipated

- Ensuring consistency between package and board workshop implementations
- Maintaining backwards compatibility with existing shape endpoints
- Proper integration testing
