# Backend Commons Refactor Plan

Purpose: Consolidate duplicated types and utilities across feature modules (boards, packages) into shared modules while keeping features decoupled.

## Status Checklist

- [x] Shared access types/utilities
  - Module: `common::access`
  - Includes: `Role`, `RoleOnly`, `GrantRequest`, `validate_grant_role`
  - Refactored: `boards::board_service`, `package::package_service`
- [ ] Shared item payload types (In Progress)
  - Module: `common::item_fields`
  - Includes: `NewItemFields`, `PatchItemFields` with validations and `deserialize_some`
  - Refactor aliases: `NewBoardItem`, `PatchBoardItem`, `NewPackageItem`, `PatchPackageItem`
  - Update conversions to DB patch/insert types in stores
- [ ] Common access guard helpers (Proposed)
  - Module: `common::service_access`
  - Helpers: `has_owner`, `has_owner_or_manage`, `ensure_not_self_revoke`
  - Use in board/package services to replace repeated guards
- [ ] SQL update builder macro/helper (Optional)
  - Module: `common::sqlx_patch`
  - Macro or helper to compose SET clauses and bind optional fields
- [ ] API router macros (Optional)
  - Module: `api::router_macros`
  - Macro to generate CRUD + access routes with pluggable types/service methods

## Notes

- Keep feature-specific DB models and stores within their modules; commons only covers shapes and logic that are identical or entity-agnostic.
- Prefer type aliases in features (e.g., `type NewBoardItem = common::item_fields::NewItemFields`) to preserve naming in feature code while sharing definitions.
