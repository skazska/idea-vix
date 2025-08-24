# Workshop Shape Management Implementation Plan

## Overview

This document outlines the implementation plan for adding shape management functionality to package workshops, enabling users to create, edit, and remove shapes as part of the workshop concept.

## Current State Analysis

### Database Schema

- ✅ `shape` table exists with basic structure (id, name)
- ✅ `package_shape` junction table exists (package_id, shape_id, name)
- ❌ Shape definition/model storage not implemented
- ❌ No semantic identifier for text-based references
- ❌ No JSON field for complex shape definitions

### Backend Status

- ✅ Package CRUD operations working
- ✅ Package access control working
- ❌ No workshop module structure
- ❌ No shape-specific API endpoints
- ❌ No shape service layer
- ❌ No shape storage layer

### Frontend Status

- ✅ Package view page exists with placeholder sections
- ✅ Basic workshop sections UI implemented (Node Shapes, Connection Lines, etc.)
- ❌ Shape creation/editing forms not implemented
- ❌ Shape listing/management not implemented
- ❌ No shape models/types defined

## Requirements Analysis

Based on the user stories and workshop specifications:

### Core Features Needed

1. **Shape CRUD Operations**: Create, Read, Update, Delete shapes within package workshop
2. **Semantic Identifiers**: Shapes need text-based identifiers for node definition references
3. **Shape Definition Model**: Complex JSON structure for shape properties
4. **Package-Shape Association**: Link shapes to packages (no overrides needed initially)
5. **Access Control**: Owner/manage permissions for shape operations
6. **Shape Rendering**: Visual representation of shape definitions
7. **Versioning Compatibility**: Design with future package versioning in mind

### Board-Package-Shape Relationship Clarification

Based on analysis, the relationship should be:
- **Package Workshop**: Contains shape definitions (owned by package)
- **Board Workshop**: References shapes from packages (read-only links)
- **Board Nodes**: Use shape references but may need local instances/clones for positioning

**No override_definition needed** because:
- Shapes belong to packages and shouldn't be modified in board context
- If board needs custom variations, it should clone the shape as a board-specific shape
- Package versioning will handle shape evolution over time

### Shape Model Requirements (from workshop.md)

```typescript
interface ShapeDefinition {
  background: {
    border: {
      path: string;
      stroke: StrokeProperties;
      fill: FillProperties;
    };
  };
  label: {
    position: Position;
    defaultText: string;
    styling: TextStyling;
  };
  content: {
    types: ContentType[];
  };
  connectionSockets: ConnectionSocket[];
}
```

### Phase 1: Database Schema Enhancement

#### 1.1 Create Migration for Shape Definitions

**File**: `server/ws/migrations/[timestamp]_enhance_shape_table.up.sql`

```sql
-- Add semantic identifier and JSON definition column
ALTER TABLE shape ADD COLUMN slug VARCHAR(100) UNIQUE; -- semantic identifier for text references
ALTER TABLE shape ADD COLUMN definition TEXT; -- JSON as TEXT in SQLite

-- Add metadata columns
ALTER TABLE shape ADD COLUMN description VARCHAR(500);
ALTER TABLE shape ADD COLUMN created_at DATETIME DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE shape ADD COLUMN updated_at DATETIME DEFAULT CURRENT_TIMESTAMP;

-- Create index for slug lookups
CREATE INDEX idx_shape_slug ON shape(slug);

-- Remove override_definition from package_shape (not needed)
-- ALTER TABLE package_shape DROP COLUMN override_definition; -- if it exists
ALTER TABLE package_shape ADD COLUMN created_at DATETIME DEFAULT CURRENT_TIMESTAMP;
```

#### 1.2 Update storage.dbml

Update the database documentation to reflect the new schema with semantic identifiers.

### Phase 2: Backend Implementation

#### 2.1 Workshop Shape Storage Layer

**File**: `server/ws/src/workshop/shape_store.rs`

```rust
pub struct ShapeDb {
    pub id: i32,
    pub name: String,
    pub slug: String, // semantic identifier for text references
    pub description: Option<String>,
    pub definition: Option<String>, // JSON
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct NewShapeDb<'a> {
    pub name: &'a str,
    pub slug: &'a str, // required semantic identifier
    pub description: Option<&'a str>,
    pub definition: Option<&'a str>,
}

pub struct PatchShapeDb<'a> {
    pub name: Option<&'a str>,
    pub slug: Option<&'a str>, // allow slug updates with uniqueness validation
    pub description: Option<Option<&'a str>>,
    pub definition: Option<Option<&'a str>>,
}

// Simplified package-shape association (no overrides)
pub struct PackageShapeDb {
    pub package_id: i32,
    pub shape_id: i32,
    pub name: Option<String>, // optional display name in package context
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl ShapeStore {
    // Global shape operations (for workshop management)
    pub async fn add_shape(&self, shape: NewShapeDb<'_>) -> Result<ShapeDb, Error>;
    pub async fn get_shape(&self, id: i32) -> Result<Option<ShapeDb>, Error>;
    pub async fn get_shape_by_slug(&self, slug: &str) -> Result<Option<ShapeDb>, Error>;
    pub async fn update_shape(&self, id: i32, patch: PatchShapeDb<'_>) -> Result<ShapeDb, Error>;
    pub async fn delete_shape(&self, id: i32) -> Result<(), Error>;
    pub async fn list_shapes(&self) -> Result<Vec<ShapeDb>, Error>;
    
    // Package-shape association operations (simplified)
    pub async fn add_shape_to_package(&self, package_id: i32, shape_id: i32, name: Option<&str>) -> Result<PackageShapeDb, Error>;
    pub async fn remove_shape_from_package(&self, package_id: i32, shape_id: i32) -> Result<(), Error>;
    pub async fn list_package_shapes(&self, package_id: i32) -> Result<Vec<(ShapeDb, PackageShapeDb)>, Error>;
    pub async fn update_package_shape_name(&self, package_id: i32, shape_id: i32, name: Option<&str>) -> Result<PackageShapeDb, Error>;
    
    // Version-aware queries (for future versioning support)
    pub async fn list_shapes_for_package_version(&self, package_id: i32, version: Option<&str>) -> Result<Vec<(ShapeDb, PackageShapeDb)>, Error>;
}
```

#### 2.2 Workshop Shape Service Layer

**File**: `server/ws/src/workshop/shape_service.rs`

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Shape {
    pub id: i32,
    pub name: String,
    pub slug: String, // semantic identifier for text-based references
    pub description: Option<String>,
    pub definition: Option<serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageShape {
    pub package_id: i32,
    pub shape: Shape,
    pub name: Option<String>, // optional display override
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct NewShapeItem {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    #[validate(length(min = 3, max = 100), regex = "^[a-z][a-z0-9-]*$")] // slug format
    pub slug: String,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    pub definition: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct PatchShapeItem {
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,
    #[validate(length(min = 3, max = 100), regex = "^[a-z][a-z0-9-]*$")]
    pub slug: Option<String>,
    #[validate(length(max = 500))]
    pub description: Option<Option<String>>,
    pub definition: Option<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct AddShapeToPackageItem {
    pub shape_id: i32,
    #[validate(length(max = 100))]
    pub name: Option<String>, // optional display name override
}

impl ShapeService {
    // Global shape management (within workshop context)
    pub async fn create_shape(&self, item: &NewShapeItem, session: &SessionData) -> Result<Shape, ModelError>;
    pub async fn get_shape(&self, id: i32) -> Result<Shape, ModelError>;
    pub async fn get_shape_by_slug(&self, slug: &str) -> Result<Shape, ModelError>;
    pub async fn update_shape(&self, id: i32, patch: &PatchShapeItem, session: &SessionData) -> Result<Shape, ModelError>;
    pub async fn delete_shape(&self, id: i32, session: &SessionData) -> Result<(), ModelError>;
    pub async fn list_shapes(&self) -> Result<Vec<Shape>, ModelError>;
    
    // Package-specific shape management
    pub async fn add_shape_to_package(&self, package_id: i32, item: &AddShapeToPackageItem, session: &SessionData) -> Result<PackageShape, ModelError>;
    pub async fn remove_shape_from_package(&self, package_id: i32, shape_id: i32, session: &SessionData) -> Result<(), ModelError>;
    pub async fn list_package_shapes(&self, package_id: i32, session: &Option<SessionData>) -> Result<Vec<PackageShape>, ModelError>;
    pub async fn update_package_shape(&self, package_id: i32, shape_id: i32, name: Option<String>, session: &SessionData) -> Result<PackageShape, ModelError>;
}
```

#### 2.3 Workshop Shape API Routes

**File**: `server/ws/src/workshop/shape_routes.rs`

```rust
pub fn router() -> Router<Arc<crate::RouteState>> {
    Router::new()
        // Global shape routes (workshop management)
        .route("/workshop/shapes", get(list_shapes).post(create_shape))
        .route("/workshop/shapes/:id", get(get_shape).patch(update_shape).delete(delete_shape))
        .route("/workshop/shapes/by-slug/:slug", get(get_shape_by_slug))
        
        // Package-shape association routes
        .route("/packages/:package_id/shapes", get(list_package_shapes).post(add_shape_to_package))
        .route("/packages/:package_id/shapes/:shape_id", patch(update_package_shape).delete(remove_shape_from_package))
}

// Handlers for package-shape operations
async fn list_package_shapes(auth: OptionalAuthToken, Path(package_id): Path<i32>) -> Result<Json<Vec<PackageShape>>, (StatusCode, String)>;
async fn add_shape_to_package(auth: AuthToken, Path(package_id): Path<i32>, ValidatedJson(item): ValidatedJson<AddShapeToPackageItem>) -> Result<(StatusCode, Json<PackageShape>), (StatusCode, String)>;
async fn update_package_shape(auth: AuthToken, Path((package_id, shape_id)): Path<(i32, i32)>, ValidatedJson(item): ValidatedJson<UpdatePackageShapeItem>) -> Result<Json<PackageShape>, (StatusCode, String)>;
async fn remove_shape_from_package(auth: AuthToken, Path((package_id, shape_id)): Path<(i32, i32)>) -> Result<StatusCode, (StatusCode, String)>;
```

#### 2.4 Workshop Module Structure

**File**: `server/ws/src/workshop.rs` (root module file, not mod.rs)

```rust
//! Workshop module: manages all workshop elements (shapes, lines, rules, layouts)
//!
//! The workshop is a conceptual collection of diagram element blueprints that can be
//! packaged and reused across different diagrams and boards.

pub mod shape_store;
pub mod shape_service;
pub mod shape_routes;
// Future: line_store, line_service, rule_store, etc.

use axum::Router;
use std::sync::Arc;

pub fn router() -> Router<Arc<crate::RouteState>> {
    Router::new()
        .nest("/", shape_routes::router())
        // Future: .nest("/", line_routes::router())
}
```

### Phase 3: Frontend Implementation

#### 3.1 Workshop Shape Models and Types

**File**: `ui/web/src/workshop/shape/model.ts`

```typescript
export interface ShapeDefinition {
  background?: {
    border?: {
      path: string;
      stroke?: StrokeProperties;
      fill?: FillProperties;
    };
  };
  label?: {
    position?: Position;
    defaultText?: string;
    styling?: TextStyling;
  };
  content?: {
    types: ContentType[];
  };
  connectionSockets?: ConnectionSocket[];
}

export interface Shape {
  id: number;
  name: string;
  slug: string; // semantic identifier for text-based references
  description?: string;
  definition?: ShapeDefinition;
  created_at: string;
  updated_at: string;
}

export interface PackageShape {
  package_id: number;
  shape: Shape;
  name?: string; // optional display name override
  created_at: string;
}

export interface NewShapeItem {
  name: string;
  slug: string; // required semantic identifier (kebab-case)
  description?: string;
  definition?: ShapeDefinition;
}

export interface AddShapeToPackageItem {
  shape_id: number;
  name?: string; // optional display name override (no definition override)
}
```

#### 3.2 Workshop Shape API Provider

**File**: `ui/web/src/workshop/shape/providers/api.ts`

```typescript
export interface WorkshopShapeApi {
  // Global workshop shape operations
  list: () => Promise<Shape[]>;
  get: (id: number) => Promise<Shape>;
  getBySlug: (slug: string) => Promise<Shape>;
  create: (item: NewShapeItem) => Promise<Shape>;
  update: (id: number, patch: Partial<NewShapeItem>) => Promise<Shape>;
  remove: (id: number) => Promise<void>;
  
  // Package-shape operations (simplified, no overrides)
  listPackageShapes: (packageId: number) => Promise<PackageShape[]>;
  addShapeToPackage: (packageId: number, item: AddShapeToPackageItem) => Promise<PackageShape>;
  updatePackageShapeName: (packageId: number, shapeId: number, name?: string) => Promise<PackageShape>;
  removeShapeFromPackage: (packageId: number, shapeId: number) => Promise<void>;
}
```

#### 3.3 Package Shapes Provider

**File**: `ui/web/src/package/providers/shapes.tsx`

```typescript
export const PackageShapesProvider: ParentComponent<{ packageId: number }> = (props) => {
  const shapeApi = getWorkshopShapeApi(useBackend());
  const shapes = createAsync(() => shapeApi.listPackageShapes(props.packageId));
  
  const addShape = async (item: AddShapeToPackageItem) => {
    const result = await shapeApi.addShapeToPackage(props.packageId, item);
    revalidate(shapes.keyFor);
    return result;
  };
  
  const removeShape = async (shapeId: number) => {
    await shapeApi.removeShapeFromPackage(props.packageId, shapeId);
    revalidate(shapes.keyFor);
  };
  
  const updateShapeName = async (shapeId: number, name?: string) => {
    const result = await shapeApi.updatePackageShapeName(props.packageId, shapeId, name);
    revalidate(shapes.keyFor);
    return result;
  };
  
  return (
    <PackageShapesContext.Provider value={[shapes, { addShape, removeShape, updateShapeName }]}>
      {props.children}
    </PackageShapesContext.Provider>
  );
};
```

#### 3.4 Shape Creation/Edit Forms

**File**: `ui/web/src/workshop/shape/forms/ShapeForm.tsx`

```typescript
export const ShapeForm: Component<{
  initialShape?: Partial<NewShapeItem>;
  onSave: (shape: NewShapeItem) => Promise<void>;
  onCancel: () => void;
}> = (props) => {
  const [name, setName] = createSignal(props.initialShape?.name || "");
  const [slug, setSlug] = createSignal(props.initialShape?.slug || "");
  const [description, setDescription] = createSignal(props.initialShape?.description || "");
  const [definition, setDefinition] = createSignal<ShapeDefinition>(props.initialShape?.definition || {});
  
  // Auto-generate slug from name
  createEffect(() => {
    if (!props.initialShape?.slug) {
      const generatedSlug = name()
        .toLowerCase()
        .replace(/[^a-z0-9]+/g, '-')
        .replace(/^-+|-+$/g, '');
      setSlug(generatedSlug);
    }
  });
  
  const handleSubmit = async (e: Event) => {
    e.preventDefault();
    await props.onSave({
      name: name(),
      slug: slug(),
      description: description() || undefined,
      definition: definition(),
    });
  };
  
  return (
    <form onSubmit={handleSubmit}>
      {/* Basic fields */}
      <div class="mb-4">
        <label class="block text-sm font-medium mb-1">Shape Name</label>
        <input 
          type="text"
          value={name()} 
          onInput={(e) => setName(e.target.value)}
          placeholder="Rectangle, Circle, Process..."
          class="w-full border rounded px-3 py-2"
          required
        />
      </div>
      
      <div class="mb-4">
        <label class="block text-sm font-medium mb-1">
          Semantic Identifier (for text references)
        </label>
        <input 
          type="text"
          value={slug()} 
          onInput={(e) => setSlug(e.target.value)}
          placeholder="rectangle, circle, process-box..."
          pattern="^[a-z][a-z0-9-]*$"
          class="w-full border rounded px-3 py-2 font-mono text-sm"
          required
        />
        <p class="text-xs text-gray-500 mt-1">
          Used in node definitions: {{shape: "{slug()}"}}
        </p>
      </div>
      
      <div class="mb-4">
        <label class="block text-sm font-medium mb-1">Description</label>
        <textarea 
          value={description()} 
          onInput={(e) => setDescription(e.target.value)}
          placeholder="Brief description of the shape..."
          class="w-full border rounded px-3 py-2"
          rows="3"
        />
      </div>
      
      {/* Shape definition editor */}
      <div class="mb-6">
        <label class="block text-sm font-medium mb-2">Shape Definition</label>
        <ShapeDefinitionEditor 
          definition={definition()} 
          onChange={setDefinition}
        />
      </div>
      
      {/* Preview */}
      <div class="mb-6">
        <label class="block text-sm font-medium mb-2">Preview</label>
        <ShapePreview definition={definition()} />
      </div>
      
      <div class="flex gap-2">
        <button 
          type="submit"
          class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
        >
          Save Shape
        </button>
        <button 
          type="button" 
          onClick={props.onCancel}
          class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded"
        >
          Cancel
        </button>
      </div>
    </form>
  );
};
```

#### 3.5 Update Package View

**File**: `ui/web/src/package/Package.tsx`

Replace the placeholder shapes section:

```typescript
// Replace the Node Shapes section with:
<Accessible roles={["owner", "manage", "edit"]}>
  <Expandable title="Node Shapes" openByDefault={false} name="package-section-shapes">
    <PackageShapesProvider packageId={Number(packageId)}>
      <PackageShapesManager />
    </PackageShapesProvider>
  </Expandable>
</Accessible>
```

#### 3.6 Package Shapes Manager Component

**File**: `ui/web/src/package/components/PackageShapesManager.tsx`

```typescript
export const PackageShapesManager: Component = () => {
  const [shapes, actions] = usePackageShapes();
  const [showAddForm, setShowAddForm] = createSignal(false);
  const [showCreateForm, setShowCreateForm] = createSignal(false);
  const [editingShape, setEditingShape] = createSignal<PackageShape | null>(null);
  
  return (
    <div class="bg-white rounded-lg shadow p-6">
      <div class="flex justify-between items-center mb-4">
        <h2 class="text-xl font-bold">Node Shapes</h2>
        <div class="flex gap-2">
          <button 
            onClick={() => setShowAddForm(true)}
            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded text-sm flex items-center gap-1"
          >
            <Plus size={'0.8rem'}/>
            Add Existing
          </button>
          <button 
            onClick={() => setShowCreateForm(true)}
            class="bg-green-500 hover:bg-green-700 text-white font-bold py-1 px-2 rounded text-sm flex items-center gap-1"
          >
            <Plus size={'0.8rem'}/>
            Create New
          </button>
        </div>
      </div>
      
      <Show 
        when={shapes()?.length}
        fallback={<div class="text-gray-500">No node shapes defined yet.</div>}
      >
        <div class="grid gap-4">
          <For each={shapes()}>
            {(shape) => (
              <PackageShapeCard 
                shape={shape} 
                onEdit={() => setEditingShape(shape)}
                onDelete={() => actions.removeShape(shape.shape.id)}
                onUpdateName={(name) => actions.updateShapeName(shape.shape.id, name)}
              />
            )}
          </For>
        </div>
      </Show>
      
      {/* Add Existing Shape Modal */}
      <Show when={showAddForm()}>
        <ModalCentered onClose={() => setShowAddForm(false)}>
          <AddExistingShapeForm 
            onSave={(item) => {
              actions.addShape(item);
              setShowAddForm(false);
            }}
            onCancel={() => setShowAddForm(false)}
          />
        </ModalCentered>
      </Show>
      
      {/* Create New Shape Modal */}
      <Show when={showCreateForm()}>
        <ModalCentered onClose={() => setShowCreateForm(false)}>
          <ShapeForm 
            onSave={async (shape) => {
              // Create shape globally first, then add to package
              const newShape = await workshopShapeApi.create(shape);
              await actions.addShape({ shape_id: newShape.id });
              setShowCreateForm(false);
            }}
            onCancel={() => setShowCreateForm(false)}
          />
        </ModalCentered>
      </Show>
      
      {/* Edit Shape Modal */}
      <Show when={editingShape()}>
        <ModalCentered onClose={() => setEditingShape(null)}>
          <ShapeForm 
            initialShape={editingShape()?.shape}
            onSave={async (updatedShape) => {
              await workshopShapeApi.update(editingShape()!.shape.id, updatedShape);
              setEditingShape(null);
              // Reload shapes to get updated data
            }}
            onCancel={() => setEditingShape(null)}
          />
        </ModalCentered>
      </Show>
    </div>
  );
};
```

### Phase 4: Shape Definition Editor

#### 4.1 Visual Shape Editor

**File**: `ui/web/src/workshop/shape/components/ShapeDefinitionEditor.tsx`

A comprehensive visual editor for shape definitions with:

- Path editor for borders
- Color pickers for fills/strokes  
- Position controls for labels
- Connection socket placement
- Real-time preview

#### 4.2 Shape Preview Component

**File**: `ui/web/src/workshop/shape/components/ShapePreview.tsx`

SVG-based rendering of shape definitions for preview purposes.

### Phase 5: Testing

#### 5.1 Backend Tests

**File**: `server/ws/tests/workshop_shapes_int.rs`

Integration tests covering:

- Shape CRUD operations with semantic identifiers
- Package-shape associations (without overrides)
- Access control validation
- JSON definition handling
- Slug uniqueness validation

#### 5.2 Frontend Tests

**File**: `ui/web/tests-e2e/workshop_shape_management.spec.ts`

E2E tests covering:

- Shape creation flow with semantic identifiers
- Shape editing flow
- Shape deletion flow
- Package-shape association flow
- Access control scenarios

### Phase 6: API Documentation

#### 6.1 Update OpenAPI Spec

**File**: `r&d/dev-docs/openapi.yaml`

Add all workshop shape-related endpoints with request/response schemas.

## Implementation Priority

### MVP (Minimum Viable Product)

1. ✅ Database migration for enhanced shape table with semantic identifiers
2. ✅ Workshop module structure (shape storage, service, routes)
3. ✅ Basic shape CRUD with slug validation  
4. ✅ Package-shape association API (no overrides)
5. ✅ Frontend models and API integration
6. ✅ Basic shape list/add/remove in package view
7. ✅ Simple shape form with name, slug, description

### Phase 2 Enhancements

1. ✅ JSON definition storage and validation
2. ✅ Visual shape definition editor
3. ✅ Shape preview rendering
4. ✅ Global shape library management
5. ✅ Slug-based shape lookup for node definitions

### Phase 3 Advanced Features  

1. ✅ Advanced shape designer with visual tools
2. ✅ Shape templates and presets
3. ✅ Shape versioning (when package versioning is implemented)
4. ✅ Shape sharing between packages
5. ✅ Real-time collaborative editing

## Success Criteria

1. **Semantic References**: Shapes can be referenced by slug in text-based node definitions
2. **Basic Functionality**: Users can add/remove shapes to/from packages
3. **Access Control**: Only users with appropriate permissions can manage shapes  
4. **Data Integrity**: Shape definitions and slugs are properly validated and stored
5. **User Experience**: Intuitive interface for shape management
6. **Performance**: Shape operations complete within reasonable time limits
7. **Testing**: Comprehensive test coverage for all shape functionality

## Risks and Mitigation

### Risk: Complex Shape Definition Model

**Mitigation**: Start with simplified shape model, iterate based on user feedback

### Risk: Slug Collision Management

**Mitigation**: Implement robust slug validation and suggest alternatives on conflicts

### Risk: Performance with Large Shape Collections

**Mitigation**: Implement pagination and caching strategies

### Risk: JSON Schema Validation Complexity

**Mitigation**: Use well-defined JSON schemas with clear validation rules

### Risk: Visual Editor Complexity

**Mitigation**: Implement progressive enhancement, starting with simple forms

## Timeline Estimate

- **MVP Implementation**: 2-3 weeks
- **Phase 2 Enhancements**: 2-3 weeks  
- **Phase 3 Advanced Features**: 3-4 weeks
- **Total**: 7-10 weeks

## Dependencies

1. Current package system must remain stable
2. Database migration system working properly
3. Access control system functioning correctly
4. Frontend build system supporting new workshop components

## Future Considerations

### Versioning Integration

When package versioning is implemented:

- Shape versions will be tied to package versions
- Board references to shapes will need version pinning
- Migration strategies for shape definition changes

### Board-Shape Relationship

When board functionality is expanded:

- Board nodes will reference package shapes by slug
- Board may clone shapes for local modifications
- Board workshop will aggregate shapes from associated packages

### Advanced Features

1. **Shape Library Marketplace**: Allow sharing shapes across users
2. **Shape Inheritance**: Support for shape hierarchies and inheritance  
3. **Shape Animations**: Define animated transitions for shapes
4. **Shape Plugins**: Allow third-party shape definitions
5. **Shape Analytics**: Track shape usage and popularity
