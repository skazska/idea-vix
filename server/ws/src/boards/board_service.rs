use crate::error::ModelError;
use crate::boards::board_store::{BoardDb, BoardStore, NewBoardDb, PatchBoardDb};
use crate::session::jwt::SessionJWTData;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::api::deserialize::deserialize_some;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Board {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct NewBoardItem {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    #[validate(length(max = 255))]
    pub icon: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct PatchBoardItem {
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_some")]
    #[validate(length(max = 500))]
    pub description: Option<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_some")]
    #[validate(length(max = 255))]
    pub icon: Option<Option<String>>,
}

/// implements direct conversion from NewBoardItem to NewBoardDb
impl<'a> From<&'a NewBoardItem> for NewBoardDb<'a> {
    fn from(item: &'a NewBoardItem) -> Self {
        Self {
            name: &item.name,
            description: item.description.as_deref(),
            icon: item.icon.as_deref(),
        }
    }
}


/// implements direct conversion from PatchBoardItem to PatchBoardDb
impl<'a> From<&'a PatchBoardItem> for PatchBoardDb<'a> {
    fn from(item: &'a PatchBoardItem) -> Self {
        Self {
            name: item.name.as_deref(),
            description: item.description.as_ref().map(|d| d.as_deref()),
            icon: item.icon.as_ref().map(|i| i.as_deref()),
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
        }
    }
}


pub struct BoardService {
    items_store: BoardStore,
}

impl<'a> BoardService {
    pub fn new(items_store: BoardStore) -> Self {
        Self {
            items_store: items_store,
        }
    }

    pub async fn add_item(&self, item: &'a NewBoardItem, session: &SessionJWTData) -> Result<Board, ModelError> {
        let result = self.items_store.add_item(item.into()).await?;

        Ok(result.into())
    }

    pub async fn get_items(&self, _session: &Option<SessionJWTData>) -> Result<Vec<Board>, ModelError> {
        let items = self.items_store.get_items().await?;

        Ok(items.into_iter().map(|item| item.into()).collect())
    }

    pub async fn get_item(&self, item_id: i32, _session: &Option<SessionJWTData>) -> Result<Board, ModelError> {
        let item = self.items_store.get_item(item_id).await?;

        Ok(item.into())
    }

    pub async fn update_item(&self, id: i32, item: &'a PatchBoardItem, _session: &SessionJWTData) -> Result<Board, ModelError> {
        let updated_item = self.items_store.update_item(id, item.into()).await?;

        Ok(updated_item.into())
    }

    pub async fn delete_item(&self, id: i32, _session: &SessionJWTData) -> Result<Board, ModelError> {
        let result = self.items_store.delete_item(id).await?;

        Ok(result.into())
    }

}
