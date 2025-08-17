//! Board storage module: SQLx-backed persistence for boards and access roles.
//!
//! Scope:
//! - CRUD on `board` rows
//! - Visibility-filtered reads based on session address
//! - Access role CRUD in `board_access`
//!
//! Design notes:
//! - Uses explicit transactions where multi-step writes must be atomic
//! - Leaves access control decisions to the service layer (except visibility filters)
//! - Patch updates bind only provided fields, supporting NULL to clear values

use std::{ops::Deref, sync::Arc};

use sqlx::Error;

use crate::session::session_service::SessionData;

// Board database content model
#[derive(sqlx::FromRow, Debug)]
pub struct BoardDb {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub is_public: bool,
}

/// New board database model
#[derive(Debug)]
pub struct NewBoardDb<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub icon: Option<&'a str>,
    pub is_public: bool,
}


/// Patch board database model
#[derive(Debug)]
pub struct PatchBoardDb<'a> {
    pub name: Option<&'a str>,
    pub description: Option<Option<&'a str>>,
    pub icon: Option<Option<&'a str>>,
    pub is_public: Option<bool>,
}

/// Database model for a single role value for a (board_id, address).
#[derive(sqlx::FromRow, Debug)]
pub struct BoardAccessRoleDb {
    pub role: String,
}

/// Insert model for a board access role entry.
#[derive(Debug)]
pub struct NewBoardAccessDb<'a> {
    pub board_id: i32,
    pub address: &'a str,
    pub role: &'a str,
}

/// Database model representing an access role mapping row.
#[derive(sqlx::FromRow, Debug)]
pub struct BoardAccessRolesDb {
    pub board_id: i32,
    pub address: String,
    pub role: String,
}


/// A board store
pub struct BoardStore {
    pub pool: Arc<sqlx::Pool<sqlx::Sqlite>>,
}

impl<'a> BoardStore {
    pub fn new(pool: Arc<sqlx::Pool<sqlx::Sqlite>>) -> Self {
        Self { 
            pool,
        }
    }

    /// Adds an item to the store
    pub async fn add_item(&self, item: NewBoardDb<'a>, session: &SessionData) -> Result<BoardDb, Error> {
        let connection = self.pool.deref();
        let mut transaction = connection.begin().await?;

        let result = match sqlx::query_as::<_, BoardDb>(
            "INSERT INTO board (name, description, icon, is_public) VALUES (?, ?, ?, ?) RETURNING id, name, description, icon, is_public",
        )
        .bind(item.name)
        .bind(item.description)
        .bind(item.icon)
        .bind(item.is_public)
        .fetch_one(&mut *transaction)
        .await {
            Ok(r) => r,
            Err(err) => {
                let _ = transaction.rollback().await;
                return Err(err);
            }
        };

        // Save owner access
        match sqlx::query("INSERT INTO board_access (board_id, address, role) VALUES (?, ?, ?)")
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

    /// Retrieves all items from the store with access control
    pub async fn get_items(&self, session: &Option<SessionData>) -> Result<Vec<BoardDb>, Error> {
        let connection = self.pool.deref();
        let mut transaction = connection.begin().await?;

        let result = if let Some(session) = session {
            // Authenticated: public or has access
            match sqlx::query_as::<_, BoardDb>(
                "SELECT b.id, b.name, b.description, b.icon, b.is_public
                 FROM board b
                 WHERE b.is_public = 1 OR EXISTS (
                   SELECT 1 FROM board_access ba
                   WHERE ba.board_id = b.id AND ba.address = ?
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
            // Unauthenticated: only public
            match sqlx::query_as::<_, BoardDb>(
                "SELECT id, name, description, icon, is_public FROM board WHERE is_public = 1",
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

    /// Retrieves a single item by its ID with access control
    pub async fn get_item(&self, item_id: i32, session: &Option<SessionData>) -> Result<BoardDb, Error> {
        let connection = self.pool.deref();
        let mut transaction = connection.begin().await?;

        let result = if let Some(session) = session {
            match sqlx::query_as::<_, BoardDb>(
                "SELECT b.id, b.name, b.description, b.icon, b.is_public
                 FROM board b
                 WHERE b.id = ? AND (b.is_public = 1 OR EXISTS (
                   SELECT 1 FROM board_access ba
                   WHERE ba.board_id = b.id AND ba.address = ?
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
            match sqlx::query_as::<_, BoardDb>(
                "SELECT id, name, description, icon, is_public FROM board WHERE id = ? AND is_public = 1",
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

    /// Updates an item in the store. Access is validated by the service layer.
    ///
    /// Notes:
    /// - Builds the SQL `SET` clause with `crate::sqlx_build_set!`, adding columns only
    ///   for fields that are `Some(..)`.
    /// - Bind values must follow the exact same order as the `SET` fragments above.
    ///   We bind each provided field in sequence, then bind `id` for the `WHERE`.
    /// - For `Option<Option<&str>>` fields like `description` and `icon`:
    ///   - `Some(Some(v))` sets the value to `v`.
    ///   - `Some(None)` sets the column to `NULL`.
    ///   - `None` means "do not update this column".
    /// - If no fields are provided, returns `Error::InvalidArgument`.
    pub async fn update_item(&self, id: i32, item: PatchBoardDb<'a>) -> Result<BoardDb, Error> {
        let set = crate::sqlx_build_set!(
            item.name.is_some() => "name = ?",
            item.description.is_some() => "description = ?",
            item.icon.is_some() => "icon = ?",
            item.is_public.is_some() => "is_public = ?"
        );

        if set.is_empty() {
            return Err(Error::InvalidArgument(
                "No fields to update in board item".to_string(),
            ));
        }
        let query = format!(
            "UPDATE board SET {} WHERE id = ? RETURNING id, name, description, icon, is_public",
            set
        );

        let connection = self.pool.deref();
        let mut transaction = connection.begin().await?;

        let q = sqlx::query_as::<_, BoardDb>(&query);
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

        // Bind id for WHERE
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

    /// Deletes an item from the store and related access rows. Access validated by service.
    pub async fn delete_item(&self, id: i32) -> Result<BoardDb, Error> {
        let connection = self.pool.deref();
        let mut transaction = connection.begin().await?;

        let result = match sqlx::query_as::<_, BoardDb>("DELETE FROM board WHERE id = ? RETURNING id, name, description, icon, is_public")
            .bind(id)
            .fetch_one(&mut *transaction)
            .await {
                Ok(row) => row,
                Err(err) => {
                    let _ = transaction.rollback().await;
                    return Err(err);
                }
            };

        // clean up access rows
        match sqlx::query("DELETE FROM board_access WHERE board_id = ?")
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

    /// Fetch roles the given `session` has for a particular board id.
    pub async fn get_access_roles(&self, id: i32, session: &SessionData) -> Result<Vec<BoardAccessRoleDb>, Error> {
        let connection = self.pool.deref();

        let access = sqlx::query_as::<_, BoardAccessRoleDb>("SELECT role FROM board_access WHERE board_id = ? AND address = ?")
            .bind(id)
            .bind(&session.address)
            .fetch_all(connection)
            .await?;

        Ok(access)
    }

    /// Grant a role to an address for a specific board.
    pub async fn add_access_role(&self, item: NewBoardAccessDb<'a>) -> Result<BoardAccessRolesDb, Error> {
        let connection = self.pool.deref();

        let result = sqlx::query_as::<_, BoardAccessRolesDb>("INSERT INTO board_access (board_id, address, role) VALUES (?, ?, ?) RETURNING board_id, address, role")
            .bind(item.board_id)
            .bind(item.address)
            .bind(item.role)
            .fetch_one(connection)
            .await?;

        Ok(result)
    }

    /// Revoke access for an address and return the removed mapping row.
    pub async fn revoke_access_role(&self, board_id: i32, address: &str) -> Result<BoardAccessRolesDb, Error> {
        let connection = self.pool.deref();

        let result = sqlx::query_as::<_, BoardAccessRolesDb>("DELETE FROM board_access WHERE board_id = ? AND address = ? RETURNING board_id, address, role")
            .bind(board_id)
            .bind(address)
            .fetch_one(connection)
            .await?;

        Ok(result)
    }

    /// List all access mappings for a board id.
    pub async fn list_access_roles(&self, board_id: i32) -> Result<Vec<BoardAccessRolesDb>, Error> {
        let connection = self.pool.deref();

        let rows = sqlx::query_as::<_, BoardAccessRolesDb>(
            "SELECT board_id, address, role FROM board_access WHERE board_id = ? ORDER BY address"
        )
        .bind(board_id)
        .fetch_all(connection)
        .await?;

        Ok(rows)
    }
}
