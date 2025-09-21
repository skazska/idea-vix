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
use std::sync::Arc;

use crate::common::access::{ ItemAccessQueries, ItemRoleDb, SqliteItemAccessQueries, ROLE_OWNER };
use crate::common::crud::{CrudQueries, CrudService, ListParams, QueryFilter, QueryLister};
use crate::common::workshop::shape_service::{Shape, ShapeService};
use crate::db::{ TrxRun, TransactionStarter };
use crate::error::ModelError;
use crate::package::package_store::{NewPackageDb, PackageDb, PackageStore, PatchPackageDb};
use crate::session::session_service::SessionData;
use serde::{Deserialize, Serialize};
use validator::Validate;
pub use crate::common::access::ItemAccessGrantDto as NewPackageAccessItem;

/// Package view
#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    /// Unique identifier of the package.
    pub id: i64,
    /// Human-readable name of the package.
    pub name: String,
    /// Optional description shown in listings and details.
    pub description: Option<String>,
    /// Optional icon URL or identifier.
    pub icon: Option<String>,
    /// Whether the package is publicly visible.
    pub is_public: bool,
}

/// New Package DTO
/// exteds base entity new DTO
/// has validations
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct NewPackageItem {
    #[serde(flatten)]
    #[validate(nested)]
    pub base: crate::common::item_fields::NewItemFields,
}

/// Patch Package DTO
/// extends base entity patch DTO
/// has validations
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct PatchPackageItem {
    #[serde(flatten)]
    #[validate(nested)]
    pub base: crate::common::item_fields::PatchItemFields,
}

/// Conversion of NewPackageItem into NewPackageDb
/// applies default for `is_public` when omitted.
impl<'d> From<&'d NewPackageItem> for NewPackageDb<'d> {
    fn from(item: &'d NewPackageItem) -> Self {
        Self {
            name: &item.base.name,
            description: item.base.description.as_deref(),
            icon: item.base.icon.as_deref(),
            is_public: item.base.is_public.unwrap_or(false),
        }
    }
}

/// Conversion of PatchPackageItem into PatchPackageDb
impl<'d> From<&'d PatchPackageItem> for PatchPackageDb<'d> {
    fn from(item: &'d PatchPackageItem) -> Self {
        Self {
            name: item.base.name.as_deref(),
            description: item.base.description.as_ref().map(|d| d.as_deref()),
            icon: item.base.icon.as_ref().map(|i| i.as_deref()),
            is_public: item.base.is_public,
        }
    }
}

/// Conversion of PackageDb into Package
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

/// Lister for packages
pub type PackageLister = ListParams;

/// Service layer encapsulating business logic for packages.
///
/// Responsibilities:
/// - Validation and conversion between API payloads and storage models
/// - Access control decisions that require knowing the caller's session
/// - Delegating persistence to `PackageStore`
pub struct PackageService {
    transaction_starter: Arc<TransactionStarter>,
    items_store: Arc<PackageStore>,
    shapes_service: Arc<ShapeService>,
    access_store: Arc<SqliteItemAccessQueries>,
}

impl PackageService {
    /// Create a new service with the provided store.
    pub fn new(transaction_starter: Arc<TransactionStarter>, items_store: Arc<PackageStore>, shapes_service: Arc<ShapeService>, access_store: Arc<SqliteItemAccessQueries>) -> Self {
        Self {
            transaction_starter,
            items_store,
            shapes_service,
            access_store,
        }
    }
}

impl CrudService for PackageService {
    type Item = Package;
    type NewItem = NewPackageItem;
    type PatchItem = PatchPackageItem;
    type SessionData = SessionData;
    type Lister = PackageLister;
    type Id = i64;

    /// Create a new package for the authenticated user.
    /// - Persists a new package via the store
    /// - Grants the caller the `owner` role
    async fn add_item<'r>(&'r self, item: &'r Self::NewItem, session: &'r Self::SessionData) -> Result<Self::Item, ModelError> {

        let db_item = NewPackageDb::from(item);

        let result = self.transaction_starter.run_in_transaction(async move |trx| {
            let item = self.items_store.add_item(&db_item, trx).await?;
            let role = ItemRoleDb { address: session.address.clone(), role: ROLE_OWNER.to_string() };
            self.access_store.add_access_role(item.id, &role, trx).await?;

            Ok(item)
        }).await?;

        Ok(Self::Item::from(result))

    }

    /// List packages visible to the optional session.
    /// - When `session` is `None`, returns only public packages
    /// - Otherwise, returns public plus accessible packages
    async fn get_items(&self, lister: &Self::Lister, session: Option<&Self::SessionData>) -> Result<Vec<Self::Item>, ModelError> {
        let mut lister = QueryLister::from(lister);
        if let Some(sess) = session {
            if lister.filter.is_none() {
                lister.filter = Some(QueryFilter {
                    access: Some(sess.address.as_str()),
                    filter: None,
                    ids: None,
                    search: None,
                });
            } else if let Some(f) = &mut lister.filter {
                f.access = Some(sess.address.as_str());
            }
        }

        let result = self.transaction_starter.run_in_transaction(async move |trx| {
            let items = self.items_store.get_items(&lister, trx).await?;

            Ok(items)
        }).await?;

        Ok(result.into_iter().map(|item| Self::Item::from(item)).collect())
    }

    /// Get a single package by id if visible to the session.
    async fn get_item(&self, item_id: Self::Id, session: Option<&Self::SessionData>) -> Result<Self::Item, ModelError> {
        let access = session.map(|s| s.address.as_str());

        let result = self.transaction_starter.run_in_transaction(async move |trx| {
            let item = self.items_store.get_item(item_id, access, trx).await?;

            Ok(item)
        }).await?;

        Ok(Self::Item::from(result))
    }

    /// Update a package.
    /// - Verifies the caller has `owner` or `manage` role
    /// - Applies partial updates via the store
    async fn update_item(&self, id: Self::Id, item: &Self::PatchItem, session: &Self::SessionData) -> Result<Self::Item, ModelError> {
        let db_item = PatchPackageDb::from(item);

        let result = self.transaction_starter.run_in_transaction(async move |trx| {
            let roles = self.access_store.get_access_roles(id, session, Some(&Vec::from([ROLE_OWNER])), trx).await?;

            if roles.len() == 0 {
                return Err(ModelError::Forbidden("You are not allowed to update this package".to_string()));
            }

            let updated_item = self.items_store.update_item(id, &db_item, trx).await?;

            Ok(updated_item)
        }).await?;

        Ok(Self::Item::from(result))
    }

    /// Delete a package.
    /// - Verifies the caller has `owner` or `manage` role
    /// - Deletes the package (and related access rows in store)
    async fn delete_item(&self, id: Self::Id, session: &Self::SessionData) -> Result<Self::Item, ModelError> {
        let result = self.transaction_starter.run_in_transaction(async move |trx| {
            let roles = self.access_store.get_access_roles(id, session, Some(&Vec::from([ROLE_OWNER])), trx).await?;

            if roles.len() == 0 {
                return Err(ModelError::Forbidden("You are not allowed to delete this package".to_string()));
            }

            let deleted_item = self.items_store.delete_item(id, trx).await?;

            Ok(deleted_item)
        }).await?;

        Ok(Self::Item::from(result))
    }

    // /// Returns package shapes
    // async fn list_shapes(&self, package_id: i32, session: &SessionData) -> Result<Vec<Shape>, ModelError> {
    //     let roles = self.items_store.get_access_roles(package_id, session).await?;
    //     if !has_owner_or_manage(roles.iter().map(|r| r.role.as_str())) {
    //         return Err(ModelError::Forbidden("You are not allowed to view package shapes".to_string()));
    //     }

    //     let rows = self.items_store.list_shapes(package_id).await?;
    //     Ok(rows.into_iter().map(|r| r.into()).collect())
    // }
}

