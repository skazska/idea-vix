# Implements Page Component to work with boards

Board is a pack of items to be used for board composition.

Board represented by:

1. name [done]
2. description [done]
3. icon [done]
4. tags

Board identified by numeric id.

Board holds:

1. config of used packages, their
1.1. node shapes
1.2. connection lines
1.3. connection rules
1.4. etc.
2. config of own sets of items (along with packaged) (shapes, lines, rules, etc.) i.e. internal package.
3. instances of shapes, lines, etc. artefacts as board content

## Page structure

### Board list

1st level view.

Allows:

1. List boards [done]
2. Add board [done]
3. Edit board properties
4. Manage items of board

Contains:

1. Title in root app header [done]
2. Bottons in root app header [done]
3. List of boards [done]
4. New board subbmission form [done]

Cases:

1. View board list
    - see items [part]
    - turn or scroll pages
    - search board
2. Submit board
    - name, description, icon [done]
3. Manage items
    - navigate to pachage view

### Board view

2nd level view.

Allows:

1. View board details
2. Edit board properties
3. Manage items of board
4. Delete board

Contains:

1. Title of page with board name in header
2. Board properties (name, description, icon, tags)
3. Buttons for edit, delete, and navigation
4. Lists of items in the board (node shapes, connection lines, connection rules, etc.) in blocks
5. Forms to add new items of diferent types

Cases:

1. View board details
    - see all properties and items
    - navigate back to board list
2. Edit board properties
    - update name, description, icon, tags
3. Manage items
    - view
    - add
    - edit
    - remove
