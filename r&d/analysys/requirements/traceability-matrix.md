# Traceability Matrix

This matrix links implemented user stories (`r&d/analysys/stories`) to documented requirements and automated tests across the backend (`server/ws/tests`) and frontend end-to-end flows (`ui/web/tests-e2e`).

Legend:

- **Status** mirrors the implementation flag in the source story (`[v]` implemented, `[~]` partially, `[]` not implemented).
- **Coverage** prefixes: **B:** backend integration tests, **F:** frontend Playwright specs.

<!-- markdownlint-disable MD033 -->

## Boards

<table>
  <thead>
    <tr>
      <th>Story / Source</th>
      <th>Acceptance Criteria</th>
      <th>Automated Coverage</th>
      <th>Gaps &amp; Notes</th>
      <th>Status</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td rowspan="3">Boards · <a href="../stories/core.md#view-boards">View boards</a></td>
      <td>Boards visible to the viewer are listed.</td>
      <td>B: <code>boards_smoke.rs::boards_crud_ok</code></td>
      <td rowspan="3">No UI coverage for unauthenticated viewer vs. invited guest flows</td>
      <td rowspan="3">[v]</td>
    </tr>
    <tr>
      <td>Unauthenticated viewers see only public boards.</td>
      <td>B: <code>boards_smoke.rs::boards_crud_ok</code></td>
    </tr>
    <tr>
      <td>Invited users can access private boards they are assigned to.</td>
      <td>B: <code>boards_smoke.rs::board_access_invite_and_permissions</code></td>
    </tr>
    <tr>
      <td rowspan="4">Boards · <a href="../stories/core.md#create-board">Create board</a></td>
      <td>Only authenticated users can create boards.</td>
      <td>B: <code>boards_regression.rs::board_create_requires_authentication</code></td>
      <td rowspan="4">Negative cases (validation errors) beyond documented checks remain untested</td>
      <td rowspan="4">[v]</td>
    </tr>
    <tr>
      <td>A valid name/description/icon submission creates a board and returns it to the caller.</td>
      <td>
        B: <code>boards_smoke.rs::boards_crud_ok</code><br/>
        F: <code>boards_flow.spec.ts</code> — create private/public
      </td>
    </tr>
    <tr>
      <td>Name validation rejects too-short board names.</td>
      <td>B: <code>boards_regression.rs::board_create_rejects_short_name</code></td>
    </tr>
    <tr>
      <td>Creating a board with a conflicting slug returns HTTP <code>409</code>.</td>
      <td>B: <code>boards_regression.rs::board_slug_conflict_returns_conflict</code></td>
    </tr>
    <tr>
      <td rowspan="3">Boards · <a href="../stories/core.md#change-board-properties">Change board properties</a></td>
      <td>Owner/manage roles can update board fields and visibility.</td>
      <td>B: <code>boards_smoke.rs::boards_crud_ok</code></td>
      <td rowspan="3">No automated check for edit-role vs. manage-role differences or null field clearing via UI</td>
      <td rowspan="3">[v]</td>
    </tr>
    <tr>
      <td>Edit-role users are blocked from updates that require manage permissions.</td>
      <td>B: <code>boards_regression.rs::board_update_requires_elevated_role</code></td>
    </tr>
    <tr>
      <td>Manage-role users can delete boards when required.</td>
      <td>B: <code>boards_regression.rs::board_delete_allows_manage_role</code></td>
    </tr>
    <tr>
      <td rowspan="4">Boards · <a href="../stories/core.md#draw-shaped-nodes-with-links">Draw shaped nodes with links</a></td>
      <td>Authorized users can switch a board into draw mode.</td>
      <td>—</td>
      <td rowspan="4">No API or UI coverage; plan backend workshop draw-mode contract tests and UI canvas regression once feature lands.</td>
      <td rowspan="4">[]</td>
    </tr>
    <tr>
      <td>Shapes from the board workshop can be placed as nodes with stable slug references.</td>
      <td>—</td>
    </tr>
    <tr>
      <td>Links connect eligible nodes per connection rules and reject invalid socket pairs.</td>
      <td>—</td>
    </tr>
    <tr>
      <td>Nodes and links can switch to another blueprint without breaking existing connections.</td>
      <td>—</td>
    </tr>
    <tr>
      <td rowspan="3">Boards · <a href="../stories/core.md#add-package-to-board">Add package to board</a></td>
      <td>Owner/manage roles can list packages available for attachment.</td>
      <td>—</td>
      <td rowspan="3">Pending backend attach/detach endpoints and UI flows; add tests to verify invitation roles inherit package access.</td>
      <td rowspan="3">[]</td>
    </tr>
    <tr>
      <td>Selecting a package attaches it to the board and exposes its workshop items.</td>
      <td>—</td>
    </tr>
    <tr>
      <td>Re-attaching an already linked package surfaces a clear conflict.</td>
      <td>—</td>
    </tr>
    <tr>
      <td rowspan="3">Boards · <a href="../stories/core.md#remove-package-from-board">Remove package from board</a></td>
      <td>Owner/manage roles can detach a package from the board.</td>
      <td>—</td>
      <td rowspan="3">Need backend regression ensuring imported items persist and UI confirmation preventing accidental loss.</td>
      <td rowspan="3">[]</td>
    </tr>
    <tr>
      <td>Imported items remain available after the source package is detached.</td>
      <td>—</td>
    </tr>
    <tr>
      <td>Detaching the final package updates the board workshop view without orphan references.</td>
      <td>—</td>
    </tr>
  </tbody>
</table>

## Packages

<table>
  <thead>
    <tr>
      <th>Story / Source</th>
      <th>Acceptance Criteria</th>
      <th>Automated Coverage</th>
      <th>Gaps &amp; Notes</th>
      <th>Status</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td rowspan="3">Packages · <a href="../stories/core.md#view-packages">View packages</a></td>
      <td>Packages visible to the viewer are listed.</td>
      <td>
        B: <code>packages_smoke.rs::packages_crud_ok</code><br/>
        F: <code>package_flow.spec.ts</code> — owner view
      </td>
      <td rowspan="3">Frontend lacks invited-user visibility checks; story marked `[~]` pending versioning rules</td>
      <td rowspan="3">[~]</td>
    </tr>
    <tr>
      <td>Unauthenticated viewers see only public packages.</td>
      <td>B: <code>packages_regression.rs::private_package_hidden_from_unauthenticated_list</code></td>
    </tr>
    <tr>
      <td>Invited users can access private packages they are assigned to.</td>
      <td>B: <code>packages_smoke.rs::package_access_invite_and_permissions</code></td>
    </tr>
    <tr>
      <td rowspan="4">Packages · <a href="../stories/core.md#create-package">Create package</a></td>
      <td>A valid name/description/icon submission creates a package and returns it to the caller.</td>
      <td>
        B: <code>packages_smoke.rs::packages_crud_ok</code><br/>
        F: <code>package_flow.spec.ts</code> — create private/public
      </td>
      <td rowspan="4">Additional validation scenarios (e.g., icon length, duplicate metadata) remain untested</td>
      <td rowspan="4">[v]</td>
    </tr>
    <tr>
      <td>Optional slug values can be provided during creation.</td>
      <td>B: <code>packages_smoke.rs::packages_crud_ok</code></td>
    </tr>
    <tr>
      <td>Creating a package with a conflicting slug returns HTTP <code>409</code>.</td>
      <td>B: <code>packages_regression.rs::package_slug_conflict_returns_conflict</code></td>
    </tr>
    <tr>
      <td>The creator is assigned owner/manage access to the new package.</td>
  <td>B: <code>packages_smoke.rs::package_access_invite_and_permissions</code></td>
    </tr>
    <tr>
      <td rowspan="4">Packages · <a href="../stories/core.md#change-package-properties">Change package properties</a></td>
      <td>Owner/manage roles can update package fields and visibility.</td>
      <td>
        B: <code>packages_smoke.rs::packages_crud_ok</code><br/>
        F: <code>package_flow.spec.ts</code> — update private/public
      </td>
      <td rowspan="4">UI lacks explicit test for manage-role editing; null-field clearing not exercised</td>
      <td rowspan="4">[v]</td>
    </tr>
    <tr>
      <td>View-role users are blocked from performing updates.</td>
      <td>B: <code>packages_regression.rs::package_update_forbidden_for_view_role</code></td>
    </tr>
    <tr>
      <td>Owner/manage roles can delete packages when required.</td>
      <td>B: <code>packages_regression.rs::package_delete_requires_owner_role</code></td>
    </tr>
    <tr>
      <td>Optional fields can be cleared via update payloads.</td>
      <td>B: <code>packages_regression.rs::package_update_clears_optional_fields</code></td>
    </tr>
    <tr>
      <td rowspan="3">Packages · <a href="../stories/core.md#delete-package">Delete package</a></td>
      <td>Package owner can initiate deletion from the package view.</td>
      <td>—</td>
      <td rowspan="3">Need backend delete endpoint regression and confirmation flow coverage.</td>
      <td rowspan="3">[]</td>
    </tr>
    <tr>
      <td>Successful deletion returns the removed package payload and it disappears from subsequent lists.</td>
      <td>—</td>
    </tr>
    <tr>
      <td>Deleting a package does not remove its usage or imported items from boards.</td>
      <td>—</td>
    </tr>
    <tr>
      <td rowspan="6">Packages · <a href="../stories/core.md#publishing">Publishing</a></td>
      <td>Newly created packages remain in draft until explicitly published.</td>
      <td>—</td>
      <td rowspan="6">Feature not yet implemented; requires draft/publish state management tests and board import integration.</td>
      <td rowspan="6">[]</td>
    </tr>
    <tr>
      <td>Publishing a package moves the current draft to a published version.</td>
      <td>—</td>
    </tr>
    <tr>
      <td>After publishing, the next change creates a new draft revision.</td>
      <td>—</td>
    </tr>
    <tr>
      <td>Only published packages can be imported into boards.</td>
      <td>—</td>
    </tr>
    <tr>
      <td>Draft packages are visible only to the owner and manage-role collaborators.</td>
      <td>—</td>
    </tr>
    <tr>
      <td>Each publish action increments the package version counter.</td>
      <td>—</td>
    </tr>
  </tbody>
</table>

## Access

<table>
  <thead>
    <tr>
      <th>Story / Source</th>
      <th>Acceptance Criteria</th>
      <th>Automated Coverage</th>
      <th>Gaps &amp; Notes</th>
      <th>Status</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td rowspan="4">Access · <a href="../stories/core.md#invite-user-to-board-or-package">Invite user to board</a></td>
      <td>Owner can list current board invitations.</td>
      <td>B: <code>boards_smoke.rs::board_access_invite_and_permissions</code></td>
      <td rowspan="4">UI test grants “edit” only; no automated regression for conflict/duplicate handling</td>
      <td rowspan="4">[v]</td>
    </tr>
    <tr>
      <td>Owner can grant roles to invitees.</td>
      <td>
        B: <code>boards_smoke.rs::board_access_invite_and_permissions</code><br/>
        F: <code>board_access_flow.spec.ts</code>
      </td>
    </tr>
    <tr>
      <td>Invited guests gain permissions that match the granted role.</td>
      <td>
        B: <code>boards_smoke.rs::board_access_invite_and_permissions</code><br/>
        F: <code>board_access_flow.spec.ts</code>
      </td>
    </tr>
    <tr>
      <td>Owner can revoke an existing invitation.</td>
      <td>
        B: <code>boards_smoke.rs::board_access_invite_and_permissions</code><br/>
        F: <code>board_access_flow.spec.ts</code>
      </td>
    </tr>
    <tr>
      <td rowspan="4">Access · <a href="../stories/core.md#invite-user-to-board-or-package">Invite user to package</a></td>
      <td>Owner can list current package invitations.</td>
      <td>B: <code>packages_smoke.rs::package_access_invite_and_permissions</code></td>
      <td rowspan="4">No automated coverage for conflict errors or downgrade flows</td>
      <td rowspan="4">[v]</td>
    </tr>
    <tr>
      <td>Owner can grant roles to package invitees.</td>
      <td>
        B: <code>packages_smoke.rs::package_access_invite_and_permissions</code><br/>
        F: <code>package_access_flow.spec.ts</code>
      </td>
    </tr>
    <tr>
      <td>Invited users receive permissions matching the granted role.</td>
      <td>
        B: <code>packages_smoke.rs::package_access_invite_and_permissions</code><br/>
        F: <code>package_access_flow.spec.ts</code>
      </td>
    </tr>
    <tr>
      <td>Owner can revoke an invitation to the package.</td>
      <td>
        B: <code>packages_smoke.rs::package_access_invite_and_permissions</code><br/>
        F: <code>package_access_flow.spec.ts</code>
      </td>
    </tr>
    <tr>
      <td rowspan="3">Access · <a href="../stories/core.md#identification">Identification</a></td>
      <td>Sign-in flow issues a verification code to the submitted email address.</td>
      <td>
        B: <code>sessions_smoke.rs::sessions_flow_ok</code><br/>
        F: <code>auth_flow.spec.ts</code> — sign-in &amp; sign-out
      </td>
      <td rowspan="3">Backend lacks negative-path tests (invalid code, expired session)</td>
      <td rowspan="3">[v]</td>
    </tr>
    <tr>
      <td>Submitting the received code establishes an authenticated session.</td>
      <td>
        B: <code>sessions_smoke.rs::sessions_flow_ok</code><br/>
        F: <code>auth_flow.spec.ts</code> — sign-in &amp; sign-out
      </td>
    </tr>
    <tr>
      <td><code>/api/session/me</code> returns the authenticated user data.</td>
      <td>
        B: <code>sessions_smoke.rs::sessions_flow_ok</code><br/>
        F: <code>auth_flow.spec.ts</code> — sign-in &amp; sign-out
      </td>
    </tr>
    <tr>
      <td rowspan="2">Access · <a href="../stories/core.md#ownership">Ownership</a></td>
      <td>Creator becomes owner of newly created boards.</td>
      <td>B: <code>boards_smoke.rs::board_access_invite_and_permissions</code></td>
      <td rowspan="2">No dedicated UI asserting ownership labeling</td>
      <td rowspan="2">[v]</td>
    </tr>
    <tr>
      <td>Creator becomes owner of newly created packages.</td>
      <td>B: <code>packages_smoke.rs::package_access_invite_and_permissions</code></td>
    </tr>
  </tbody>
</table>

## Workshop

<table>
  <thead>
    <tr>
      <th>Story / Source</th>
      <th>Acceptance Criteria</th>
      <th>Automated Coverage</th>
      <th>Gaps &amp; Notes</th>
      <th>Status</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td rowspan="4">Workshop · <a href="../stories/core.md#manage-node-shapes-links-rules-and-layout">Manage node shapes/links</a></td>
      <td>Board workshop lists shapes, links, rules, and layouts with unique slugs.</td>
      <td>—</td>
      <td rowspan="4">Feature remains `[ ]`; requires end-to-end workshop CRUD coverage plus backend contract tests before GA.</td>
      <td rowspan="4">[]</td>
    </tr>
    <tr>
      <td>Allowed roles can create, update, and delete each blueprint while preserving slug uniqueness.</td>
      <td>—</td>
    </tr>
    <tr>
      <td>Blueprint definitions capture geometry, labels, sockets, and rules per <a href="../descisions/workshop.md">workshop decisions</a>.</td>
      <td>—</td>
    </tr>
    <tr>
      <td>Layout changes propagate to draw mode without breaking existing nodes or links.</td>
      <td>—</td>
    </tr>
    <tr>
      <td rowspan="3">Boards · <a href="../stories/core.md#import-items-from-package">Import items from package</a></td>
      <td>Manage role can browse package workshop items and mark them for import.</td>
      <td>—</td>
      <td rowspan="3">Need backend copy semantics tests for slug composition and UI regression for import list refresh.</td>
      <td rowspan="3">[]</td>
    </tr>
    <tr>
      <td>Imported items inherit the package slug without colliding with existing board items.</td>
      <td>—</td>
    </tr>
    <tr>
      <td>Imported blueprints become available in board draw mode immediately.</td>
      <td>—</td>
    </tr>
    <tr>
      <td rowspan="3">Boards · <a href="../stories/core.md#unimport-items-from-package">Unimport items from package</a></td>
      <td>Manage role can unmark imported items from the board workshop list.</td>
      <td>—</td>
      <td rowspan="3">Add regression asserting dependency checks and verifying draw mode palette updates.</td>
      <td rowspan="3">[]</td>
    </tr>
    <tr>
      <td>System prevents unimporting items that are currently used on a board layout.</td>
      <td>—</td>
    </tr>
    <tr>
      <td>Unimported items disappear from draw mode while preserved ones keep functioning.</td>
      <td>—</td>
    </tr>
    <tr>
      <td rowspan="3">Packages · <a href="../stories/core.md#add-items-to-package">Add items to workshop</a></td>
      <td>Owner/manage roles can add shape, link, rule, and layout blueprints.</td>
      <td>—</td>
      <td rowspan="3">Implementation in progress `[~]`; add backend validation suites and UI form regression.</td>
      <td rowspan="3">[~]</td>
    </tr>
    <tr>
      <td>Blueprints validate required geometry, sockets, and styling per <a href="../descisions/workshop.md">workshop spec</a>.</td>
      <td>—</td>
    </tr>
    <tr>
      <td>Slug uniqueness is enforced within the package; duplicates yield validation errors.</td>
      <td>—</td>
    </tr>
    <tr>
      <td rowspan="3">Packages · <a href="../stories/core.md#view-item-1">View item</a></td>
      <td>Any viewer with package access can open a workshop item detail view.</td>
      <td>—</td>
      <td rowspan="3">No automated coverage; future tests should snapshot API payloads and UI render fidelity.</td>
      <td rowspan="3">[]</td>
    </tr>
    <tr>
      <td>Detail view renders blueprint metadata exactly as stored.</td>
      <td>—</td>
    </tr>
    <tr>
      <td>Versionless packages hide unpublished items from non-owners.</td>
      <td>—</td>
    </tr>
    <tr>
      <td rowspan="3">Packages · <a href="../stories/core.md#edit-1">Edit item</a></td>
      <td>Owner/manage roles can modify blueprint metadata while keeping slug immutable.</td>
      <td>—</td>
      <td rowspan="3">Need backend mutation tests for partial updates and UI form validation scenarios.</td>
      <td rowspan="3">[]</td>
    </tr>
    <tr>
      <td>Changes propagate to dependent boards on next import without overwriting local overrides.</td>
      <td>—</td>
    </tr>
    <tr>
      <td>Invalid edits (e.g., removing required sockets) return validation errors.</td>
      <td>—</td>
    </tr>
    <tr>
      <td rowspan="3">Packages · <a href="../stories/core.md#remove-item-from-package">Remove item from package</a></td>
      <td>Owner/manage roles can remove workshop items from a package.</td>
      <td>—</td>
      <td rowspan="3">Pending delete endpoint; add tests covering dependency checks and UI confirmations.</td>
      <td rowspan="3">[]</td>
    </tr>
    <tr>
      <td>Removal is blocked when items are imported by boards to prevent breaking diagrams.</td>
      <td>—</td>
    </tr>
    <tr>
      <td>Successful removal updates package listings and future import availability.</td>
      <td>—</td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-enable MD033 -->

## Summary

- CRUD and access-management stories now map to explicit backend and frontend checks, making coverage gaps (validation negatives, invited-user UI flows) easier to spot at a glance.
- Board draw mode, package attachments, delete flows, and publishing lifecycles have explicit acceptance criteria with clear automation gaps for upcoming work.
- Session authentication is covered for the happy path across backend and UI, but failure-mode tests are still outstanding.
