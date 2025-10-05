# Board Workshop Implementation - Research

## Analysis of Package Workshop Implementation

From `server/ws/src/package.rs`, the package workshop implementation follows this pattern:

### 1. Service Initialization (lines 86-101)
- Creates four `WorkshopService` instances (shape, line, rule, layout)
- Each service wraps a `WorkshopStore` with the appropriate `WorkshopItemType`
- All services share the same `TransactionStarter`

```rust
let shape_service = Arc::new(WorkshopService::new(
    transaction_starter.clone(),
    Arc::new(WorkshopStore::new(WorkshopItemType::Shape))
));
// ... similar for line, rule, layout
```

### 2. RouteState Structure (lines 52-62)
- Includes all four workshop services as fields
- Services are `Arc<WorkshopService>` for shared ownership

### 3. Route Registration (lines 131-146)
- For each item type, four routes are registered:
  - GET `/{id}/workshop/{item_type}` - list
  - POST `/{id}/workshop/{item_type}` - add
  - PUT `/{id}/workshop/{item_type}/{item_id}` - update
  - DELETE `/{id}/workshop/{item_type}/{item_id}` - remove

### 4. Handler Pattern
Each item type has four handlers following this structure:

**List Handler** (e.g., `list_package_shapes`):
- Accepts optional authentication
- Calls `service.list_entity_items("package", package_id, &list_params, session)`
- Returns `Vec<WorkshopItem>`

**Add Handler** (e.g., `add_package_shape`):
- Requires authentication
- Validates `NewWorkshopItem` payload
- Calls `service.create_entity_item("package", package_id, &mut new_item, &session)`
- Returns 201 Created with `WorkshopItem`

**Update Handler** (e.g., `update_package_shape`):
- Requires authentication
- Validates `PatchWorkshopItem` payload
- Calls `service.update_item(item_id, &request, &session)`
- Returns updated `WorkshopItem`
- Note: TODO comment about validating entity ownership

**Delete Handler** (e.g., `remove_package_shape`):
- Requires authentication
- Calls `service.delete_item(item_id, &session)`
- Returns 204 No Content
- Note: TODO comment about validating entity ownership

## Analysis of Current Board Workshop Implementation

From `server/ws/src/boards.rs`:

### Current State (lines 37-70)
- Only has `shape_service` in RouteState
- Only four shape-related routes registered (lines 73-76)
- Only shape handlers implemented (lines 233-295)

### What's Missing
- `line_service`, `rule_service`, `layout_service` initialization
- RouteState fields for the three missing services
- 12 additional route registrations (4 routes × 3 item types)
- 12 additional handler functions

## Workshop Service and Store

From analysis of imports and usage:

- `WorkshopService` provides generic CRUD operations for workshop items
- `WorkshopStore` handles database operations, parameterized by `WorkshopItemType`
- `WorkshopItemType` enum has variants: Shape, Line, Rule, Layout
- Entity association is handled via entity_type ("board"/"package") and entity_id

## Implementation Strategy

The implementation should:

1. **Mirror package implementation** - Copy the pattern exactly for consistency
2. **Maintain entity type** - Use "board" as entity_type in all workshop service calls
3. **Follow naming convention** - `list_board_lines`, `add_board_lines`, etc.
4. **Keep TODOs** - Preserve TODO comments about validation (same as package)
5. **No breaking changes** - Existing shape endpoints remain unchanged

## Code Quality Considerations

- **Type Safety**: All handlers use strong typing via `ValidatedJson` and proper error mapping
- **Error Handling**: Consistent `map_err(|e| e.into())` pattern for error conversion
- **Authorization**: Follows session-based auth pattern
- **Modularity**: Each handler is independent and testable
- **Documentation**: Handler doc comments describe requirements and behavior

## Conclusion

The implementation is straightforward:
- Add three more `WorkshopService` instances for lines, rules, layouts
- Add them to RouteState
- Register 12 new routes
- Implement 12 handlers following the exact pattern from shapes

No architectural changes needed. This is a feature completion task using established patterns.
