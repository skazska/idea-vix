# Backend Commons Refactor Plan

Purpose: Consolidate duplicated types and utilities across feature modules (boards, packages) into shared modules while keeping features decoupled.

## Status Checklist

- [x] Shared access types/utilities
  - Module: `common::access`
  - Includes: `Role`, `RoleOnly`, `GrantRequest`, `validate_grant_role`
  - Refactored: `boards::board_service`, `package::package_service`
- [x] Shared item payload types (Done)
  - Module: `common::item_fields`
  - Includes: `NewItemFields`, `PatchItemFields` with validations and `deserialize_some`
  - Refactor wrappers: `boards::NewBoardItem/PatchBoardItem`, `package::NewPackageItem/PatchPackageItem` wrap `base: NewItemFields/PatchItemFields`
  - Conversions to DB insert/patch types updated to use wrapper `base`
- [x] Common access guard helpers (Done)
  - Module: `common::service_access`
  - Helpers: `has_owner`, `has_owner_or_manage`, `ensure_not_self_revoke`
  - Used in board/package services to replace repeated guards
- [x] SQL update builder macro/helper (Done)
  - Module: `common::sqlx_patch`
  - Macro: `sqlx_build_set!` to compose dynamic SET clauses
  - Used in: `boards::board_store::update_item`, `package::package_store::update_item`
- [x] API router macros (Optional, now Done)
  - Module: `common::router_macros`
  - Macro: `resource_routes!` to generate CRUD + access routes (used by boards and package routers)
  - Benefit: removes duplicate route wiring and keeps handler signatures unchanged

## Notes

- Keep feature-specific DB models and stores within their modules; commons only covers shapes and logic that are identical or entity-agnostic.
- Prefer type aliases in features (e.g., `type NewBoardItem = common::item_fields::NewItemFields`) to preserve naming in feature code while sharing definitions.

### Wrapper pattern for feature-specific fields

Rust has no inheritance; when a feature needs extra fields on top of the shared payloads, wrap the common type and flatten it:

```rust
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, validator::Validate)]
pub struct NewBoardItem {
  #[serde(flatten)]
  #[validate(nested)]
  pub base: common::item_fields::NewItemFields,
  // add per-entity fields here, e.g.:
  // #[validate(length(max = 20))]
  // pub color: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, validator::Validate)]
pub struct PatchBoardItem {
  #[serde(flatten)]
  #[validate(nested)]
  pub base: common::item_fields::PatchItemFields,
  // per-entity patch field example with nullable semantics:
  // #[serde(default, deserialize_with = "api::deserialize::deserialize_some")]
  // pub color: Option<Option<String>>,
}
```

Keep DB conversion impls using `item.base` for shared columns and add specific column handling as needed.
