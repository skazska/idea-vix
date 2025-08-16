# Implements Page Component to work with boards

Board is a pack of items to be used for board composition.

Board represented by:

1. name [done]
2. description [done]
3. icon [done]
4. is public [done]
5. tags [tbd]

Board identified by numeric id.

Board holds:

1. config of used packages, their  [tbd]
1.1. node shapes
1.2. connection lines
1.3. connection rules
1.4. etc.
2. config of own sets of items (along with packaged) (shapes, lines, rules, etc.) i.e. internal package. [tbd]
3. instances of shapes, lines, etc. artefacts as board content [tbd]

## Page structure

### Board list

1st level view.

Allows:

1. List boards [done]
2. Add board [done]
3. Edit board properties [done]
4. Manage items of board [tbd]

Contains:

1. Title in root app header [done]
2. Bottons in root app header [done]
3. List of boards [done]
4. New board subbmission form [done]

Cases:

1. View board list
    - see items [part]
    - turn or scroll pages [tbd]
    - search board [tbd]
2. Submit board
    - name, description, icon, is public [done]
3. Manage items
    - navigate to package view [done]

### Board view

2nd level view.

Allows:

1. View board details [done]
2. Edit board properties [done]
3. Manage items of board [tbd]
4. Delete board [done]

Contains:

1. Title of page with board name in header
2. Board properties (name, description, icon, is public, tags)
3. Buttons for edit, delete, and navigation [done]
4. Lists of items in the board (node shapes, connection lines, connection rules, etc.) in blocks [tbd]
5. Forms to add new items of diferent types [tbd]

Cases:

1. View board details
    - see all properties and items
    - navigate back to board list
2. Edit board properties
    - update name, description, icon, is public, tags
3. Manage items
    - view
    - add
    - edit
    - remove
