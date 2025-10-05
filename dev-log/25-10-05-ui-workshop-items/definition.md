# UI Workshop Items for Board - Definition

## Purpose and Goals
Implement UI for managing workshop items (shapes, lines, rules, layouts) in board detail view, similar to how it's already implemented for packages.

## Problem Statement
Currently, the Package detail page (`Package.tsx`) has a Workshop Items section that allows users to manage workshop items through the WorkshopManager component. However, the Board detail page (`Board.tsx`) does not have this functionality, even though the backend API endpoints for board workshop items already exist.

## Definition of Done
- [ ] Board detail page displays Workshop Items section (similar to Package detail page)
- [ ] Users with appropriate permissions (owner, manage, edit roles) can view the Workshop Items section
- [ ] Users can create, edit, and delete workshop items (shapes, lines, rules, layouts) for a board
- [ ] Workshop items are displayed in tabbed interface by type
- [ ] UI follows the same pattern and styling as Package workshop implementation
- [ ] Component properly integrates with existing WorkshopManager and WorkshopManagerProvider

## References
- **Existing Implementation**: `/ui/web/src/package/Package.tsx` (lines 276-284)
- **Workshop Components**: `/ui/web/src/common/workshop/` directory
- **Target File**: `/ui/web/src/boards/Board.tsx`
- **API Documentation**: `r&d/dev-docs/openapi.yaml` (board workshop endpoints exist at `/api/board/{id}/workshop/*`)

## Requirements
1. **Functionality**: Add expandable Workshop Items section to Board.tsx
2. **Permission**: Only accessible to users with owner, manage, or edit roles
3. **Components**: Use existing `WorkshopManagerProvider` and `WorkshopManager` components
4. **Entity Type**: Pass `entityType="board"` to workshop components
5. **Consistency**: Match the implementation pattern used in Package.tsx

## Known Constraints
- Backend API already implemented and tested (as per recent completed iteration)
- Workshop components are generic and support both "package" and "board" entity types
- Should follow existing code patterns and styling from Package implementation
