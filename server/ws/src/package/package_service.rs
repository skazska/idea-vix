use crate::error::ModelError;
use crate::package::package_store::{NewPackageDb, PackageAccessRoleDb, PackageDb, PackageStore, PatchPackageDb};
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
    pub is_public: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct NewPackageItem {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    #[validate(length(max = 255))]
    pub icon: Option<String>,
    pub is_public: Option<bool>,
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
    pub is_public: Option<bool>,
}

/// implements direct conversion from NewPackageItem to NewPackageDb
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

/// implements direct conversion from PatchPackageItem to PatchPackageDb
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

/// implements direct conversion from PackageDb to Package
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PackageAccess {
    Owner,
    Manage,
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

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct PackageAccessRole {
    pub role: PackageAccess,
}

impl From<PackageAccessRoleDb> for PackageAccessRole {
    fn from(item: PackageAccessRoleDb) -> Self {
        Self {
            role: item.role.into(),
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
        let result = self.items_store.add_item(item.into(), session).await?;

        Ok(result.into())
    }

    pub async fn get_items(&self, session: &Option<SessionData>) -> Result<Vec<Package>, ModelError> {
        let items = self.items_store.get_items(session).await?;

        Ok(items.into_iter().map(|item| item.into()).collect())
    }

    pub async fn get_item(&self, item_id: i32, session: &Option<SessionData>) -> Result<Package, ModelError> {
        let item = self.items_store.get_item(item_id, session).await?;

        Ok(item.into())
    }

    pub async fn update_item(&self, id: i32, item: &'a PatchPackageItem, session: &SessionData) -> Result<Package, ModelError> {
        let updated_item = self.items_store.update_item(id, item.into(), session).await?;

        Ok(updated_item.into())
    }

    pub async fn delete_item(&self, id: i32, session: &SessionData) -> Result<Package, ModelError> {
        let roles = self.items_store.get_access_roles(id, session).await?;

        if !roles.iter().any(|r| r.role == "owner" || r.role == "manage") {
            return Err(ModelError::Forbidden("You are not allowed to delete this package".to_string()));
        }

        let result = self.items_store.delete_item(id).await?;

        Ok(result.into())
    }

}
