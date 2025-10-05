# User stories

## Legend

[v] - implemented
[~] - partially implemented
[]  - not implemented
[?] - unknown
[MAYBE] - may be not needed?

## Concept

To draw [diagram](diagrams.md#concept) on [Board](#boards) one need to have set of shapes and links available in the [Workshop](diagrams.md#workshop) of the board to put shapes on [layout](diagrams.md#layouts) and connect them with links according to [connection rules](diagrams.md#connection-rules).
Workshop provides a collection of predefined shapes, links with rules that can be used to create diagrams.
Shapes and links are a drawing primitives blueprints, board can be of layout.

All components can be defined for Board or imported from packages.

## Boards

Status [~]

### View boards

Status [v]

Who: any user [v]

How:

1. Navigate to boards page [v]
2. List available boards [v]
3. Open board view [v]

Restrictions:

1. unidentified user can view only public boards [v]
2. authenticated user can view own boards and public boards [v]
3. invited user can view boards they are invited to [v]

### Create board

Status [v]

Who: authenticated user [v]

How:

1. Navigate to boards [v]
2. Summon "Create Board" form [v]
3. Fill in board properties (name, description, icon) [v]
4. Submit form [v]

### Work with board

Status [v]

Who: any user [v]

How:

1. Navigate to boards [v]
2. Open board view [v]

restrictions:

1. unidentified user can view only public board [v]
2. authenticated user can view own board and public board [v]
3. invited user can view board they are invited to [v]

#### Change board properties

Status [v]

Who:

- board owner [v]
- user invited to board with "manage" permission [v]

How:

1. switch to edit mode in board view [v]
2. Fill in properties form [v]
3. save changes [v]

#### Draw shaped nodes with links

Status []

Who:

- board owner []
- user invited to board with "draw" permission []

How:

0. Switch to "Draw" mode in board view []
1. Put shapes on board: []
1.1. there is a set of available (defined) shapes to be placed at board as nodes (workshop) []
1.2. A node of shape can be placed on board as connected, when attaching a link to node which already on board []
1.3. change node shape from available in workshop []
2. Connect shapes (put links): []
2.1. link 2 placed nodes []
2.2. attach link to placed node []
2.3. change link type from available in workshop []

#### add package to board

Status []

Who:

- board owner []

How:

1. click "Add Package" button in board view []
2. Select package from list []
3. Click "Add" button []

#### remove package from board

Status []

Who:

- board owner []

How:

1. Select package from list []
2. Click "Remove" button on package []

Restrictions:

- If there are items imported from package to board, removing package from board does not remove items from board.

#### Manage node shapes, links, rules and layout (Workshop)

Status []

Who:

- board owner []
- user invited to board with "manage" permission []

##### import items from package

Status []

Who:

- board owner []
- user invited to board with "manage" permission []

How:

0. view items in package []
1. check "Use" box for items to import to board []

Restrictions:

- need to be able to reference items with same slugs from different packages []
- if there allready item with same slug in board workshop, references to it stay same []

#### unimport items from package

Status []

Who:

- board owner []
- user invited to board with "manage" permission []

How:

0. view items in workshop []
1. uncheck "Use" box for items to remove from board []

Restrictions:

- if item is used in diagram, it cannot be unimported from workshop []

##### add items to workshop

Status []

How:

0. view items in workshop [~]
1. Activate "Add Shape" to add new shape item to package []
    - define shape item []
    - save shape item []
2. Activate "Add Link" to add new link item to package []
    - define link item []
    - save link item []
3. Activate "Add Rule" to add new connection rule item to package []
    - define rule item []
    - save rule item []
4. Activate "Add Layout" to add new layout item to package []
    - define layout item []
    - save layout item []

Restrictions:

- cannot add item with same slug as any existing item in package []

##### view item

Status []

Who: any user with access to the board []

How:

1. Open board view []
2. Switch to "Workshop" tab []
3. Select item to view []
4. View item []

##### edit item

Status []

Who:

- board owner []
- user invited to board with "manage" permission []

How:

1. Open board view []
2. Switch to "Workshop" tab []
3. Select item to edit []
4. Edit item []

##### remove item from workshop

Status []

Who:

- board owner []
- user invited to board with "manage" permission []

How:

1. Open board view []
2. Switch to "Workshop" tab []
3. Select item to remove []
4. Remove item []

Restrictions:

- cannot remove item if it is used in diagram []

## Delete board

Status []

Who:

- board owner []

How:

1. Navigate to board view []
2. Click "Delete Board" button []
3. Confirm deletion []

## Notes

Crud operations for board is similar to package crud operations.

## Packages

Status [~]

### View packages

Status [~]

Who: any user [v]

How:

1. Navigate to packages page [v]
2. List available packages [v]
3. Open package view [v]

Restrictions:

1. unidentified user can view only public packages [v]
2. authenticated user can view own packages and public packages [v]
3. invited user can view packages they are invited to [v]
4. package with no version is visible only to its owner []

### Create package

Status [v]

Who: authenticated user [v]

How:

1. Navigate to packages page [v]
2. Summon "Create Package" form [v]
3. Fill in package properties (name, description, icon, is_public) [v]
4. Submit form [v]

### Work with package

Status [~]

Who: any user [v]

How:

1. Navigate to packages page [v]
2. Open package view [v]

Restrictions:

1. unidentified user can view only public packages [v]
2. authenticated user can view own packages and public packages [v]
3. invited user can view packages they are invited to [v]

#### Change package properties

Status [v]

Who:

- package owner [v]
- user invited to package with "manage" permission [v]

How:

1. switch to edit mode in package view [v]
2. Fill in properties form [v]
3. save changes [v]

#### Add items to workshop

Status [~]

Who:

- package owner []
- user invited to package with "manage" permission []

How:

0. view items in workshop [~]
1. Activate "Add Shape" to add new shape item to package [v]
    - define shape item []
    - save shape item [v]
2. Activate "Add Link" to add new link item to package [v]
    - define link item []
    - save link item [v]
3. Activate "Add Rule" to add new connection rule item to package [v]
    - define rule item []
    - save rule item [v]
4. Activate "Add Layout" to add new layout item to package [v]
    - define layout item []
    - save layout item [v]

Restrictions:

- slugs of items in package must be unique []

#### view package item

Status []

Who: any user with access to the package []

How:

1. Open package view []
2. Select item to view []
3. View item []

#### edit package item

Status []

Who:

- package owner []
- user invited to package with "manage" permission []

How:

1. Activate "Edit Item" of item in package view's item list []
2. Modify item []
3. Save changes []

Restrictions:

#### remove item from package

Status []

Who:

- package owner []
- user invited to package with "manage" permission []

How:

1. Activate "Remove Item" in package view []
2. Select item to remove []
3. Confirm removal []

Restrictions:

### Delete package

Status []

Who: package owner []

How:

1. Navigate to package view []
2. Click "Delete Package" button []
3. Confirm deletion []

Restrictions:

Deleting package does not remove its usage in boards or imported items from boards.

### Publishing

Status []

Who:

- package owner []
- user invited to package with "manage" permission []

How:

1. Navigate to package view []
2. Click "Publish" button []
3. Confirm publishing []

Restrictions:

- on create package is in draft state until published []
- after publishing, first change to package creates new draft []
- only published package can be imported to board []
- package in draft state is visible only to its owner and users with "manage" permission []
- each publishing increments package version []

### Package Notes

Crud operations for package is similar to board crud operations.

## Access

Status [v]

### Identification

Status [v]

Who: any user [v]

How:

1. Activate "Sign In" [v]
2. Fill address to receive confirmation code [v]
3. Submit [v]
4. Enter confirmation code [v]
5. Submit [v]

### Ownership

Status [v]

Who: identified user [v]

How:

1. Create board [v]
2. Create package [v]

Explanation:
package and board ownership is defined by address used for identification.

### Invite user to board or package

Status [v]

Who: board or package owner [v]

How:

1. View list of invited users [v]
   - backend [v]
   - frontend [v]
2. Activate "Invite User" in package view [v]
3. Fill in user address and permissions [v]
4. Submit invitation [v]
   - backend [v]
   - frontend [v]
5. Revoke invitation [v]
   - backend [v]
   - frontend [v]

Explanation:
The invited user will receive a notification and when identified by this address will have access and permissions defined in invitation.
