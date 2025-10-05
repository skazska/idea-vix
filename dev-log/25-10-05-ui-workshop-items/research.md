# UI Workshop Items for Board - Research

## Analysis of Package Implementation

### Package.tsx Workshop Section (lines 276-284)
The package detail page includes a Workshop Items section within an Expandable component:

```tsx
<Accessible roles={["owner", "manage", "edit"]}>
    <Expandable title="Workshop Items" openByDefault={false} name="package-section-workshop">
        <div class="bg-white rounded-lg shadow p-6">
            <WorkshopManagerProvider entityType="package" entityId={packageId!}>
                <WorkshopManager entityType="package" entityId={packageId!} />
            </WorkshopManagerProvider>
        </div>
    </Expandable>
</Accessible>
```

### Board.tsx Current State (lines 266-272)
The board detail page currently has only the Access management section:

```tsx
<div class="space-y-6">
    {/* Access management */}
    <Accessible roles={["owner", "manage"]}>
        <Expandable title={...} openByDefault={false} name="board-section-access">
            <AccessMapProvider id={b().id} api={boardApi}>
                <AccessManager id={b().id}/>
            </AccessMapProvider>
        </Expandable>
    </Accessible>
</div>
```

### Required Imports
Package.tsx already imports:
- `WorkshopManagerProvider` and `WorkshopManager` from `"../common/workshop"`

Board.tsx needs to add these imports.

## Workshop Component Analysis

### WorkshopManager Component
Located at `/ui/web/src/common/workshop/components/WorkshopManager.tsx`:
- Accepts props: `entityType: "package" | "board"` and `entityId: string`
- Displays tabbed interface for shapes, lines, rules, layouts
- Handles create, edit, delete operations
- Generic implementation works for both packages and boards

### WorkshopManagerProvider
Located at `/ui/web/src/common/workshop/providers.tsx`:
- Sets up context for workshop items management
- Accepts same props as WorkshopManager
- Fetches workshop items from API based on entity type and ID
- Provides CRUD operations through context

## Backend API Verification

### Board Workshop API Endpoints (from openapi.yaml)
All necessary endpoints exist:
- `GET /api/board/{id}/workshop/shapes` - List shapes
- `POST /api/board/{id}/workshop/shapes` - Create shape
- `PUT /api/board/{id}/workshop/shapes/{shape_id}` - Update shape  
- `DELETE /api/board/{id}/workshop/shapes/{shape_id}` - Delete shape
- Similar endpoints for `lines`, `rules`, and `layouts`

### API Integration
The `createEntityWorkshopApi` function in `/ui/web/src/common/workshop/api.ts` already handles both entity types:
```typescript
export function createEntityWorkshopApi(
    backend: IBackend,
    entityType: WorkshopEntityType,
    entityId: string
): IWorkshopApi
```

## Insights and Conclusions

### 1. Implementation is Straightforward
Since the workshop components are already generic and support board entity type, the implementation is a simple copy-paste with "package" → "board" replacements.

### 2. No New Components Needed
All required components already exist and are designed to work with both packages and boards.

### 3. Minimal Code Changes
Only one file needs to be modified: `Board.tsx`
Changes required:
1. Add imports for WorkshopManagerProvider and WorkshopManager
2. Add Workshop Items section within the Board Sections div
3. Use "board" as entityType instead of "package"

### 4. Consistency with Package Implementation
The implementation should match the Package.tsx pattern exactly to maintain consistency across the application.

## No Weak Points Identified
- Backend API is complete and tested
- Workshop components are proven to work (used in Package)
- No additional configuration or setup needed
