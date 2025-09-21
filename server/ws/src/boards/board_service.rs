//! Board service module: business logic for managing boards.
//!
//! Responsibilities:
//! - Validate and transform API payloads to storage models
//! - Enforce access control (e.g., `owner`/`manage` for update/delete)
//! - Convert storage models to API responses
//! - Delegate persistence to the storage layer (`BoardStore`)
//!
//! Prerequisites:
//! - An instance of `BoardStore` to interact with the database
//!
//! Example:
//! ```ignore
//! use std::sync::Arc;
//! use crate::boards::board_store::BoardStore;
//! use crate::boards::board_service::{BoardService, NewBoardItem};
//! use crate::session::session_service::SessionData;
//!
//! async fn example(pool: Arc<sqlx::Pool<sqlx::Sqlite>>, session: SessionData) {
//!     let store = BoardStore::new(pool);
//!     let service = BoardService::new(store);
//!
//!     // create
//!     let new_item = NewBoardItem { name: "Demo".into(), description: None, icon: None, is_public: Some(true) };
//!     let created = service.add_item(&new_item, &session).await.unwrap();
//!
//!     // list (visible to session)
//!     let visible = service.get_items(&Some(session)).await.unwrap();
//!     assert!(visible.iter().any(|p| p.id == created.id));
//! }
//! ```
use std::sync::Arc;

use crate::common::access::{ ItemAccessQueries, ItemRoleDb, SqliteItemAccessQueries, ROLE_OWNER, ROLE_MANAGE, ROLE_EDIT };
use crate::common::crud::{CrudQueries, CrudService, ListParams, QueryFilter, QueryLister};
use crate::common::slug::generate_slug;
use crate::db::TransactionStarter;
use crate::error::ModelError;
use crate::boards::board_store::{BoardDb, BoardStore, NewBoardDb, PatchBoardDb};
use crate::session::session_service::SessionData;
use serde::{Deserialize, Serialize};
use validator::Validate;
pub use crate::common::access::ItemAccessGrantDto as NewBoardAccessItem;

/// API representation of a board.
///
/// This is the structure returned to clients by HTTP handlers and used
/// throughout the service layer. It is converted from the storage model (`BoardDb`).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Board {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub is_public: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct NewBoardItem {
    #[serde(flatten)]
    #[validate(nested)]
    pub base: crate::common::item_fields::NewItemFields,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct PatchBoardItem {
    #[serde(flatten)]
    #[validate(nested)]
    pub base: crate::common::item_fields::PatchItemFields,
}

/// implements direct conversion from NewBoardItem to NewBoardDb
impl<'a> From<&'a NewBoardItem> for NewBoardDb<'a> {
    fn from(item: &'a NewBoardItem) -> Self {
        let slug= match &item.base.slug {
            Some(s) => s,
            None => panic!("Slug must be provided or generated in the service layer"),
        };

        Self {
            name: &item.base.name,
            slug,
            description: item.base.description.as_deref(),
            icon: item.base.icon.as_deref(),
            is_public: item.base.is_public.unwrap_or(false),
        }
    }
}


/// implements direct conversion from PatchBoardItem to PatchBoardDb
impl<'a> From<&'a PatchBoardItem> for PatchBoardDb<'a> {
    fn from(item: &'a PatchBoardItem) -> Self {
        Self {
            name: item.base.name.as_deref(),
            description: item.base.description.as_ref().map(|d| d.as_deref()),
            icon: item.base.icon.as_ref().map(|i| i.as_deref()),
            is_public: item.base.is_public,
        }
    }
}

/// implements direct conversion from BoardDb to Board
impl From<BoardDb> for Board {
    fn from(item: BoardDb) -> Self {
        Self {
            id: item.id,
            name: item.name,
            slug: item.slug,
            description: item.description,
            icon: item.icon,
            is_public: item.is_public,
        }
    }
}

/// Lister for boards
pub type BoardLister = ListParams;

/// Service layer encapsulating business logic for boards.
///
/// Responsibilities:
/// - Validation and conversion between API payloads and storage models
/// - Access control decisions that require knowing the caller's session
/// - Delegating persistence to `BoardStore`
pub struct BoardService {
    transaction_starter: Arc<TransactionStarter>,
    items_store: Arc<BoardStore>,
    access_store: Arc<SqliteItemAccessQueries>,
}

impl BoardService {
    /// Create a new service with the provided store.
    pub fn new(transaction_starter: Arc<TransactionStarter>, items_store: Arc<BoardStore>, access_store: Arc<SqliteItemAccessQueries>) -> Self {
        Self {
            transaction_starter,
            items_store,
            access_store,
        }
    }
}

impl CrudService for BoardService {
    type Item = Board;
    type NewItem = NewBoardItem;
    type PatchItem = PatchBoardItem;
    type SessionData = SessionData;
    type Lister = BoardLister;
    type Id = i64;

    /// Create a new board for the authenticated user.
    /// - Persists a new board via the store
    /// - Grants the caller the `owner` role
    async fn add_item<'r>(&'r self, item: &'r mut Self::NewItem, session: &'r Self::SessionData) -> Result<Self::Item, ModelError> {
        let slug = generate_slug(item.base.slug.as_deref(), &item.base.name);
        item.base.slug = Some(slug);

        let db_item = NewBoardDb::from(& *item);

        let result = self.transaction_starter.run_in_transaction(async move |trx| {
            let item = self.items_store.add_item(&db_item, trx).await?;
            let role = ItemRoleDb { address: session.address.clone(), role: ROLE_OWNER.to_string() };
            self.access_store.add_access_role(item.id, &role, trx).await?;

            Ok(item)
        }).await?;

        Ok(Self::Item::from(result))

    }

    /// List boards visible to the optional session.
    /// - When `session` is `None`, returns only public boards
    /// - Otherwise, returns public plus accessible boards
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

    /// Get a single board by id if visible to the session.
    async fn get_item(&self, item_id: Self::Id, session: Option<&Self::SessionData>) -> Result<Self::Item, ModelError> {
        let access = session.map(|s| s.address.as_str());

        let result = self.transaction_starter.run_in_transaction(async move |trx| {
            let item = self.items_store.get_item(item_id, access, trx).await?;

            Ok(item)
        }).await?;

        Ok(Self::Item::from(result))
    }

    /// Update a board.
    /// - Verifies the caller has `owner` or `manage` role
    /// - Applies partial updates via the store
    async fn update_item(&self, id: Self::Id, item: &Self::PatchItem, session: &Self::SessionData) -> Result<Self::Item, ModelError> {
        let db_item = PatchBoardDb::from(item);

        let result = self.transaction_starter.run_in_transaction(async move |trx| {
            let roles = self.access_store.get_access_roles(id, session, Some(&Vec::from([ROLE_OWNER, ROLE_MANAGE, ROLE_EDIT])), trx).await?;

            if roles.len() == 0 {
                return Err(ModelError::Forbidden("You are not allowed to update this board".to_string()));
            }

            let updated_item = self.items_store.update_item(id, &db_item, trx).await?;

            Ok(updated_item)
        }).await?;

        Ok(Self::Item::from(result))
    }

    /// Delete a board.
    /// - Verifies the caller has `owner` or `manage` role
    /// - Deletes the board (and related access rows in store)
    async fn delete_item(&self, id: Self::Id, session: &Self::SessionData) -> Result<Self::Item, ModelError> {
        let result = self.transaction_starter.run_in_transaction(async move |trx| {
            let roles = self.access_store.get_access_roles(id, session, Some(&Vec::from([ROLE_OWNER, ROLE_MANAGE])), trx).await?;

            if roles.len() == 0 {
                return Err(ModelError::Forbidden("You are not allowed to delete this board".to_string()));
            }

            let deleted_item = self.items_store.delete_item(id, trx).await?;

            Ok(deleted_item)
        }).await?;

        Ok(Self::Item::from(result))
    }
}