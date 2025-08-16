# Implements Page Component to work with packages

Package is a pack of items to be used for board composition.

Package represented by:

1. name [done]
2. description [done]
3. icon [done]
4. is public [done]
5. tags [tbd]

Package identified by numeric id.

Package holds sets of elements of different types:

1. node shapes [tbd]
2. connection lines [tbd]
3. connection rules [tbd]
4. TBD

## Page structure

### Package list

1st level view.

Allows:

1. List packages [done]
2. Add package [done]
3. Edit package properties [done]
4. Manage items of package [tbd]

Contains:

1. Title in root app header [done]
2. Buttons in root app header [done]
3. List of packages [done]
4. New package submission form [done]

Cases:

1. View package list
    - see items [part]
    - turn or scroll pages [tbd]
    - search package [tbd]
2. Submit package
    - name, description, icon, is public [done]
3. Manage items
    - navigate to package view [done]

### Package view

2nd level view.

Allows:

1. View package details [done]
2. Edit package properties [done]
3. Manage items of package [tbd]
4. Delete package [done]

Contains:

1. Title of page with package name in header [done]
2. Package properties (name, description, icon, is public, tags) [done]
3. Buttons for edit, delete, and navigation [done]
4. Lists of items in the package (node shapes, connection lines, connection rules, etc.) in blocks [tbd]
5. Forms to add new items of diferent types [tbd]

Cases:

1. View package details
    - see all properties and items
    - navigate back to package list
2. Edit package properties
    - update name, description, icon, is public, tags
3. Manage items
    - view
    - add
    - edit
    - remove
