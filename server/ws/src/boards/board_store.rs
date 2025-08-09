use std::{ops::Deref, sync::Arc};

use sqlx::Error;

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
    pub async fn add_item(&self, item: NewBoardDb<'a>) -> Result<BoardDb, Error> {
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

        match transaction.commit().await {
            Ok(_) => Ok(result),
            Err(err) => Err(err),
        }
    }

    /// Retrieves all items from the store
    pub async fn get_items(&self) -> Result<Vec<BoardDb>, Error> {
        let connection = self.pool.deref();
        let mut transaction = connection.begin().await?;

        let result = match sqlx::query_as::<_, BoardDb>(
            "SELECT id, name, description, icon FROM board"
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
    pub async fn get_item(&self, item_id: i32) -> Result<BoardDb, Error> {
        let connection = self.pool.deref();
        let mut transaction = connection.begin().await?;

        let result = match sqlx::query_as::<_, BoardDb>(
            "SELECT id, name, description, icon FROM board WHERE id = ?"
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
    pub async fn update_item(&self, id: i32, item: PatchBoardDb<'a>) -> Result<BoardDb, Error> {
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
        query.push_str(" WHERE id = ? RETURNING id, name, description, icon");

        let connection = self.pool.deref();
        let mut transaction = connection.begin().await?;

        // println!("Executing query: {}", query);

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
    pub async fn delete_item(&self, id: i32) -> Result<BoardDb, Error> {
        let connection = self.pool.deref();
        let mut transaction = connection.begin().await?;

        let result = match sqlx::query_as::<_, BoardDb>("DELETE FROM board WHERE id = ? RETURNING id, name, description, icon")
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
