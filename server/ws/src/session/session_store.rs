use std::{ sync::Arc, ops::Deref };

// Session database contents model 
#[derive(sqlx::FromRow, Debug)]
pub struct SessionDb {
    pub id: i64,
    pub address: String,
    pub sent_at: i64,
    pub expires_at: i64,
}

/// Init session database model
#[derive(Debug)]
pub struct InitSessionDb<'a> {
    pub address: &'a str,
    pub code: &'a str,
    pub sent_at: i64,
    pub expires_at: i64,
}

/// Confirm session database model
#[derive(Debug)]
pub struct ConfirmSessionDb<'a> {
    pub address: &'a str,
    pub code: &'a str,
}

/// A session store
/// This store handles user sessions, including initialization and confirmation.
pub struct SessionStore {
    pool: Arc<sqlx::Pool<sqlx::Sqlite>>,
}

impl<'a> SessionStore {
    pub fn new(pool: Arc<sqlx::Pool<sqlx::Sqlite>>) -> Self { Self { pool } }

    /// Initializes a session by storing the address, code, and sent time in the database
    pub async fn init_session(&self, item: InitSessionDb<'a>) -> Result<SessionDb, sqlx::Error> {
        let connection = self.pool.deref();
        let mut transaction = connection.begin().await?;

        let result = match sqlx::query_as::<_, SessionDb>(
            "INSERT INTO session (address, code, sent_at, expires_at) VALUES (?, ?, ?, ?) RETURNING id, address, code, sent_at, expires_at",
        )
        .bind(item.address)
        .bind(item.code)
        .bind(item.sent_at)
        .bind(item.expires_at)
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

    /// Confirms a session by checking the address and code
    pub async fn confirm_session(&self, item: ConfirmSessionDb<'a>) -> Result<SessionDb, sqlx::Error> {
        let connection = self.pool.deref();
        let mut transaction = connection.begin().await?;

        let result = match sqlx::query_as::<_, SessionDb>(
            "SELECT id, address, sent_at, expires_at FROM session WHERE address = ? AND code = ?",
        )
        .bind(item.address)
        .bind(item.code)
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