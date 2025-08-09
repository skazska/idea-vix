use crate::error::ModelError;
use crate::package::package_store::{NewPackageDb, PackageDb, PackageStore, PatchPackageDb};
use crate::session::session_service::SessionData;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::{api::deserialize::deserialize_some};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct NewPackageItem {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    #[validate(length(max = 255))]
    pub icon: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct PatchPackageItem {
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_some")]
    #[validate(length(max = 500))]
    pub description: Option<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_some")]
    #[validate(length(max = 255))]
    pub icon: Option<Option<String>>,
}

/// implements direct conversion from NewPackageItem to NewPackageDb
impl<'a> From<&'a NewPackageItem> for NewPackageDb<'a> {
    fn from(item: &'a NewPackageItem) -> Self {
        Self {
            name: &item.name,
            description: item.description.as_deref(),
            icon: item.icon.as_deref(),
        }
    }
}

/// implements direct conversion from PatchPackageItem to PatchPackageDb
impl<'a> From<&'a PatchPackageItem> for PatchPackageDb<'a> {
    fn from(item: &'a PatchPackageItem) -> Self {
        Self {
            name: item.name.as_deref(),
            description: item.description.as_ref().map(|d| d.as_deref()),
            icon: item.icon.as_ref().map(|i| i.as_deref()),
        }
    }
}

/// implements direct conversion from PackageDb to Package
impl From<PackageDb> for Package {
    fn from(item: PackageDb) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon: item.icon,
        }
    }
}


pub struct PackageService {
    items_store: PackageStore,
}

impl<'a> PackageService {
    pub fn new(items_store: PackageStore) -> Self {
        Self {
            items_store: items_store,
        }
    }

    pub async fn add_item(&self, item: &'a NewPackageItem, session: &SessionData) -> Result<Package, ModelError> {
        let result = self.items_store.add_item(item.into()).await?;

        Ok(result.into())
    }

    pub async fn get_items(&self, _session: &Option<SessionData>) -> Result<Vec<Package>, ModelError> {
        let items = self.items_store.get_items().await?;

        Ok(items.into_iter().map(|item| item.into()).collect())
    }

    pub async fn get_item(&self, item_id: i32, _session: &Option<SessionData>) -> Result<Package, ModelError> {
        let item = self.items_store.get_item(item_id).await?;

        Ok(item.into())
    }

    pub async fn update_item(&self, id: i32, item: &'a PatchPackageItem, _session: &SessionData) -> Result<Package, ModelError> {
        let updated_item = self.items_store.update_item(id, item.into()).await?;

        Ok(updated_item.into())
    }

    pub async fn delete_item(&self, id: i32, _session: &SessionData) -> Result<Package, ModelError> {
        let result = self.items_store.delete_item(id).await?;

        Ok(result.into())
    }

}
