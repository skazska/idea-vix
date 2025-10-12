use std::sync::Arc;

use crate::{
    common::{
        crud::{CrudQueries, ListParams, QueryLister},
    },
    db::TransactionStarter,
    error::ModelError,
    session::session_service::SessionData,
    workshop::{
        workshop_service::{NewWorkshopItem, PatchWorkshopItem, WorkshopItem},
        workshop_store::{NewWorkshopItemDb, PatchWorkshopItemDb, WorkshopStore}
    }
};

/// Generic service layer for workshop items.
pub struct WorkshopEntityService {
    transaction_starter: Arc<TransactionStarter>,
    store: Arc<WorkshopStore>,
    entity_table: String, // "package" or "board"
}

impl WorkshopEntityService {
    /// Create a new service with the provided store.
    pub fn new(entity_table: impl AsRef<str>, transaction_starter: Arc<TransactionStarter>, store: Arc<WorkshopStore>) -> Self {
        Self { transaction_starter, store, entity_table: entity_table.as_ref().to_string() }
    }

    /// List workshop items linked to an entity.
    pub async fn list_entity_items(
        &self,
        entity_id: i64,
        list_params: &ListParams,
        _session: Option<&SessionData>,
    ) -> Result<Vec<WorkshopItem>, ModelError> {
        let store = self.store.clone();
        
        let lister = QueryLister::from(list_params);

        let result = self.transaction_starter.run_in_transaction(async move |trx| {
            // TODO: Validate that session has access to entity

            let items = store.list_entity_items(&self.entity_table, entity_id, &lister, trx).await?;

            Ok(items.into_iter().map(WorkshopItem::from).collect())
        }).await?;

        Ok(result)
    }

    /// Create a workshop item and link it to an entity (package or board).
    pub async fn create_entity_item(
        &self,
        entity_id: i64,
        new_item: &mut NewWorkshopItem,
        _session: &SessionData,
    ) -> Result<WorkshopItem, ModelError> {
        let store = self.store.clone();
        let result = self.transaction_starter.run_in_transaction(async move |trx| {
            // TODO: Validate that session has manage access to entity

            // First, create the workshop item globally
            let new_item_db = NewWorkshopItemDb::from(&*new_item);
            let created_item = store.add_item(&new_item_db, trx).await?;
            
            // Then, link it to the entity (package/board owns this item, so origin_id is None)
            let _link = store.link_item_to_entity(
                &self.entity_table,
                entity_id,
                created_item.id,
                None, // origin_id - None means entity owns the item
                trx,
            ).await?;

            Ok(WorkshopItem::from(created_item))
        }).await?;

        Ok(result)
    }

    /// update a workshop item linked to an entity (package or board).
    pub async fn update_entity_item(
        &self,
        _entity_id: i64,
        item_id: i64,
        patch: &PatchWorkshopItem,
        _session: &SessionData,
    ) -> Result<WorkshopItem, ModelError> {
        let store = self.store.clone();
        let patch_item = PatchWorkshopItemDb::from(patch);
        let result = self.transaction_starter.run_in_transaction(async move |trx| {
            // TODO: Validate that session has manage access to entity

            let updated_item = store.update_item(item_id, &patch_item, trx).await?;
            Ok(WorkshopItem::from(updated_item))
        }).await?;

        Ok(result)
    }

    /// Delete a workshop item linked to an entity (package or board).
    pub async fn delete_entity_item(
        &self,
        entity_id: i64,
        item_id: i64,
        _session: &SessionData,
    ) -> Result<(), ModelError> {
        let store = self.store.clone();
        self.transaction_starter.run_in_transaction(async move |trx| {
            // TODO: Validate that session has manage access to entity

            store.unlink_item_from_entity(&self.entity_table, entity_id, item_id, trx).await?;
            Ok(())
        }).await?;

        Ok(())
    }
}

