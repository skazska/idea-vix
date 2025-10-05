# UI Workshop Items for Board - Testing Plan

## Testing Strategy

Since this is a UI feature addition that reuses existing tested components, testing will focus on:
1. Build verification (no compile errors)
2. Visual verification (if possible)
3. Integration with existing functionality

## Test Cases

### TC1: Component Integration
**Objective**: Verify that workshop components integrate correctly with Board.tsx

**Steps**:
1. Build the frontend application
2. Check for TypeScript compilation errors
3. Check for any console errors

**Expected Result**: Clean build with no errors

---

### TC2: UI Rendering (Manual Test - if possible)
**Objective**: Verify Workshop Items section appears in board detail view

**Preconditions**: 
- User logged in
- User has access to at least one board with owner/manage/edit role

**Steps**:
1. Navigate to a board detail page
2. Scroll to the sections area
3. Look for "Workshop Items" expandable section

**Expected Result**: 
- Workshop Items section visible after Access section
- Section is collapsed by default

---

### TC3: Permission-based Access (Manual Test - if possible)
**Objective**: Verify Workshop Items section respects role permissions

**Test Cases**:
- User with owner role: Should see Workshop Items section
- User with manage role: Should see Workshop Items section  
- User with edit role: Should see Workshop Items section
- User with view role: Should NOT see Workshop Items section
- Unauthenticated user viewing public board: Should NOT see Workshop Items section

**Expected Result**: Section only visible to users with owner, manage, or edit roles

---

### TC4: Workshop Manager Functionality (Manual Test - if possible)
**Objective**: Verify workshop manager functions correctly for boards

**Steps**:
1. Expand Workshop Items section
2. Verify tabs for Shapes, Lines, Rules, Layouts are visible
3. Try to add a new shape
4. Try to edit an existing item
5. Try to delete an item

**Expected Result**: All CRUD operations work correctly for board workshop items

---

### TC5: No Regression
**Objective**: Verify existing board functionality still works

**Steps**:
1. Verify board detail page loads correctly
2. Verify Access management section still works
3. Verify edit/delete board functionality still works

**Expected Result**: No existing functionality is broken

## Automated Tests

No new automated tests required at this stage because:
- Workshop components already have their own tests
- This is purely integrating existing components
- E2E tests for workshop functionality already exist from package implementation

## Definition of Done Verification

After testing, verify all DoD items:
- [ ] Board detail page displays Workshop Items section
- [ ] Only users with appropriate permissions can view the section
- [ ] Users can create, edit, and delete workshop items
- [ ] Workshop items displayed in tabbed interface by type
- [ ] UI follows same pattern as Package workshop implementation
- [ ] Component properly integrates with existing WorkshopManager
