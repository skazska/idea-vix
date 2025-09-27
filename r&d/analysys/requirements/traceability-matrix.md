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
      <td rowspan="2">Boards · <a href="../stories/core.md#view-boards">View boards</a></td>
      <td rowspan="2">List boards, respect public/private visibility, invited users can view private boards</td>
      <td>B: <code>boards_int.rs::boards_crud_ok</code></td>
      <td rowspan="2">No UI coverage for unauthenticated viewer vs. invited guest flows</td>
      <td rowspan="2">[v]</td>
    </tr>
    <tr>
      <td>B: <code>boards_int.rs::board_access_invite_and_permissions</code></td>
    </tr>
    <tr>
      <td rowspan="2">Boards · <a href="../stories/core.md#create-board">Create board</a></td>
      <td rowspan="2">Authenticated user can submit form with name/description/icon and see new board</td>
      <td>B: <code>boards_int.rs::boards_crud_ok</code></td>
      <td rowspan="2">Negative cases (validation errors) untested</td>
      <td rowspan="2">[v]</td>
    </tr>
    <tr>
      <td>F: <code>boards_flow.spec.ts</code> — create private/public</td>
    </tr>
    <tr>
      <td rowspan="3">Boards · <a href="../stories/core.md#change-board-properties">Change board properties</a></td>
      <td rowspan="3">Owner/manage can edit fields, visibility toggles persisted</td>
      <td>B: <code>boards_int.rs::boards_crud_ok</code></td>
      <td rowspan="3">No automated check for edit-role vs. manage-role differences or null field clearing via UI</td>
      <td rowspan="3">[v]</td>
    </tr>
    <tr>
      <td>B: <code>boards_int.rs::board_access_invite_and_permissions</code></td>
    </tr>
    <tr>
      <td>F: <code>boards_flow.spec.ts</code> — update scenarios</td>
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
      <td rowspan="3">List packages with visibility rules; invited users access private packages</td>
      <td>B: <code>packages_int.rs::packages_crud_ok</code></td>
      <td rowspan="3">Frontend lacks invited-user visibility checks; story marked `[~]` pending versioning rules</td>
      <td rowspan="3">[~]</td>
    </tr>
    <tr>
      <td>B: <code>packages_int.rs::package_access_invite_and_permissions</code></td>
    </tr>
    <tr>
      <td>F: <code>package_flow.spec.ts</code> — owner view</td>
    </tr>
    <tr>
      <td rowspan="2">Packages · <a href="../stories/core.md#create-package">Create package</a></td>
      <td rowspan="2">Authenticated user creates package with metadata and optional slug, sees it in list</td>
      <td>B: <code>packages_int.rs::packages_crud_ok</code></td>
      <td rowspan="2">No coverage for slug conflicts or validation failures</td>
      <td rowspan="2">[v]</td>
    </tr>
    <tr>
      <td>F: <code>package_flow.spec.ts</code> — create private/public</td>
    </tr>
    <tr>
      <td rowspan="3">Packages · <a href="../stories/core.md#change-package-properties">Change package properties</a></td>
      <td rowspan="3">Owner/manage can edit fields, toggle visibility, retain slug</td>
      <td>B: <code>packages_int.rs::packages_crud_ok</code></td>
      <td rowspan="3">UI lacks explicit test for manage-role editing; null-field clearing not exercised</td>
      <td rowspan="3">[v]</td>
    </tr>
    <tr>
      <td>B: <code>packages_int.rs::package_access_invite_and_permissions</code></td>
    </tr>
    <tr>
      <td>F: <code>package_flow.spec.ts</code> — update private/public</td>
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
      <td rowspan="2">Access · <a href="../stories/core.md#invite-user-to-board-or-package">Invite user to board</a></td>
      <td rowspan="2">Owner lists invites, grants role, guest gains permissions, owner can revoke</td>
      <td>B: <code>boards_int.rs::board_access_invite_and_permissions</code></td>
      <td rowspan="2">UI test grants “edit” only; no automated regression for conflict/duplicate handling</td>
      <td rowspan="2">[v]</td>
    </tr>
    <tr>
      <td>F: <code>board_access_flow.spec.ts</code></td>
    </tr>
    <tr>
      <td rowspan="2">Access · <a href="../stories/core.md#invite-user-to-board-or-package">Invite user to package</a></td>
      <td rowspan="2">Owner grants/revokes roles; invited user permissions enforced</td>
      <td>B: <code>packages_int.rs::package_access_invite_and_permissions</code></td>
      <td rowspan="2">No automated coverage for conflict errors or downgrade flows</td>
      <td rowspan="2">[v]</td>
    </tr>
    <tr>
      <td>F: <code>package_access_flow.spec.ts</code></td>
    </tr>
    <tr>
      <td rowspan="2">Access · <a href="../stories/core.md#identification">Identification</a></td>
      <td rowspan="2">Email-based sign-in flow issues code, verification establishes session, <code>/api/session/me</code> returns data</td>
      <td>B: <code>sessions_int.rs::sessions_flow_ok</code></td>
      <td rowspan="2">Backend lacks negative-path tests (invalid code, expired session)</td>
      <td rowspan="2">[v]</td>
    </tr>
    <tr>
      <td>F: <code>auth_flow.spec.ts</code> — sign-in &amp; sign-out</td>
    </tr>
    <tr>
      <td rowspan="2">Access · <a href="../stories/core.md#ownership">Ownership</a></td>
      <td rowspan="2">Creator becomes owner of created boards/packages</td>
      <td>B: <code>boards_int.rs::board_access_invite_and_permissions</code></td>
      <td rowspan="2">No dedicated UI asserting ownership labeling</td>
      <td rowspan="2">[v]</td>
    </tr>
    <tr>
      <td>B: <code>packages_int.rs::package_access_invite_and_permissions</code></td>
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
      <td>Workshop · <a href="../stories/core.md#manage-node-shapes-links-rules-and-layout">Manage node shapes/links</a></td>
      <td>Define and manage workshop items (shapes/links/rules/layouts) for boards</td>
      <td>—</td>
      <td>Feature remains `[ ]`; add tests when functionality ships</td>
      <td>[]</td>
    </tr>
    <tr>
      <td>Packages · <a href="../stories/core.md#add-items-to-package">Add items to workshop</a></td>
      <td>Define package-local workshop items (shapes/links/rules/layouts)</td>
      <td>—</td>
      <td>Implementation in progress `[~]`; no automated coverage yet</td>
      <td>[~]</td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-enable MD033 -->

## Summary

- CRUD and access-management stories now map to explicit backend and frontend checks, making coverage gaps (validation negatives, invited-user UI flows) easier to spot at a glance.
- Workshop-related capabilities remain uncovered because the functionality is not yet implemented; these rows stay as placeholders for future tests.
- Session authentication is covered for the happy path across backend and UI, but failure-mode tests are still outstanding.
