/// Common module for database infrastructure.

use std::{sync::Arc, time::SystemTime};

use sqlx::sqlite::SqliteJournalMode;

/// returns unix timestamp from SystemTime
pub fn to_unix_timestamp(system_time: SystemTime) -> u64 {
    system_time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

/// converts unix timestamp to SystemTime
pub fn from_unix_timestamp(timestamp: u64) -> SystemTime {
    SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(timestamp)
}


pub struct SqlitePool (Arc<sqlx::Pool<sqlx::Sqlite>>);

impl SqlitePool {
    /// Creates a new SqlitePool with the given URL and size
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
        SqlitePool(Arc::new(pool))
    }

    /// Pool implementation for Sqlite
    pub fn get(&self) -> Arc<sqlx::Pool<sqlx::Sqlite>> {
        self.0.clone()
    }
}
