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
///
/// Usage:
/// ```ignore
/// Router::new()
///   .merge(crate::resource_routes!(get_items, add_item, get_item, update_item, delete_item))
///   .with_state(state)
/// ```
#[macro_export]
macro_rules! resource_router {
    (
        $get_items:expr,
        $add_item:expr,
        $get_item:expr,
        $update_item:expr,
        $delete_item:expr,
        $state:expr
    ) => {{
        ::axum::Router::new()
            .route("/{id}", ::axum::routing::get($get_item).put($update_item).delete($delete_item))
            .route("/", ::axum::routing::get($get_items).post($add_item))
            .with_state($state)
    }};
}

/// Build a Router with standard access-management routes for an entity.
///
/// Routes included (relative to mount point):
/// - POST    "/access"             => $add_access
/// - GET     "/access"             => $list_access
/// - GET     "/access/my"          => $check_access
/// - DELETE  "/access/{address}"   => $revoke_access
///
/// Usage:
/// ```ignore
/// Router::new()
///   .nest("/1", crate::access_router!(add_access, list_access, revoke_access, check_access))
///   .with_state(state)
/// ```
#[macro_export]
macro_rules! access_router {
    (
        $add_access:expr,
        $list_access:expr,
        $revoke_access:expr,
        $check_access:expr,
        $state:expr
    ) => {{
        ::axum::Router::new()
            .route("/access/my", ::axum::routing::get($check_access))
            .route("/access/{address}", ::axum::routing::delete($revoke_access))
            .route("/access", ::axum::routing::post($add_access).get($list_access))
            .with_state($state)
        }};
}

/// Build a Router with standard workshop-item of given type routes for an entity.
///
/// Routes included (relative to mount point):
/// - GET     "workshop_type/"                   => $get_workshop_items
/// - POST    "workshop_type/"                   => $add_workshop_item
/// - PUT     "workshop_type/{id}"               => $update_workshop_item
/// - DELETE  "workshop_type/{id}"               => $delete_workshop_item
///
/// Usage:
/// ```ignore
/// Router::new()
///   .nest("/1", crate::workshop_item_router!("workshop_type", get_items, add_item, update_item, delete_item))
///   .with_state(state)
/// ```
#[macro_export]
macro_rules! workshop_item_router {
    (
        $get_workshop_items:expr,
        $add_workshop_item:expr,
        $update_workshop_item:expr,
        $delete_workshop_item:expr,
        $workshop_state:expr
    ) => {{
        ::axum::Router::new()
            .route("/{workshop_item_id}", ::axum::routing::put($update_workshop_item).delete($delete_workshop_item))
            .route("/", ::axum::routing::get($get_workshop_items).post($add_workshop_item))
            .with_state($workshop_state)
    }};
}


#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{extract::{Path, State}, response::{self, IntoResponse}, Json};
    use tower::util::ServiceExt;
    use serde::Serialize;

    #[derive(Debug, Clone, Serialize)]
    struct TestItem {
        id: i64,
        name: String,
    }

    #[derive(Debug, Clone)]
    struct CommonState {
        common: String,
    }

    #[derive(Debug, Clone)]
    struct CrudState {
        crud: String,
    }

    #[derive(Debug, Clone)]
    struct AccessState {
        access: String,
    }

    #[tokio::test]
    async fn test_resource_routes() {
        let common_state: Arc<CommonState> = Arc::new(CommonState {
            common: "common".to_string(),
        });

        let crud_state: Arc<CrudState> = Arc::new(CrudState {
            crud: "crud".to_string(),
        });

        let access_state: Arc<AccessState> = Arc::new(AccessState {
            access: "access".to_string(),
        });

        let router = resource_router!(
            get_items,
            add_item,
            get_item,
            update_item,
            delete_item,
            Arc::new((common_state.clone(), crud_state.clone()))
        )
        .nest("/1", access_router!(
            add_access,
            list_access,
            revoke_access,
            check_access,
            Arc::new((common_state.clone(), access_state.clone()))
        ));

        let response = router.clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/")
                    .method(axum::http::Method::GET)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), 200);

        let response = router.clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/1")
                    .method(axum::http::Method::GET)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), 200);

        let response = router.clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/1")
                    .method(axum::http::Method::PUT)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), 200);

        let response = router.clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/1")
                    .method(axum::http::Method::DELETE)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), 200);

        let response = router.clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/1/access")
                    .method(axum::http::Method::POST)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), 200);
        let response = router.clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/1/access")
                    .method(axum::http::Method::GET)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), 200);
        let response = router.clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/1/access/user1")
                    .method(axum::http::Method::DELETE)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), 200);
        let response = router.clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/1/access/user2")
                    .method(axum::http::Method::DELETE)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), 404);

        let response = router.clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/1/access/my")
                    .method(axum::http::Method::GET)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), 200);

    }

    async fn get_items(State(state): State<Arc<(Arc<CommonState>, Arc<CrudState>)>>) -> impl IntoResponse {
        let (common_state, crud_state) = state.as_ref();
        Json(vec![
            TestItem {
                id: 1,
                name: common_state.common.clone(),
            },
            TestItem {
                id: 2,
                name: crud_state.crud.clone(),
            },
        ])
    }
    async fn add_item() -> impl IntoResponse {
        Json(TestItem {
            id: 3,
            name: "Item 3".to_string(),
        })
    }
    async fn get_item() -> impl IntoResponse {
        Json(TestItem {
            id: 1,
            name: "Item 1".to_string(),
        })
    }
    async fn update_item() -> impl IntoResponse {
        Json(TestItem {
            id: 1,
            name: "Updated Item 1".to_string(),
        })
    }
    async fn delete_item() -> impl IntoResponse {
        Json("Item deleted")
    }
    async fn add_access() -> impl IntoResponse {
        Json("Access added")
    }
    async fn list_access(State(state): State<Arc<(Arc<CommonState>, Arc<AccessState>)>>) -> impl IntoResponse {
        let (common_state, access_state) = state.as_ref();
        Json(vec![common_state.common.clone(), access_state.access.clone()])
    }
    async fn revoke_access(path: Path<String>) -> impl IntoResponse {
        let path: &str = path.as_ref();
        if path != "user1" {
            return response::Response::builder()
                .status(404)
                .body(axum::body::Body::from("User not found"))
                .unwrap()
                .into_response();
        }

        response::Response::builder()
            .status(200)
            .body(axum::body::Body::from(format!("Access revoked for {}", path)))
            .unwrap()
            .into_response()
    }
    async fn check_access() -> impl IntoResponse {
        Json("Access granted")
    }
}