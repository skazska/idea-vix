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

Diagram elements blueprints can be packaged to be reused in different diagrams.

Diagram has own workshop including items from packages and own items.

Elements blueprints have to be added to the workshop to be used in the diagram.

All blueprints of board's workshop should be referenceable uniquely.

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