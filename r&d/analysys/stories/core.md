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
- [Access](#access)
  - [Identification](#identification)
  - [Ownership](#ownership)
  - [Invite user to board or package](#invite-user-to-board-or-package)

## Implementation status

### Legend

[v] - implemented
[~] - partially implemented
[]  - not implemented
[?] - unknown
[MAYBE] - may be not needed?

## Boards

Status [~]

### View boards

Status [~]

Who: any user [v]

How:

1. Navigate to boards page [v]
2. List available boards [v]
3. Open board view [v]

Restrictions:

1. unidentified user can view only public boards [~]
2. authenticated user can view own boards and public boards [~]
3. invited user can view boards they are invited to [~]

### Create board

Status [~]

Who: authenticated user [~]

How:

1. Navigate to boards [v]
2. Summon "Create Board" form [v]
3. Fill in board properties (name, description, icon) [v]
4. Submit form [v]

### Work with board

Status [~]

Who: any user [v]

How:

1. Navigate to boards [v]
2. Open board view [v]

restrictions:

1. unidentified user can view only public board [~]
2. authenticated user can view own board and public board [~]
3. invited user can view board they are invited to []

#### Change board properties

Status [~]

Who:

- board owner [~]
- user invited to board with "manage" permission []

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

#### Manage node shapes, link types and restrictions by shapes connection rules

Status []

Who:

- board owner []
- user invited to board with "manage" permission []

How:

0. Activate "Manage workshop" in board view []
1. by adding from package []
    - list available packages []
    - select package []
    - check items to use in board []
2. by adding internal items []
    - add new item (node shape, link type, connection rule) []
    - define item []
    - save item []

#### add package to board

Status []

Who:

- board owner []

How:

1. click "Add Package" button in board view []
2. Select package from list []
3. Click "Add" button []

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
3. invited user can view packages they are invited to []

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
3. invited user can view packages they are invited to []

#### Change package properties

Status [~]

Who:

- package owner [v]
- user invited to package with "manage" permission []

How:

1. switch to edit mode in package view [v]
2. Fill in properties form [v]
3. save changes [v]

#### Add items to package

Status []

Who:

- package owner []
- user invited to package with "manage" permission []

How:

1. Activate "Add Shape" to add new shape item to package []
    - define shape item []
    - save shape item []
2. Activate "Add Link" to add new link item to package []
    - define link item []
    - save link item []
3. Activate "Add Rule" to add new connection rule item to package []
    - define rule item []
    - save rule item []

## Access

Status [v]

### Identification

Status [~]

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

Status []

Who: board or package owner []

How:

1. Activate "Invite User" in board or package view []
2. Fill in user address and permissions []
3. Submit invitation []

Explanation:
The invited user will receive a notification and when identified by this address will have access and permissions defined in invitation.
