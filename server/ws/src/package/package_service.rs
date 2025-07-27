use crate::error::ModelError;
use crate::package::package_store::PackageStore;
use crate::package::package_types::{ NewPackageItem, PatchPackageItem, Package };

pub struct PackageService {
    items_store: PackageStore,
}

impl<'a> PackageService {
    pub fn new(items_store: PackageStore) -> Self {
        Self {
            items_store: items_store,
        }
    }

    pub async fn add_item(&self, item: &'a NewPackageItem) -> Result<Package, ModelError> {
        let result = self.items_store.add_item(item.into()).await?;

        Ok(result.into())
    }

    pub async fn get_items(&self) -> Result<Vec<Package>, ModelError> {
        let items = self.items_store.get_items().await?;

        Ok(items.into_iter().map(|item| item.into()).collect())
    }

    pub async fn get_item(&self, item_id: i32) -> Result<Package, ModelError> {
        let item = self.items_store.get_item(item_id).await?;

        Ok(item.into())
    }

    pub async fn update_item(&self, id: i32, item: &'a PatchPackageItem) -> Result<Package, ModelError> {
        let updated_item = self.items_store.update_item(id, item.into()).await?;

        Ok(updated_item.into())
    }

    pub async fn delete_item(&self, id: i32) -> Result<Package, ModelError> {
        let result = self.items_store.delete_item(id).await?;

        Ok(result.into())
    }

}
