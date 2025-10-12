//! Generic workshop service module: business logic for managing all workshop item types.
//!
//! Responsibilities:
//! - Validate and transform API payloads to storage models
//! - Enforce access control for workshop item operations
//! - Convert storage models to API responses
//! - Delegate persistence to the storage layer (WorkshopStore)
//! - Support all workshop item types: shapes, lines, rules, layouts

use std::sync::Arc;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::common::{
    crud::{CrudQueries, CrudService, ListParams, QueryLister},
    slug::SLUG_REGEX,
};
use crate::db::TransactionStarter;
use crate::error::ModelError;
use crate::session::session_service::SessionData;
use crate::workshop::workshop_store::{
    NewWorkshopItemDb, PatchWorkshopItemDb, WorkshopItemDb, WorkshopStore,
    EntityWorkshopItemDb,
};

/// API representation of a workshop item.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkshopItem {
    pub id: i64,
    pub name: String,
    pub slug: String, // semantic identifier for text-based references
    pub description: Option<String>,
    pub definition: serde_json::Value,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Request model for creating a new workshop item.
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct NewWorkshopItem {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    #[validate(length(min = 3, max = 100))]
    #[validate(regex(path = "*SLUG_REGEX"))]
    pub slug: String,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    pub definition: serde_json::Value,
}

/// Request model for updating an existing workshop item.
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct PatchWorkshopItem {
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,
    #[validate(length(max = 500))]
    pub description: Option<Option<String>>,
    pub definition: Option<serde_json::Value>,
}

impl<'a> From<&'a NewWorkshopItem> for NewWorkshopItemDb<'a> {
    fn from(item: &'a NewWorkshopItem) -> Self {
        Self {
            name: &item.name,
            slug: &item.slug,
            description: item.description.as_deref(),
            definition: &item.definition,
        }
    }
}

impl<'a> From<&'a PatchWorkshopItem> for PatchWorkshopItemDb<'a> {
    fn from(item: &'a PatchWorkshopItem) -> Self {
        Self {
            name: item.name.as_deref(),
            description: item.description.as_ref().map(|opt| opt.as_deref()),
            definition: item.definition.as_ref(),
        }
    }
}

impl From<WorkshopItemDb> for WorkshopItem {
    fn from(db: WorkshopItemDb) -> Self {
        Self {
            id: db.id,
            name: db.name,
            slug: db.slug,
            description: db.description,
            definition: db.definition,
            created_at: db.created_at,
            updated_at: db.updated_at,
        }
    }
}


/// Response model for entity-workshop-item association.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntityWorkshopItem {
    pub item_id: i64,           // entity id (package_id or board_id)
    pub workshop_item_id: i64,  // workshop item id 
    pub origin_id: Option<i64>, // package id for board imports, None for entity own items
    pub created_at: i64,
}

impl From<EntityWorkshopItemDb> for EntityWorkshopItem {
    fn from(db: EntityWorkshopItemDb) -> Self {
        Self {
            item_id: db.item_id,
            workshop_item_id: db.workshop_item_id,
            origin_id: db.origin_id,
            created_at: db.created_at,
        }
    }
}

/// Generic service layer for workshop items.
pub struct WorkshopService {
    transaction_starter: Arc<TransactionStarter>,
    store: Arc<WorkshopStore>,
}

impl WorkshopService {
    /// Create a new service with the provided store.
    pub fn new(transaction_starter: Arc<TransactionStarter>, store: Arc<WorkshopStore>) -> Self {
        Self { transaction_starter, store }
    }

    /// Get a workshop item by its semantic identifier (slug).
    pub async fn get_item_by_slug(&self, slug: &str) -> Result<WorkshopItem, ModelError> {
        let result = self.transaction_starter.run_in_transaction(async move |trx| {
            let item = self.store.get_item_by_slug(slug, trx).await?;
            Ok(item)
        }).await?;
        
        Ok(WorkshopItem::from(result))
    }
}

impl CrudService for WorkshopService {
    type Item = WorkshopItem;
    type NewItem = NewWorkshopItem;
    type PatchItem = PatchWorkshopItem;
    type SessionData = SessionData;
    type Lister = ListParams;
    type Id = i64;

    fn add_item<'r>(&'r self, item: &'r mut Self::NewItem, _session: &'r Self::SessionData) -> impl Future<Output = Result<Self::Item, ModelError>> + 'r {
        async move {
            let new_item = NewWorkshopItemDb::from(&*item);
            let store = self.store.clone();
            
            let result = self.transaction_starter.run_in_transaction(async move |trx| {
                let created = store.add_item(&new_item, trx).await?;
                Ok(created)
            }).await?;
            
            Ok(WorkshopItem::from(result))
        }
    }

    fn get_items<'r>(&'r self, list_params: &'r Self::Lister, _session: Option<&'r Self::SessionData>) -> impl Future<Output = Result<Vec<Self::Item>, ModelError>> {
        async move {
            let lister = QueryLister::from(list_params);
            let store = self.store.clone();

            let result = self.transaction_starter.run_in_transaction(async move |trx| {
                let items = store.get_items(&lister, trx).await?;
                Ok(items)
            }).await?;

            Ok(result.into_iter().map(WorkshopItem::from).collect())
        }
    }

    fn get_item<'r>(&'r self, id: Self::Id, _session: Option<&'r Self::SessionData>) -> impl Future<Output = Result<Self::Item, ModelError>> {
        async move {
            let store = self.store.clone();
            
            let result = self.transaction_starter.run_in_transaction(async move |trx| {
                let item = store.get_item(id, None, trx).await?;
                Ok(item)
            }).await?;

            Ok(WorkshopItem::from(result))
        }
    }

    fn update_item<'r>(&'r self, id: Self::Id, patch: &'r Self::PatchItem, _session: &'r Self::SessionData) -> impl Future<Output = Result<Self::Item, ModelError>> {
        async move {
            let patch_item = PatchWorkshopItemDb::from(patch);
            let store = self.store.clone();
            
            let result = self.transaction_starter.run_in_transaction(async move |trx| {
                let updated = store.update_item(id, &patch_item, trx).await?;
                Ok(updated)
            }).await?;

            Ok(WorkshopItem::from(result))
        }
    }

    fn delete_item<'r>(&'r self, id: Self::Id, _session: &'r Self::SessionData) -> impl Future<Output = Result<Self::Item, ModelError>> {
        async move {
            let store = self.store.clone();
            
            let result = self.transaction_starter.run_in_transaction(async move |trx| {
                let deleted = store.delete_item(id, trx).await?;
                Ok(deleted)
            }).await?;

            Ok(WorkshopItem::from(result))
        }
    }
}

/// Type aliases for specific workshop item types for backward compatibility
pub type Shape = WorkshopItem;
pub type NewShapeItem = NewWorkshopItem;
pub type PatchShapeItem = PatchWorkshopItem;
pub type ShapeService = WorkshopService;

pub type Line = WorkshopItem;
pub type NewLineItem = NewWorkshopItem;
pub type PatchLineItem = PatchWorkshopItem;
pub type LineService = WorkshopService;

pub type Rule = WorkshopItem;
pub type NewRuleItem = NewWorkshopItem;
pub type PatchRuleItem = PatchWorkshopItem;
pub type RuleService = WorkshopService;

pub type Layout = WorkshopItem;
pub type NewLayoutItem = NewWorkshopItem;
pub type PatchLayoutItem = PatchWorkshopItem;
pub type LayoutService = WorkshopService;