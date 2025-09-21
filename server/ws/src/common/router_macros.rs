//! Router wiring macros for common REST resource shapes.
//!
//! Provides a small macro to assemble a standard set of routes for
//! CRUD + access-management endpoints used by multiple features.

/// Build a Router with standard REST routes for an entity supporting access-management.
///
/// Routes included (relative to mount point):
/// - GET     "/"                   => $get_items
/// - POST    "/"                   => $add_item
/// - GET     "/{id}"               => $get_item
/// - PUT     "/{id}"               => $update_item
/// - DELETE  "/{id}"               => $delete_item
/// - POST    "/{id}/access"        => $add_access
/// - GET     "/{id}/access"        => $list_access
/// - DELETE  "/{id}/access/{address}" => $revoke_access
///
/// Usage:
/// ```ignore
/// Router::new()
///   .merge(crate::resource_routes!(get_items, add_item, get_item, update_item, delete_item, add_access, list_access, revoke_access))
///   .with_state(state)
/// ```
#[macro_export]
macro_rules! resource_routes {
    (
        $get_items:expr,
        $add_item:expr,
        $get_item:expr,
        $update_item:expr,
        $delete_item:expr,
        $add_access:expr,
        $list_access:expr,
        $revoke_access:expr,
        $check_access:expr
    ) => {{
        ::axum::Router::new()
            .route("/", ::axum::routing::get($get_items))
            .route("/", ::axum::routing::post($add_item))
            .route("/{id}/access", ::axum::routing::post($add_access))
            .route("/{id}/access", ::axum::routing::get($list_access))
            .route("/{id}/access/{address}", ::axum::routing::delete($revoke_access))
            .route("/{id}/my/access", ::axum::routing::get($check_access))
            .route("/{id}", ::axum::routing::get($get_item))
            .route("/{id}", ::axum::routing::put($update_item))
            .route("/{id}", ::axum::routing::delete($delete_item))
    }};
}
