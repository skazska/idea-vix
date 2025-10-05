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
- Test in browser (if possible) to ensure:
  - Workshop Items section appears in board detail view
  - Section is only visible to users with owner/manage/edit roles
  - Can create, edit, and delete workshop items
  - Tab navigation works correctly between item types

---

## Expected Outcome

After implementation:
1. Board detail page will have a Workshop Items section identical to Package detail page
2. Users can manage shapes, lines, rules, and layouts for boards
3. The UI maintains consistency with the existing package implementation
4. No breaking changes to existing functionality

## Dependencies

- No new dependencies required
- All workshop components already exist and support board entity type
- Backend API already implemented and tested
