//! Package service module: business logic for managing packages.
//!
//! Responsibilities:
//! - Validate and transform API payloads to storage models
//! - Enforce access control (e.g., `owner`/`manage` for update/delete)
//! - Convert storage models to API responses
//! - Delegate persistence to the storage layer (`PackageStore`)
//!
//! Prerequisites:
//! - An instance of `PackageStore` to interact with the database
//!
//! Example:
//! ```ignore
//! use std::sync::Arc;
//! use crate::package::package_store::PackageStore;
//! use crate::package::package_service::{PackageService, NewPackageItem};
//! use crate::session::session_service::SessionData;
//!
//! async fn example(pool: Arc<sqlx::Pool<sqlx::Sqlite>>, session: SessionData) {
//!     let store = PackageStore::new(pool);
//!     let service = PackageService::new(store);
//!
//!     // create
//!     let new_item = NewPackageItem { name: "Demo".into(), description: None, icon: None, is_public: Some(true) };
//!     let created = service.add_item(&new_item, &session).await.unwrap();
//!
//!     // list (visible to session)
//!     let visible = service.get_items(&Some(session)).await.unwrap();
//!     assert!(visible.iter().any(|p| p.id == created.id));
//! }
//! ```
//
use crate::error::ModelError;
use crate::package::package_store::{NewPackageDb, PackageAccessRoleDb, PackageDb, PackageStore, PatchPackageDb};
use crate::session::session_service::SessionData;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::{api::deserialize::deserialize_some};

/// API representation of a package.
///
/// This is the structure returned to clients by HTTP handlers and used
/// throughout the service layer. It is converted from the storage model (`PackageDb`).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    /// Unique identifier of the package.
    pub id: i32,
    /// Human-readable name of the package.
    pub name: String,
    /// Optional description shown in listings and details.
    pub description: Option<String>,
    /// Optional icon URL or identifier.
    pub icon: Option<String>,
    /// Whether the package is publicly visible.
    pub is_public: bool,
}

/// Payload to create a new package.
///
/// Validation:
/// - `name`: 3..=100 chars
/// - `description`: up to 500 chars
/// - `icon`: up to 255 chars
/// - `is_public`: defaults to `false` if omitted
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct NewPackageItem {
    #[validate(length(min = 3, max = 100))]
    /// Package name (3..=100 chars).
    pub name: String,
    #[validate(length(max = 500))]
    /// Optional description (<= 500 chars).
    pub description: Option<String>,
    #[validate(length(max = 255))]
    /// Optional icon URL or identifier (<= 255 chars).
    pub icon: Option<String>,
    /// Public visibility flag. Defaults to `false` when omitted.
    pub is_public: Option<bool>,
}

/// Payload to partially update an existing package.
///
/// Semantics for optional nested options:
/// - `description`: `None` -> not provided; `Some(None)` -> set to NULL; `Some(Some(v))` -> set to value
/// - `icon`: same as `description`
///
/// Examples:
/// - Update name only: `{ "name": "New Name" }`
/// - Clear description: `{ "description": null }`
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct PatchPackageItem {
    #[validate(length(min = 3, max = 100))]
    /// Optional new name (3..=100 chars).
    pub name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_some")]
    #[validate(length(max = 500))]
    /// Optional new description; `null` clears it.
    pub description: Option<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_some")]
    #[validate(length(max = 255))]
    /// Optional new icon; `null` clears it.
    pub icon: Option<Option<String>>,
    /// Optional new public flag.
    pub is_public: Option<bool>,
}

/// Conversion: `&NewPackageItem` -> storage model `NewPackageDb`.
///
/// Applies default for `is_public` when omitted.
impl<'a> From<&'a NewPackageItem> for NewPackageDb<'a> {
    fn from(item: &'a NewPackageItem) -> Self {
        Self {
            name: &item.name,
            description: item.description.as_deref(),
            icon: item.icon.as_deref(),
            is_public: item.is_public.unwrap_or(false),
        }
    }
}

/// Conversion: `&PatchPackageItem` -> storage model `PatchPackageDb`.
///
/// Nested optional fields map to `Option<Option<&str>>` to support clearing values.
impl<'a> From<&'a PatchPackageItem> for PatchPackageDb<'a> {
    fn from(item: &'a PatchPackageItem) -> Self {
        Self {
            name: item.name.as_deref(),
            description: item.description.as_ref().map(|d| d.as_deref()),
            icon: item.icon.as_ref().map(|i| i.as_deref()),
            is_public: item.is_public,
        }
    }
}

/// Conversion: storage model `PackageDb` -> API model `Package`.
impl From<PackageDb> for Package {
    fn from(item: PackageDb) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon: item.icon,
            is_public: item.is_public,
        }
    }
}

/// Access roles applicable to a package.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PackageAccess {
    /// Full control; can update/delete and manage roles.
    Owner,
    /// Management rights; can update/delete.
    Manage,
    /// Read-only access.
    Read,
}

impl From<String> for PackageAccess {
    fn from(role: String) -> Self {
        match role.as_str() {
            "owner" => PackageAccess::Owner,
            "manage" => PackageAccess::Manage,
            "read" => PackageAccess::Read,
            _ => panic!("Unknown package access role: {}", role),
        }
    }
}

impl Into<String> for PackageAccess {
    fn into(self) -> String {
        match self {
            PackageAccess::Owner => "owner".to_string(),
            PackageAccess::Manage => "manage".to_string(),
            PackageAccess::Read => "read".to_string(),
        }
    }
}

/// Wrapper returned by service when reporting a role for a package.
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct PackageAccessRole {
    /// The effective role granted for a package.
    pub role: PackageAccess,
}

impl From<PackageAccessRoleDb> for PackageAccessRole {
    fn from(item: PackageAccessRoleDb) -> Self {
        Self {
            role: item.role.into(),
        }
    }
}


/// Service layer encapsulating business logic for packages.
///
/// Responsibilities:
/// - Validation and conversion between API payloads and storage models
/// - Access control decisions that require knowing the caller's session
/// - Delegating persistence to `PackageStore`
pub struct PackageService {
    items_store: PackageStore,
}

impl<'a> PackageService {
    /// Create a new service with the provided store.
    pub fn new(items_store: PackageStore) -> Self {
        Self {
            items_store: items_store,
        }
    }

    /// Create a new package for the authenticated user.
    /// - Persists a new package via the store
    /// - Grants the caller the `owner` role
    pub async fn add_item(&self, item: &'a NewPackageItem, session: &SessionData) -> Result<Package, ModelError> {
        let result = self.items_store.add_item(item.into(), session).await?;

        Ok(result.into())
    }

    /// List packages visible to the optional session.
    /// - When `session` is `None`, returns only public packages
    /// - Otherwise, returns public plus accessible packages
    pub async fn get_items(&self, session: &Option<SessionData>) -> Result<Vec<Package>, ModelError> {
        let items = self.items_store.get_items(session).await?;

        Ok(items.into_iter().map(|item| item.into()).collect())
    }

    /// Get a single package by id if visible to the session.
    pub async fn get_item(&self, item_id: i32, session: &Option<SessionData>) -> Result<Package, ModelError> {
        let item = self.items_store.get_item(item_id, session).await?;

        Ok(item.into())
    }

    /// Update a package.
    /// - Verifies the caller has `owner` or `manage` role
    /// - Applies partial updates via the store
    pub async fn update_item(&self, id: i32, item: &'a PatchPackageItem, session: &SessionData) -> Result<Package, ModelError> {
        // access check moved to service: require owner or manage
        let roles = self.items_store.get_access_roles(id, session).await?;
        if !roles.iter().any(|r| r.role == "owner" || r.role == "manage") {
            return Err(ModelError::Forbidden("You are not allowed to update this package".to_string()));
        }

        let updated_item = self.items_store.update_item(id, item.into()).await?;

        Ok(updated_item.into())
    }

    /// Delete a package.
    /// - Verifies the caller has `owner` or `manage` role
    /// - Deletes the package (and related access rows in store)
    pub async fn delete_item(&self, id: i32, session: &SessionData) -> Result<Package, ModelError> {
        let roles = self.items_store.get_access_roles(id, session).await?;

        if !roles.iter().any(|r| r.role == "owner" || r.role == "manage") {
            return Err(ModelError::Forbidden("You are not allowed to delete this package".to_string()));
        }

        let result = self.items_store.delete_item(id).await?;

        Ok(result.into())
    }

}
