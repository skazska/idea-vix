# User stories User stories

- [Boards](#boards)
  - [View boards](#view-boards)
  - [Create board](#create-board)
  - [Work with board](#work-with-board)
    - [Change board properties](#change-board-properties)
    - [Draw shaped nodes with links](#draw-shaped-nodes-with-links)
    - [Manage node shapes, link types and restrictions by shapes connection rules](#manage-node-shapes-link-types-and-restrictions-by-shapes-connection-rules)
    - [add package to board](#add-package-to-board)
- [Packages](#packages)
  - [View packages](#view-packages)
  - [Create package](#create-package)
  - [Work with package](#work-with-package)
    - [Change package properties](#change-package-properties)
    - [Add items to package](#add-items-to-package)
- [Users](#users)
  - [Identification](#identification)
  - [Ownership](#ownership)
  - [Invite user to board or package](#invite-user-to-board-or-package)

## Boards

### View boards

Who: any user

How:

1. Navigate to boards page
2. List available boards
3. Open board view

Restrictions:

1. unidentified user can view only public boards
2. authenticated user can view own boards and public boards
3. invited user can view boards they are invited to

### Create board

Who: authenticated user

How:

1. Navigate to boards
2. Summon "Create Board" form
3. Fill in board properties (name, description, icon)
4. Submit form

### Work with board

Who: any user

How:

1. Navigate to boards
2. Open board view

restrictions:

1. unidentified user can view only public board
2. authenticated user can view own board and public board
3. invited user can view board they are invited to

#### Change board properties

Who:

- board owner
- user invited to board with "manage" permission

How:

1. switch to edit mode in board view
2. Fill in properties form
3. save changes

#### Draw shaped nodes with links

Who:

- board owner
- user invited to board with "draw" permission

How:

0. Switch to "Draw" mode in board view
1. Put shapes on board:
1.1. there is a set of available (defined) shapes to be placed at board as nodes (workshop)
1.2. A node of shape can be placed on board as connected, when attaching a link to node which already on board.
1.3. change node shape from available in workshop
2. Connect shapes (put links):
2.1. link 2 placed nodes
2.2. attach link to placed node
2.3. change link type from available in workshop

#### Manage node shapes, link types and restrictions by shapes connection rules

Who:

- board owner
- user invited to board with "manage" permission

How:

0. Activate "Manage workshop" in board view
1. by adding from package
    - list available packages
    - select package
    - check items to use in board
2. by adding internal items
    - add new item (node shape, link type, connection rule)
    - define item
    - save item

#### add package to board

Who:

- board owner

How:

1. click "Add Package" button in board view
2. Select package from list
3. Click "Add" button

## Packages

### View packages

Who: any user

How:

1. Navigate to packages page
2. List available packages
3. Open package view

Restrictions:

1. unidentified user can view only public packages
2. authenticated user can view own packages and public packages
3. invited user can view packages they are invited to [MAYBE]

### Create package

Who: authenticated user

How:

1. Navigate to packages page
2. Summon "Create Package" form
3. Fill in package properties (name, description, icon)
4. Submit form

### Work with package

Who: any user

How:

1. Navigate to packages page
2. Open package view

Restrictions:

1. unidentified user can view only public packages
2. authenticated user can view own packages and public packages
3. invited user can view packages they are invited to [MAYBE]

#### Change package properties

Who:

- package owner
- user invited to package with "manage" permission [MAYBE]

How:

1. switch to edit mode in package view
2. Fill in properties form
3. save changes

#### Add items to package

Who:

- package owner
- user invited to package with "manage" permission [MAYBE]

How:

1. Activate "Add Shape" to add new shape item to package
    - define shape item
    - save shape item
2. Activate "Add Link" to add new link item to package
    - define link item
    - save link item
3. Activate "Add Rule" to add new connection rule item to package
    - define rule item
    - save rule item

## Users

### Identification

Who: any user

How:

1. Activate "Sign In"
2. Fill address to receive confirmation code
3. Submit
4. Enter confirmation code
5. Submit

### Ownership

Who: identified user

How:

1. Create board
2. Create package

Explanation:
package and board ownership is defined by address used for identification.

### Invite user to board or package

Who: board or package owner

How:

1. Activate "Invite User" in board or package view
2. Fill in user address and permissions
3. Submit invitation

Explanation:
The invited user will receive a notification and when identified by this address will have access and permissions defined in invitation.
