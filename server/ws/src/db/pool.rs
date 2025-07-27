use std::{sync::Arc};

use sqlx::sqlite::SqliteJournalMode;

/// Pool struct
pub struct Pool<T> {
    pool: Arc<T>,
}

/// Pool implementation for Sqlite
impl Pool<sqlx::Pool<sqlx::Sqlite>> {
    pub async fn new(url: &str, size: u8) -> Self {
        let opts = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(url)
            .journal_mode(SqliteJournalMode::Wal) // Use WAL mode for better concurrency
            .create_if_missing(true); // Create the database file if it doesn't exist
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(size.into())
            .connect_with(opts)
            .await
            .expect("Failed to create database pool");
        Pool { pool: Arc::new(pool) }
    }

    pub fn get(&self) -> Arc<sqlx::Pool<sqlx::Sqlite>> {
        self.pool.clone()
    }
}

