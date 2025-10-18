# Definition: Allow Owner to Add Package to Board

## Purpose and Goals

Implement functionality that allows board owners to add packages to a board. This enables boards to reference and use workshop items (shapes, lines, rules, layouts) from packages.

**User Story Reference:** `r&d/analysys/stories/core.md#add-package-to-board`

## Definition of Done

1. ✅ There are UI elements in board view page, allowing owner to start searching for package to add it
2. ✅ There are integration and e2e tests covering this functionality passing
3. ✅ Documentation actualized (OpenAPI spec, traceability matrix)
4. ✅ Coverage matrix actualized

## Feature Description

Board owners should be able to:
- View available packages (those they own or that are public)
- Search/filter packages
- Add a package to their board (creates a board-package association)
- View packages already added to the board
- Remove packages from the board (future story)

This feature creates the foundation for importing workshop items from packages into boards.

## References

### Focus Documents
- `r&d/analysys/stories/core.md` - User stories (specifically "add package to board")
- `r&d/dev-docs/storage.dbml` - Database schema (board_package junction table already exists)
- `r&d/dev-docs/openapi.yaml` - API specification (needs updates)
- `r&d/analysys/requirements/traceability-matrix.md` - Test coverage tracking

### Related Code
- Backend: `server/ws/src/boards/` - Board feature module
- Backend: `server/ws/src/package/` - Package feature module
- Frontend: `ui/web/src/boards/` - Board UI components
- Frontend: `ui/web/src/package/` - Package UI components
- Database: `board_package` junction table (already exists in schema)

### Previous Iterations
- `dev-log/25-10-05-board-workshop/` - Workshop functionality for boards
- `dev-log/25-10-05-ui-workshop-items/` - UI for managing workshop items

## Requirements and Specifications

### Backend API Requirements
- `POST /api/board/{id}/packages` - Add a package to a board
  - Requires authentication
  - Requires owner role on the board
  - Request body: `{ "package_id": number }`
  - Returns: updated list of board packages or the added package
  - Error cases: 404 (board/package not found), 403 (not owner), 409 (already added), 401 (not authenticated)

- `GET /api/board/{id}/packages` - List packages associated with a board
  - Requires authentication or board must be public
  - Returns: array of packages

- `DELETE /api/board/{id}/packages/{package_id}` - Remove a package from a board (optional for this iteration)
  - Requires authentication
  - Requires owner role on the board
  - Returns: updated list of board packages

### Frontend UI Requirements
- Add "Manage Packages" or similar UI element to board view
- Show a list of currently added packages
- Provide search/filter functionality for available packages
- "Add Package" button that opens package selection dialog
- Visual feedback when adding packages
- Only show to board owners (based on access role)

### Database
- Junction table `board_package` already exists with columns:
  - `board_id` (references board.id)
  - `package_id` (references package.id)
  - Primary key: (board_id, package_id)

### Authorization Rules
- Only board owners can add/remove packages from boards
- Package visibility: board owner can add:
  - Public packages (any user)
  - Packages they own
  - Packages they have access to (view/edit/manage role)

## Open Questions

1. Should we return just the added package or the full list of board packages?
   - **Decision:** Return full list for consistency with access endpoints
   
2. Should adding a package automatically import any workshop items?
   - **Decision:** No, this is handled in a separate story ("import items from package")

3. Should we prevent removing a package if its items are in use?
   - **Decision:** Per the story requirements, removal doesn't delete items. Items stay on board even if source package is removed. Handle in future story.

4. Do we need pagination for package lists?
   - **Decision:** Start without pagination, add if needed

## Anticipated Challenges

1. **Authorization complexity**: Need to check both board ownership AND package visibility
2. **Duplicate prevention**: Ensure same package can't be added twice (handled by DB unique constraint)
3. **Consistency with existing patterns**: Following the established patterns for board access, workshop items, etc.
4. **UI/UX**: Creating an intuitive interface for package selection without cluttering the board view
