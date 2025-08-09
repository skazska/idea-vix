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
}

/// New board database model
#[derive(Debug)]
pub struct NewBoardDb<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub icon: Option<&'a str>,
}


/// Patch board database model
#[derive(Debug)]
pub struct PatchBoardDb<'a> {
    pub name: Option<&'a str>,
    pub description: Option<Option<&'a str>>,
    pub icon: Option<Option<&'a str>>,
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
            "INSERT INTO board (name, description, icon) VALUES (?, ?, ?) RETURNING id, name, description, icon",
        )
        .bind(item.name)
        .bind(item.description)
        .bind(item.icon)
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
                "SELECT b.id, b.name, b.description, b.icon
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
                "SELECT id, name, description, icon FROM board WHERE is_public = 1",
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
                "SELECT b.id, b.name, b.description, b.icon
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
                "SELECT id, name, description, icon FROM board WHERE id = ? AND is_public = 1",
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

    /// Updates an item in the store (owner or manage)
    pub async fn update_item(&self, id: i32, item: PatchBoardDb<'a>, session: &SessionData) -> Result<BoardDb, Error> {
        let mut query = String::from("UPDATE board SET ");

        //params strings
        let mut params = Vec::<&str>::new();

        if item.name.is_some() { params.push("name = ?");}
        if item.description.is_some() { params.push("description = ?"); }
        if item.icon.is_some() { params.push("icon = ?"); }

        if params.is_empty() {
            return Err(Error::InvalidArgument(
                "No fields to update in board item".to_string(),
            ));
        }

        query.push_str(&params.join(", "));
        query.push_str(" WHERE id = ? AND EXISTS (SELECT 1 FROM board_access WHERE board_id = ? AND address = ? AND role IN ('owner','manage')) RETURNING id, name, description, icon");

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

        // Bind id for WHERE, then id again for EXISTS, then address
        let result = match q
            .bind(&id)
            .bind(&id)
            .bind(&session.address)
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

    /// Deletes an item from the store (owner only)
    pub async fn delete_item(&self, id: i32, session: &SessionData) -> Result<BoardDb, Error> {
        let connection = self.pool.deref();
        let mut transaction = connection.begin().await?;

        let result = match sqlx::query_as::<_, BoardDb>("DELETE FROM board WHERE id = ? AND EXISTS (SELECT 1 FROM board_access WHERE board_id = ? AND address = ? AND role = 'owner') RETURNING id, name, description, icon")
            .bind(id)
            .bind(id)
            .bind(&session.address)
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
