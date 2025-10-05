# UI Workshop Items for Board - Implementation Plan

## Overview

Add Workshop Items management section to Board detail page by integrating existing workshop components.

## Tasks

### 1. Add Workshop Component Imports to Board.tsx

**File**: `/ui/web/src/boards/Board.tsx`

**Changes**: Add imports at the top of the file (after existing imports):

```typescript
import { WorkshopManagerProvider, WorkshopManager } from "../common/workshop";
```

**Why**: Need to import the workshop management components that are already used in Package.tsx

---

### 2. Add Workshop Items Section to Board Detail View

**File**: `/ui/web/src/boards/Board.tsx`

**Location**: Inside the "Board Sections" div (after the Access management section, around line 272)

**Changes**: Add new Expandable section for Workshop Items:

```tsx
{/* Workshop Items Section */}
<Accessible roles={["owner", "manage", "edit"]}>
    <Expandable title="Workshop Items" openByDefault={false} name="board-section-workshop">
        <div class="bg-white rounded-lg shadow p-6">
            <WorkshopManagerProvider entityType="board" entityId={boardId!}>
                <WorkshopManager entityType="board" entityId={boardId!} />
            </WorkshopManagerProvider>
        </div>
    </Expandable>
</Accessible>
```

**Why**: 
- Matches the exact pattern used in Package.tsx
- Uses `entityType="board"` to target board workshop endpoints
- Wrapped in `Accessible` component to restrict access to users with appropriate permissions
- Uses `Expandable` component for consistent UI with other sections

---

### 3. Verify Implementation

**Actions**:
- Build the frontend application
- Verify no TypeScript errors

---

### 4. Create E2E Tests

**File**: `/ui/web/tests-e2e/board_workshop_flow.spec.ts`

**Content**: Mirror the package workshop E2E tests with board-specific adaptations

**Test Cases to Implement**:
1. Open workshop section and display tabs
2. Create shape items
3. Create items of all types (shapes, lines, rules, layouts)
4. Edit existing workshop items
5. Delete workshop items
6. Form validation
7. Cancel form without saving
8. Switch between item type tabs
9. Persist workshop items after page reload

**Why**: 
- Ensures board workshop functionality works correctly
- Provides automated regression testing
- Maintains parity with package workshop test coverage

---

## Expected Outcome

After implementation:
1. Board detail page will have a Workshop Items section identical to Package detail page
2. Users can manage shapes, lines, rules, and layouts for boards
3. The UI maintains consistency with the existing package implementation
4. No breaking changes to existing functionality
5. Comprehensive E2E test coverage ensures quality and prevents regressions

## Dependencies

- No new dependencies required
- All workshop components already exist and support board entity type
- Backend API already implemented and tested
