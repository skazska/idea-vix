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
//
use crate::error::ModelError;
use crate::boards::board_store::{BoardDb, BoardStore, NewBoardDb, PatchBoardDb, BoardAccessRoleDb, BoardAccessRolesDb, NewBoardAccessDb};
use crate::session::session_service::SessionData;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// API representation of a board.
///
/// This is the structure returned to clients by HTTP handlers and used
/// throughout the service layer. It is converted from the storage model (`BoardDb`).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Board {
    pub id: i32,
    pub name: String,
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
        Self {
            name: &item.base.name,
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
            description: item.description,
            icon: item.icon,
            is_public: item.is_public,
        }
    }
}

pub use crate::common::access::RoleOnly as BoardAccessRole;
pub use crate::common::access::GrantRequest as NewBoardAccessItem;
use crate::common::access::validate_grant_role;

/// Response model for a board access mapping.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BoardAccessRoles {
    pub board_id: i32,
    pub address: String,
    pub role: String,
}

impl From<BoardAccessRolesDb> for BoardAccessRoles {
    fn from(value: BoardAccessRolesDb) -> Self {
        Self { board_id: value.board_id, address: value.address, role: value.role }
    }
}

// NewBoardAccessItem is re-exported from common::access::GrantRequest

/// Service layer encapsulating business logic for boards.
pub struct BoardService {
    items_store: BoardStore,
}

impl<'a> BoardService {
    pub fn new(items_store: BoardStore) -> Self {
        Self {
            items_store: items_store,
        }
    }

    pub async fn add_item(&self, item: &'a NewBoardItem, session: &SessionData) -> Result<Board, ModelError> {
        let result = self.items_store.add_item(item.into(), session).await?;

        Ok(result.into())
    }

    pub async fn get_items(&self, session: &Option<SessionData>) -> Result<Vec<Board>, ModelError> {
        let items = self.items_store.get_items(session).await?;

        Ok(items.into_iter().map(|item| item.into()).collect())
    }

    pub async fn get_item(&self, item_id: i32, session: &Option<SessionData>) -> Result<Board, ModelError> {
        let item = self.items_store.get_item(item_id, session).await?;

        Ok(item.into())
    }

    pub async fn update_item(&self, id: i32, item: &'a PatchBoardItem, session: &SessionData) -> Result<Board, ModelError> {
        // access check moved to service: require owner or manage
        let roles = self.items_store.get_access_roles(id, session).await?;
        if !roles.iter().any(|r| r.role == "owner" || r.role == "manage") {
            return Err(ModelError::Forbidden("You are not allowed to update this board".to_string()));
        }

        let updated_item = self.items_store.update_item(id, item.into()).await?;

        Ok(updated_item.into())
    }

    pub async fn delete_item(&self, id: i32, session: &SessionData) -> Result<Board, ModelError> {
        // access check moved to service: require owner or manage
        let roles = self.items_store.get_access_roles(id, session).await?;
        if !roles.iter().any(|r| r.role == "owner" || r.role == "manage") {
            return Err(ModelError::Forbidden("You are not allowed to delete this board".to_string()));
        }

        let result = self.items_store.delete_item(id).await?;

        Ok(result.into())
    }

    /// Grant access role to an address (owner only)
    pub async fn add_access_role(&self, board_id: i32, item: &NewBoardAccessItem, session: &SessionData) -> Result<BoardAccessRoles, ModelError> {
        // Only owner can manage access list
        let roles = self.items_store.get_access_roles(board_id, session).await?;
        if !roles.iter().any(|r| r.role == "owner") {
            return Err(ModelError::Forbidden("Only owner can grant access".to_string()));
        }

        // validate role value
    if validate_grant_role(&item.role).is_err() {
            return Err(ModelError::BadRequest("Invalid role".to_string()));
        }

        // Build store model and insert
        let db_item = NewBoardAccessDb { board_id, address: &item.address, role: &item.role };
        let stored = self.items_store.add_access_role(db_item).await?;
        Ok(stored.into())
    }

    /// Revoke access for an address (owner only)
    pub async fn revoke_access_role(&self, board_id: i32, address: &str, session: &SessionData) -> Result<BoardAccessRoles, ModelError> {
        let roles = self.items_store.get_access_roles(board_id, session).await?;
        if !roles.iter().any(|r| r.role == "owner") {
            return Err(ModelError::Forbidden("Only owner can revoke access".to_string()));
        }
        // Prevent owner from revoking their own access
        if address == session.address {
            return Err(ModelError::BadRequest("Cannot revoke own access".to_string()));
        }

        let stored = self.items_store.revoke_access_role(board_id, address).await?;
        Ok(stored.into())
    }

    /// List access mappings for a board (owner only)
    pub async fn list_access_roles(&self, board_id: i32, session: &SessionData) -> Result<Vec<BoardAccessRoles>, ModelError> {
        // Only owner can view access list
        let roles = self.items_store.get_access_roles(board_id, session).await?;
        if !roles.iter().any(|r| r.role == "owner") {
            return Err(ModelError::Forbidden("Only owner can list access".to_string()));
        }

        let rows = self.items_store.list_access_roles(board_id).await?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

// --- helpers and conversions ---
impl From<BoardAccessRoleDb> for BoardAccessRole {
    fn from(item: BoardAccessRoleDb) -> Self {
        Self { role: item.role.into() }
    }
}
