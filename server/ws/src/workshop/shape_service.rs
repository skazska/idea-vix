//! Workshop shape service module: business logic for managing shapes.
//!
//! Responsibilities:
//! - Validate and transform API payloads to storage models
//! - Enforce access control for shape operations
//! - Convert storage models to API responses
//! - Delegate persistence to the storage layer (`ShapeStore`)

use std::sync::Arc;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::common::crud::{CrudQueries, CrudService, ListParams, QueryFilter, QueryLister};
use crate::db::{ TransactionStarter };
use crate::error::ModelError;
use crate::session::session_service::SessionData;
use crate::workshop::shape_store::{NewShapeDb, PatchShapeDb, ShapeDb, ShapeStore};

lazy_static! {
    static ref SLUG_REGEX: Regex = Regex::new(r"^[a-z][a-z0-9-]*$").unwrap();
}

/// API representation of a shape.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Shape {
    pub id: i64,
    pub name: String,
    pub slug: String, // semantic identifier for text-based references
    pub description: Option<String>,
    pub definition: serde_json::Value,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Request model for creating a new shape.
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct NewShapeItem {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    #[validate(length(min = 3, max = 100))]
    #[validate(regex(path = "*SLUG_REGEX"))]
    pub slug: String,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    pub definition: serde_json::Value,
}

/// Request model for updating an existing shape.
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct PatchShapeItem {
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,
    #[validate(length(max = 500))]
    pub description: Option<Option<String>>,
    pub definition: Option<serde_json::Value>,
}

impl<'a> From<&'a NewShapeItem> for NewShapeDb<'a> {
    fn from(item: &'a NewShapeItem) -> Self {
        Self {
            name: &item.name,
            slug: &item.slug,
            description: item.description.as_deref(),
            definition: &item.definition,
        }
    }
}

impl<'a> From<&'a PatchShapeItem> for PatchShapeDb<'a> {
    fn from(item: &'a PatchShapeItem) -> Self {
        Self {
            name: item.name.as_deref(),
            description: item.description.as_ref().map(|opt| opt.as_deref()),
            definition: item.definition.as_ref(),
        }
    }
}

impl From<ShapeDb> for Shape {
    fn from(db: ShapeDb) -> Self {
        Self {
            id: db.id,
            name: db.name,
            slug: db.slug,
            description: db.description,
            definition: db.definition,
            created_at: db.created_at.try_into().unwrap_or(0),
            updated_at: db.updated_at.try_into().unwrap_or(0),
        }
    }
}

/// Lister for shapes
pub type ShapeLister = ListParams;

/// Service layer encapsulating business logic for workshop shapes.
///
/// Responsibilities:
/// - Validation and conversion between API payloads and storage models
/// - Access control decisions that require knowing the caller's session
/// - Delegating persistence to `ShapeStore`
pub struct ShapeService {
    transaction_starter: Arc<TransactionStarter>,
    items_store: Arc<ShapeStore>,
}

impl ShapeService {
    /// Create a new service with the provided dependencies.
    pub fn new(transaction_starter: Arc<TransactionStarter>, items_store: Arc<ShapeStore>) -> Self {
        Self {
            transaction_starter,
            items_store,
        }
    }

    /// Get a shape by its unique slug.
    pub async fn get_shape_by_slug<'r>(&'r self, slug: &'r str) -> Result<Shape, ModelError> {
        let result = self.transaction_starter.run_in_transaction(async move |transaction| {
            // Create a lister to find shape by slug using search functionality
            let lister = QueryLister {
                filter: Some(QueryFilter {
                    access: None,
                    filter: None,
                    ids: None,
                    search: Some(slug), // Assuming slug search works
                }),
                pager: crate::common::crud::QueryPager { limit: 1, offset: None },
            };
            
            let items = self.items_store.get_items(&lister, transaction).await?;
            let item = items.into_iter().find(|s| s.slug == slug)
                .ok_or(ModelError::NotFound("Shape not found".to_string()))?;

            Ok(item)
        }).await?;

        Ok(Shape::from(result))
    }

}

impl CrudService for ShapeService {
    type Item = Shape;
    type NewItem = NewShapeItem;
    type PatchItem = PatchShapeItem;
    type SessionData = SessionData;
    type Lister = ShapeLister;
    type Id = i64;

    /// Create a new shape for the authenticated user.
    async fn add_item<'r>(&'r self, item: &'r mut Self::NewItem, _session: &'r Self::SessionData) -> Result<Self::Item, ModelError> {
        let db_item = NewShapeDb::from(&*item);

        let result = self.transaction_starter.run_in_transaction(async move |trx| {
            let item = self.items_store.add_item(&db_item, trx).await?;
            Ok(item)
        }).await?;

        Ok(Self::Item::from(result))
    }


    /// List shapes based on the provided lister parameters.
    async fn get_items<'r>(&'r self, lister: &'r Self::Lister, _session: Option<&'r Self::SessionData>) -> Result<Vec<Self::Item>, ModelError> {
        let lister = QueryLister::from(lister);

        let result = self.transaction_starter.run_in_transaction(async move |transaction| {
            let items = self.items_store.get_items(&lister, transaction).await?;
            Ok(items)
        }).await?;

        Ok(result.into_iter().map(|item| Self::Item::from(item)).collect())
    }

    /// Get a single shape by id.
    async fn get_item<'r>(&'r self, item_id: Self::Id, _session: Option<&'r Self::SessionData>) -> Result<Self::Item, ModelError> {
        let result = self.transaction_starter.run_in_transaction(async move |transaction| {
            let item = self.items_store.get_item(item_id, None, transaction).await?;
            Ok(item)
        }).await?;

        Ok(Self::Item::from(result))
    }

    /// Update a shape.
    async fn update_item<'r>(&'r self, id: Self::Id, item: &'r Self::PatchItem, _session: &'r Self::SessionData) -> Result<Self::Item, ModelError> {
        let db_item = PatchShapeDb::from(item);

        let result = self.transaction_starter.run_in_transaction(async move |transaction| {
            let updated_item = self.items_store.update_item(id, &db_item, transaction).await?;
            Ok(updated_item)
        }).await?;

        Ok(Self::Item::from(result))
    }

    /// Delete a shape.
    async fn delete_item<'r>(&'r self, id: Self::Id, _session: &'r Self::SessionData) -> Result<Self::Item, ModelError> {
        let result = self.transaction_starter.run_in_transaction(async move |transaction| {
            let deleted_item = self.items_store.delete_item(id, transaction).await?;
            Ok(deleted_item)
        }).await?;

        Ok(Self::Item::from(result))
    }
}
