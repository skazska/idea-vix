//! Package storage module: SQLx-backed persistence for packages and access roles.
//!
//! Scope:
//! - CRUD on `package` rows
//! - Visibility-filtered reads based on session address
//! - Access role CRUD in `package_access`
//!
//! Design notes:
//! - Uses explicit transactions where multi-step writes must be atomic
//! - Leaves access control decisions to the service layer (except visibility filters)
//! - Patch updates bind only provided fields, supporting NULL to clear values
//
use std::{ops::Deref, sync::Arc};

use sqlx::Error;

use crate::{session::session_service::SessionData};

/// Database model for a package row.
#[derive(sqlx::FromRow, Debug)]
pub struct PackageDb {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub is_public: bool,
}

/// Insert model for a new package.
#[derive(Debug)]
pub struct NewPackageDb<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub icon: Option<&'a str>,
    pub is_public: bool,
}

/// Update model for an existing package.
///
/// Notes about nested options:
/// - `description: Option<Option<&str>>`: `None` -> not provided; `Some(None)` -> set NULL; `Some(Some(v))` -> set value
/// - `icon` follows the same pattern.
#[derive(Debug)]
pub struct PatchPackageDb<'a> {
    pub name: Option<&'a str>,
    pub description: Option<Option<&'a str>>,
    pub icon: Option<Option<&'a str>>,
    pub is_public: Option<bool>,
}

/// Database model for a single role value for a (package_id, address).
#[derive(sqlx::FromRow, Debug)]
pub struct PackageAccessRoleDb {
    pub role: String,
}

/// Insert model for a package access role entry.
#[derive(Debug)]
pub struct NewPackageAccessDb<'a> {
    pub package_id: i32,
    pub address: &'a str,
    pub role: &'a str,
}

/// Database model representing an access role mapping row.
#[derive(sqlx::FromRow, Debug)]
pub struct PackageAccessRolesDb {
    pub package_id: i32,
    pub address: String,
    pub role: String,
}

// (internal helper note left intentionally blank)

/// Storage layer for packages, backed by SQLx + SQLite.
///
/// Handles transactions and low-level SQL, without applying access-control decisions
/// (those are enforced by the service layer where required).
pub struct PackageStore {
    pub pool: Arc<sqlx::Pool<sqlx::Sqlite>>,
}

impl<'a> PackageStore {
    /// Create a new store using the given pool.
    pub fn new(pool: Arc<sqlx::Pool<sqlx::Sqlite>>) -> Self {
        Self { 
            pool,
        }
    }

    /// Insert a new package and assign `owner` role to the provided session address.
    /// Uses a transaction to ensure both package row and access row are created.
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

    /// Retrieve packages visible to the optional session.
    /// When `session` is `None`, only public packages are returned.
    pub async fn get_items(&self, session: &Option<SessionData>) -> Result<Vec<PackageDb>, Error> {
        let connection = self.pool.deref();
        let mut transaction = connection.begin().await?;

        let result = if let Some(session) = session {
            match sqlx::query_as::<_, PackageDb>(
                "SELECT p.id, p.name, p.description, p.icon, p.is_public
                 FROM package p
                 WHERE p.is_public = 1 OR EXISTS (
                   SELECT 1 FROM package_access pa
                   WHERE pa.package_id = p.id AND pa.address = ?
                 )",
            )
            .bind(&session.address)
            .fetch_all(&mut *transaction)
            .await {
                Ok(rows) => rows,
                Err(err) => {
                    let _ = transaction.rollback().await;
                    return Err(err);
                }
            }
        } else {
            match sqlx::query_as::<_, PackageDb>(
                "SELECT id, name, description, icon, is_public FROM package WHERE is_public = 1",
            )
            .fetch_all(&mut *transaction)
            .await {
                Ok(rows) => rows,
                Err(err) => {
                    let _ = transaction.rollback().await;
                    return Err(err);
                }
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(result),
            Err(err) => Err(err),
        }
    }

    /// Retrieve a single package by id if visible to the optional session.
    pub async fn get_item(&self, item_id: i32, session: &Option<SessionData>) -> Result<PackageDb, Error> {
        let connection = self.pool.deref();
        let mut transaction = connection.begin().await?;

        let result = if let Some(session) = session {
            match sqlx::query_as::<_, PackageDb>(
                "SELECT p.id, p.name, p.description, p.icon, p.is_public
                 FROM package p
                 WHERE p.id = ? AND (p.is_public = 1 OR EXISTS (
                   SELECT 1 FROM package_access pa
                   WHERE pa.package_id = p.id AND pa.address = ?
                 ))",
            )
            .bind(item_id)
            .bind(&session.address)
            .fetch_one(&mut *transaction)
            .await {
                Ok(row) => row,
                Err(err) => {
                    let _ = transaction.rollback().await;
                    return Err(err);
                }
            }
        } else {
            match sqlx::query_as::<_, PackageDb>(
                "SELECT id, name, description, icon, is_public FROM package WHERE id = ? AND is_public = 1",
            )
            .bind(item_id)
            .fetch_one(&mut *transaction)
            .await {
                Ok(row) => row,
                Err(err) => {
                    let _ = transaction.rollback().await;
                    return Err(err);
                }
            }
        };

        match transaction.commit().await {
            Ok(_) => Ok(result),
            Err(err) => Err(err),
        }
    }

    /// Update a package row. Access is validated by the service layer.
    /// Dynamically builds the SQL to bind only provided fields, including NULL for cleared values.
    pub async fn update_item(&self, id: i32, item: PatchPackageDb<'a>) -> Result<PackageDb, Error> {
        let set = crate::sqlx_build_set!(
            item.name.is_some() => "name = ?",
            item.description.is_some() => "description = ?",
            item.icon.is_some() => "icon = ?",
            item.is_public.is_some() => "is_public = ?"
        );

        if set.is_empty() {
            return Err(Error::InvalidArgument(
                "No fields to update in package item".to_string(),
            ));
        }
    let query = format!(
            "UPDATE package SET {} WHERE id = ? RETURNING id, name, description, icon, is_public",
            set
        );

        let connection = self.pool.deref();
        let mut transaction = connection.begin().await?;

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

    /// Delete a package row and its access entries within a transaction.
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

        match sqlx::query("DELETE FROM package_access WHERE package_id = ?")
            .bind(id)
            .execute(&mut *transaction)
            .await {
                Ok(_) => {}
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

    /// Fetch roles the given `session` has for a particular package id.
    pub async fn get_access_roles(&self, id: i32, session: &SessionData) -> Result<Vec<PackageAccessRoleDb>, Error> {
        let connection = self.pool.deref();

        let access = sqlx::query_as::<_, PackageAccessRoleDb>("SELECT role FROM package_access WHERE package_id = ? AND address = ?")
            .bind(id)
            .bind(&session.address)
            .fetch_all(connection)
            .await?;

        Ok(access)
    }

    /// Grant a role to an address for a specific package.
    pub async fn add_access_role(&self, item: NewPackageAccessDb<'a>) -> Result<PackageAccessRolesDb, Error> {
        let connection = self.pool.deref();

        let result = sqlx::query_as::<_, PackageAccessRolesDb>("INSERT INTO package_access (package_id, address, role) VALUES (?, ?, ?) RETURNING package_id, address, role")
            .bind(item.package_id)
            .bind(item.address)
            .bind(item.role)
            .fetch_one(connection)
            .await?;

        Ok(result)
    }

    /// Revoke access for an address and return the removed mapping row.
    pub async fn revoke_access_role(&self, package_id: i32, address: &str) -> Result<PackageAccessRolesDb, Error> {
        let connection = self.pool.deref();

        let result = sqlx::query_as::<_, PackageAccessRolesDb>("DELETE FROM package_access WHERE package_id = ? AND address = ? RETURNING package_id, address, role")
            .bind(package_id)
            .bind(address)
            .fetch_one(connection)
            .await?;

        Ok(result)
    }

    /// List all access mappings for a package id.
    pub async fn list_access_roles(&self, package_id: i32) -> Result<Vec<PackageAccessRolesDb>, Error> {
        let connection = self.pool.deref();

        let rows = sqlx::query_as::<_, PackageAccessRolesDb>(
            "SELECT package_id, address, role FROM package_access WHERE package_id = ? ORDER BY address"
        )
        .bind(package_id)
        .fetch_all(connection)
        .await?;

        Ok(rows)
    }
}
