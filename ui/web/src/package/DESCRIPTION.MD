# Implements Page Component to work with packages

Package is a pack of items to be used for board composition.

Package represented by:

1. name [done]
2. description [done]
3. icon [done]
4. tags

Package identified by numeric id.

Package holds sets of elements of different types:

1. node shapes
2. connection lines
3. connection rules
4. TBD

## Page structure

### Package list

1st level view.

Allows:

1. List packages [done]
2. Add package [done]
3. Edit package properties
4. Manage items of package

Contains:

1. Title in root app header [done]
2. Bottons in root app header [done]
3. List of packages [done]
4. New package subbmission form [done]

Cases:

1. View package list
    - see items [part]
    - turn or scroll pages
    - search package
2. Submit package
    - name, description, icon [done]
3. Manage items
    - navigate to pachage view

### Package view

2nd level view.

Allows:

1. View package details
2. Edit package properties
3. Manage items of package
4. Delete package

Contains:

1. Title of page with package name in header
2. Package properties (name, description, icon, tags)
3. Buttons for edit, delete, and navigation
4. Lists of items in the package (node shapes, connection lines, connection rules, etc.) in blocks
5. Forms to add new items of diferent types

Cases:

1. View package details
    - see all properties and items
    - navigate back to package list
2. Edit package properties
    - update name, description, icon, tags
3. Manage items
    - view
    - add
    - edit
    - remove
