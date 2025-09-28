# Workshop

## Implementation status

### Legend

[v] - implemented
[~] - partially implemented
[]  - not implemented
[?] - unknown
[MAYBE] - may be not needed?

## Concept

Workshop is a collection of diagram elements blueprints: shapes, links, connection rules, and layouts.

Diagram elements blueprints can be packaged (be be a package) to be reused in different diagrams in boards.

Board has its own workshop (as its repository of elements) consisting of items imported from packages and own items.

Elements blueprints have to be added to the workshop to be used in the diagram.

All blueprints of board's workshop should be referenceable uniquely by slug.

Slug is unique within the workshop.

Globally elements can be referenced by composition of package's or board's slug and element's slug.

Locally in board workshop items can be referenced by element's (shape, link, connection rule, layout) slug.

When importing from package, items inherit package's slug to allow import of items with same slug from different packages and then reference them locally in board's diagram.

## Shape

- is a blueprint for diagram node.
- defines visual appearance and behavior of the node in the diagram.
- defines available options for the node.
- defines restrictions for options of the node.
- defines rendering of the node.

### Model

- background - defines background properties
  - border - defines border properties
    - path - defines the border path
    - stroke - defines the border stroke
    - fill - defines the border fill
- ...
- label - defines label properties
  - position (relative to shape bounds) - defines label position
  - default text - defines default label text
  - styling - defines label styling
- content:
  - types - defines content types available
    - styling - defines content styling
    - type specific options - defines options specific to content type
- connection sockets, each
  - shape - defines the shape of the connection socket
  - tags - defines the tags associated with the connection socket
  - allowed positions - defines the allowed positions for the connection socket

content types:

- text
  - styling:
    - ...
  - type specific options:
    - ...

connection sockets:

- basic
- input
- output

connection socket shape:

- path

### Add Shape

Status []

Who: allowed user.

How:

- activate "Add Shape". []
- define shape []

## Link TODO

- is a blueprint for diagram line.
- defines visual appearance and behavior of the link in the diagram.
- defines available options for the link.
- defines restrictions for options of the link.
- defines rendering of the link.

### Model

- stroke - defines the stroke properties
  - width - defines the stroke width
  - color - defines the stroke color
- source socket - defines the source connection socket
- target socket - defines the target connection socket

### Add Link

Status []

Who: allowed user.

How:

- activate "Add Link". []
- define link []

## Connection Rule TODO



## Layout TODO



## Management of workshop (as package or in board)

Any workshop item (shape, link, connection rule, layout) has to belong to either package or board.
When importing from package to board, item's properties should stay same even if the item is modified (or deleted) in the package.

As a package or repository of board, workshop management (without board/package context specifics) is actualy a CRUD operations for:

- shape
- link
- connection rule
- layout

CRUD Operations (in board/package context but without their specifics) for workshop:

- add: create and link item to package or board it is being added to.
- edit: modify item properties.
- remove: unlink item from package or board it is being removed from and delete it.
- view: view item properties.
- list: list items in package or board workshop.

## Global workshop

- list (search): search for items in package or board workshop.
- view: view item properties.
- edit: modify item properties.

## Import from package to board

As imported item has to be referenceable in board's diagram, it has to be linked to board's workshop.
As imported items from different packages may have same slug, imported items have to inherit package's slug to be referenceable by composition of package's slug and item's slug.
As imported item has to be modifiable in board's workshop, and stay same when modified (or deleted) in package, imported item has to be copied to board's workshop. So slug maight differ from original item's slug, thus it can be changed to `package-slug:item-slug` in import if needed to avoid conflicts with existing items.

We show imported items separately from board's own items and management mechanics might differ.
