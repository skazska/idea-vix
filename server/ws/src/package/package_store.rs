use std::{ops::Deref, sync::Arc};

use sqlx::Error;

use crate::session::session_service::{Session, SessionData};

/// Package database content model
#[derive(sqlx::FromRow, Debug)]
pub struct PackageDb {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub is_public: bool,
}

/// New package database model
#[derive(Debug)]
pub struct NewPackageDb<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub icon: Option<&'a str>,
    pub is_public: bool,
}

/// Patch package database model
#[derive(Debug)]
pub struct PatchPackageDb<'a> {
    pub name: Option<&'a str>,
    pub description: Option<Option<&'a str>>,
    pub icon: Option<Option<&'a str>>,
    pub is_public: Option<bool>,
}

/// returns function 

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
    pub async fn add_item(&self, item: NewPackageDb<'a>, session: &SessionData) -> Result<PackageDb, Error> {
        let connection = self.pool.deref();

        // Start a transaction
        let mut transaction = connection.begin().await?;

        // Use transaction for all operations and commit at the end
        let result = match sqlx::query_as::<_, PackageDb>(
            "INSERT INTO package (name, description, icon, is_public) VALUES (?, ?, ?, ?) RETURNING id, name, description, icon, is_public",
        )
            .bind(item.name)
            .bind(item.description)
            .bind(item.icon)
            .bind(item.is_public)
            .fetch_one(&mut *transaction)
            .await {
                Ok(result) => result,
                Err(err) => {
                    let _ = transaction.rollback().await;
                    return Err(err);
                }
            };

        match sqlx::query("INSERT INTO package_access (package_id, address, role) VALUES (?, ?, ?)")
            .bind(result.id)
            .bind(&session.address)
            .bind("owner")
            .execute(&mut *transaction)
            .await {
                Ok(_) => {},
                Err(err) => {
                    let _ = transaction.rollback().await;
                    return Err(err);
                }
            };

        match transaction.commit().await {
            Ok(_) => Ok(result),
            Err(err) => Err(err),
        }
    }

    /// Retrieves all items from the store
    pub async fn get_items(&self) -> Result<Vec<PackageDb>, Error> {
        let connection = self.pool.deref();
        let mut transaction = connection.begin().await?;

        let result = match sqlx::query_as::<_, PackageDb>(
            "SELECT id, name, description, icon, is_public FROM package"
        )
        .fetch_all(&mut *transaction)
        .await {
            Ok(rows) => rows,
            Err(err) => {
                let _ = transaction.rollback().await;
                return Err(err);
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(result),
            Err(err) => Err(err),
        }
    }

    /// Retrieves a single item by its ID
    pub async fn get_item(&self, item_id: i32) -> Result<PackageDb, Error> {
        let connection = self.pool.deref();
        let mut transaction = connection.begin().await?;

        let result = match sqlx::query_as::<_, PackageDb>(
            "SELECT id, name, description, icon, is_public FROM package WHERE id = ?"
        )
        .bind(item_id)
        .fetch_one(&mut *transaction)
        .await {
            Ok(row) => row,
            Err(err) => {
                let _ = transaction.rollback().await;
                return Err(err);
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(result),
            Err(err) => Err(err),
        }
    }

    /// Updates an item in the store
    pub async fn update_item(&self, id: i32, item: PatchPackageDb<'a>) -> Result<PackageDb, Error> {
        let mut query = String::from("UPDATE package SET ");

        //params strings
        let mut params = Vec::<&str>::new();

        if item.name.is_some() { params.push("name = ?");}
        if item.description.is_some() { params.push("description = ?"); }
        if item.icon.is_some() { params.push("icon = ?"); }
        if item.is_public.is_some() { params.push("is_public = ?"); }

        if params.is_empty() {
            return Err(Error::InvalidArgument(
                "No fields to update in package item".to_string(),
            ));
        }

        query.push_str(&params.join(", "));
        // Ensure a space before WHERE
        query.push_str(" WHERE id = ? RETURNING id, name, description, icon, is_public");

        let connection = self.pool.deref();
        let mut transaction = connection.begin().await?;

        // println!("Executing query: {}", query);

        let q = sqlx::query_as::<_, PackageDb>(&query);
        let q = match item.name {
            Some(name) => q.bind(name),
            _ => q,
        };
        // Bind Option<&str> correctly when provided, including NULL
        let q = match item.description {
            Some(desc_opt) => q.bind(desc_opt),
            None => q,
        };
        let q = match item.icon {
            Some(icon_opt) => q.bind(icon_opt),
            None => q,
        };
        let q = match item.is_public {
            Some(is_public) => q.bind(is_public),
            None => q,
        };

        let result = match q
            .bind(&id)
            .fetch_one(&mut *transaction)
            .await {
                Ok(row) => row,
                Err(err) => {
                    let _ = transaction.rollback().await;
                    return Err(err);
                }
            };

        match transaction.commit().await {
            Ok(_) => Ok(result),
            Err(err) => Err(err),
        }
    }

    /// Deletes an item from the store
    pub async fn delete_item(&self, id: i32) -> Result<PackageDb, Error> {
        let connection = self.pool.deref();
        let mut transaction = connection.begin().await?;

        let result = match sqlx::query_as::<_, PackageDb>("DELETE FROM package WHERE id = ? RETURNING id, name, description, icon, is_public")
            .bind(id)
            .fetch_one(&mut *transaction)
            .await {
                Ok(row) => row,
                Err(err) => {
                    let _ = transaction.rollback().await;
                    return Err(err);
                }
            };

        match transaction.commit().await {
            Ok(_) => Ok(result),
            Err(err) => Err(err),
        }
    }
}
