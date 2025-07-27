use std::{ops::Deref, sync::Arc};

use sqlx::Error;

use crate::{db::models::{ NewPackageDb, PatchPackageDb, PackageDb }};

/// A package store
pub struct PackageStore {
    pub pool: Arc<sqlx::Pool<sqlx::Sqlite>>,
}

impl<'a> PackageStore {
    pub fn new(pool: Arc<sqlx::Pool<sqlx::Sqlite>>) -> Self {
        Self { 
            pool,
        }
    }

    /// Adds an item to the store
    pub async fn add_item(&self, item: NewPackageDb<'a>) -> Result<PackageDb, Error> {
        sqlx::query_as::<_, PackageDb>(
            "INSERT INTO package (name, description, icon) VALUES (?, ?, ?) RETURNING id, name, description, icon",
        ).bind(item.name)
        .bind(item.description)
        .bind(item.icon)
        .fetch_one(self.pool.deref())
        .await
    }

    /// Retrieves all items from the store
    pub async fn get_items(&self) -> Result<Vec<PackageDb>, Error> {
        sqlx::query_as::<_, PackageDb>(
            "SELECT id, name, description, icon FROM package"
        )
        .fetch_all(self.pool.deref())
        .await
    }

    /// Retrieves a single item by its ID
    pub async fn get_item(&self, item_id: i32) -> Result<PackageDb, Error> {
        sqlx::query_as::<_, PackageDb>(
            "SELECT id, name, description, icon FROM package WHERE id = ?"
        )
        .bind(item_id)
        .fetch_one(self.pool.deref())
        .await
    }

    /// Updates an item in the store
    pub async fn update_item(&self, id: i32, item: PatchPackageDb<'a>) -> Result<PackageDb, Error> {
        let mut query = String::from("UPDATE package SET ");

        //params strings
        let mut params = Vec::<&str>::new();

        if item.name.is_some() { params.push("name = ?");}
        if item.description.is_some() { params.push("description = ? "); }
        if item.icon.is_some() { params.push("icon = ? "); }

        if params.is_empty() {
            return Err(Error::InvalidArgument(
                "No fields to update in package item".to_string(),
            ));
        }

        query.push_str(&params.join(", "));

        query.push_str("WHERE id = ? RETURNING id, name, description, icon");


        println!("Executing query: {}", query);

        let q = sqlx::query_as::<_, PackageDb>(&query);
        let q = match item.name {
            Some(name) => q.bind(name),
            _ => q,
        };
        let q = match item.description {
            Some(Some(desc)) => q.bind(desc),
            _ => q,
        };
        let q = match item.icon {
            Some(Some(icon)) => q.bind(icon),
            _ => q,
        };

        q.bind(&id).fetch_one(self.pool.deref())
            .await
    }

    /// Deletes an item from the store
    pub async fn delete_item(&self, id: i32) -> Result<PackageDb, Error> {
        sqlx::query_as::<_, PackageDb>("DELETE FROM package WHERE id = ? RETURNING id, name, description, icon")
            .bind(id)
            .fetch_one(self.pool.deref())
            .await
    }
}
