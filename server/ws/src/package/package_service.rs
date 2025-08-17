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
use crate::package::package_store::{NewPackageAccessDb, NewPackageDb, PackageAccessRoleDb, PackageAccessRolesDb, PackageDb, PackageStore, PatchPackageDb};
use crate::session::session_service::SessionData;
use serde::{Deserialize, Serialize};
use validator::Validate;

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

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct NewPackageItem {
    #[serde(flatten)]
    #[validate(nested)]
    pub base: crate::common::item_fields::NewItemFields,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct PatchPackageItem {
    #[serde(flatten)]
    #[validate(nested)]
    pub base: crate::common::item_fields::PatchItemFields,
}

/// Conversion: `&NewPackageItem` -> storage model `NewPackageDb`.
///
/// Applies default for `is_public` when omitted.
impl<'a> From<&'a NewPackageItem> for NewPackageDb<'a> {
    fn from(item: &'a NewPackageItem) -> Self {
        Self {
            name: &item.base.name,
            description: item.base.description.as_deref(),
            icon: item.base.icon.as_deref(),
            is_public: item.base.is_public.unwrap_or(false),
        }
    }
}

/// Conversion: `&PatchPackageItem` -> storage model `PatchPackageDb`.
///
/// Nested optional fields map to `Option<Option<&str>>` to support clearing values.
impl<'a> From<&'a PatchPackageItem> for PatchPackageDb<'a> {
    fn from(item: &'a PatchPackageItem) -> Self {
        Self {
            name: item.base.name.as_deref(),
            description: item.base.description.as_ref().map(|d| d.as_deref()),
            icon: item.base.icon.as_ref().map(|i| i.as_deref()),
            is_public: item.base.is_public,
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

pub use crate::common::access::RoleOnly as PackageAccessRole;
pub use crate::common::access::GrantRequest as NewPackageAccessItem;
use crate::common::access::validate_grant_role;


/// Service layer encapsulating business logic for packages.
///
/// Responsibilities:
/// - Validation and conversion between API payloads and storage models
/// - Access control decisions that require knowing the caller's session
/// - Delegating persistence to `PackageStore`
pub struct PackageService {
    items_store: PackageStore,
}

// NewPackageAccessItem is re-exported from common::access::GrantRequest

/// Response model for a package access mapping.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageAccessRoles {
    pub package_id: i32,
    pub address: String,
    pub role: String,
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

    /// Grant access role to an address (owner only)
    pub async fn add_access_role(&self, package_id: i32, item: &NewPackageAccessItem, session: &SessionData) -> Result<PackageAccessRoles, ModelError> {
        // Only owner can manage access list
        let roles = self.items_store.get_access_roles(package_id, session).await?;
        if !roles.iter().any(|r| r.role == "owner") {
            return Err(ModelError::Forbidden("Only owner can grant access".to_string()));
        }

        // validate role value
    if validate_grant_role(&item.role).is_err() {
            return Err(ModelError::BadRequest("Invalid role".to_string()));
        }

        // Build store model and insert
        let db_item = NewPackageAccessDb { package_id, address: &item.address, role: &item.role };
        let stored = self.items_store.add_access_role(db_item).await?;
        Ok(stored.into())
    }

    /// Revoke access for an address (owner only)
    pub async fn revoke_access_role(&self, package_id: i32, address: &str, session: &SessionData) -> Result<PackageAccessRoles, ModelError> {
        let roles = self.items_store.get_access_roles(package_id, session).await?;
        if !roles.iter().any(|r| r.role == "owner") {
            return Err(ModelError::Forbidden("Only owner can revoke access".to_string()));
        }
        // Prevent owner from revoking their own owner role via this endpoint
        if address == session.address {
            return Err(ModelError::BadRequest("Cannot revoke own access".to_string()));
        }

        let stored = self.items_store.revoke_access_role(package_id, address).await?;
        Ok(stored.into())
    }

    /// List access mappings for a package (owner only)
    pub async fn list_access_roles(&self, package_id: i32, session: &SessionData) -> Result<Vec<PackageAccessRoles>, ModelError> {
        // Only owner can view access list
        let roles = self.items_store.get_access_roles(package_id, session).await?;
        if !roles.iter().any(|r| r.role == "owner") {
            return Err(ModelError::Forbidden("Only owner can list access".to_string()));
        }

        let rows = self.items_store.list_access_roles(package_id).await?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

// --- helpers and conversions ---

impl From<PackageAccessRolesDb> for PackageAccessRoles {
    fn from(value: PackageAccessRolesDb) -> Self {
        Self { package_id: value.package_id, address: value.address, role: value.role }
    }
}

impl From<PackageAccessRoleDb> for PackageAccessRole {
    fn from(item: PackageAccessRoleDb) -> Self {
        Self { role: item.role.into() }
    }
}
