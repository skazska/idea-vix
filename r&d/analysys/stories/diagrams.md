# Board Diagrams

## Implementation status

### Legend

[v] - implemented
[~] - partially implemented
[]  - not implemented
[?] - unknown
[MAYBE] - may be not needed?

## Concept TODO

### Diagram draw artifacts

- Shapes: represent entities or concepts
- Links: represent relationships between shapes
- Connection rules: define how shapes can be connected

### Layouts

Define possible arrangement of shapes and links in the diagram.
Layouts define discrete placements of shapes.

### Shapes

Is a geometric representation of an entity.

- Have place position (x, y) in layout
- Can have background with:
  - border line path,
  - stroke,
  - fill,
- Can have label.
- Can have content:
  - text
  - image?
  - icon?
  - line path?
  - container
- Can have spacing (occupy contiguous area of placements).
- Can have connection points with:
  - position (relative to shape bounds)
  - shape
  - tags

### Links

- Can connect shapes.
- Have line type.
- Have points.
  - color,
  - opacity
  - stroke,
  - tags
  - start/end
- Can have label.
- Can have content:
  - text
  - image?
  - icon?
  - line path?

### Connection rules

- Define how shapes can be connected by links.
- Can enforce constraints on connections (e.g., only allow certain shapes to connect by certain links or recuire shapes to be connected).

### Workshop

Provides Shapes and Links accessible in board.

## Drawing

Who:

