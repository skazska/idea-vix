/// Common module for database infrastructure.

use std::{ pin::Pin, sync::Arc, time::SystemTime };
use sqlx::sqlite::SqliteJournalMode;

use crate::error::ModelError;

/// returns unix timestamp from SystemTime
pub fn to_unix_timestamp(system_time: SystemTime) -> u64 {
    system_time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

/// converts unix timestamp to SystemTime
pub fn from_unix_timestamp(timestamp: u64) -> SystemTime {
    SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(timestamp)
}

pub trait QueriesIntId {
    type Id: Send + Sync + Unpin;

    fn get_id(&self, id: Self::Id) -> impl sqlx::Encode<'_, sqlx::Sqlite> + sqlx::Type<sqlx::Sqlite> + '_;
}


pub trait TransactionHandler {
    type Error;
    type ModelError;
    type Transaction;

    fn get(&mut self) -> &mut Self::Transaction;

    async fn commit(self) -> Result<(), Self::ModelError>;

    async fn rollback(self) -> Result<(), Self::ModelError>;

    async fn exec_box<R>(&mut self, func: Box<dyn Fn(&mut Self::Transaction) -> Pin<Box<dyn Future<Output=Result<R, Self::ModelError>> + '_>>>) -> Result<R, Self::ModelError>
    where
        Self: Sized,
    {
        func(self.get()).await
    }

    async fn exec<R>(&mut self, func: impl AsyncFnOnce(&mut Self::Transaction) -> Result<R, Self::ModelError> + '_) -> Result<R, Self::ModelError>
    {
        func(self.get()).await
    }


    async fn end<R>(self, result: Result<R, Self::ModelError>) -> Result<R, Self::ModelError>
    where
        Self: Sized,
    {
        match result {
            Ok(val) => {
                self.commit().await?;
                Ok(val)
            },
            Err(err) => {
                self.rollback().await?;
                Err(err)
            }
        }
    }

    async fn run_box<R>(self, func: Box<dyn Fn(&mut Self::Transaction) -> Pin<Box<dyn Future<Output=Result<R, Self::ModelError>> + '_>>> ) -> Result<R, Self::ModelError>
    where
        Self: Sized,
    {
        let mut tx = self;
        let res = tx.exec_box(func).await;
        tx.end(res).await
    }

    async fn run<R>(self, func: impl AsyncFnOnce(&mut Self::Transaction) -> Result<R, Self::ModelError> + '_) -> Result<R, Self::ModelError>
    where
        Self: Sized,
    {
        let mut tx = self;
        let res = tx.exec(func).await;
        tx.end(res).await
    } 
}

pub struct SqliteTransaction<'t> {
    transaction: sqlx::Transaction<'t, sqlx::Sqlite>,
}

impl<'t> SqliteTransaction<'t> {
    pub fn new(transaction: sqlx::Transaction<'t, sqlx::Sqlite>) -> Self {
        Self { transaction }
    }
}

impl<'t> TransactionHandler for SqliteTransaction<'t> {
    type Error = sqlx::Error;
    type ModelError = ModelError;
    type Transaction = sqlx::Transaction<'t, sqlx::Sqlite>;

    fn get(&mut self) -> &mut Self::Transaction {
        &mut self.transaction
    }

    async fn commit(self) -> Result<(), Self::ModelError> {
        self.transaction.commit().await.map_err(|e| e.into())
    }

    async fn rollback(self) -> Result<(), Self::ModelError> {
        self.transaction.rollback().await.map_err(|e| e.into())
    }
}

pub struct TransactionStarter {
    pool: sqlx::Pool<sqlx::Sqlite>,
}

impl TransactionStarter {
    /// Starts a new transaction
    pub async fn begin<'t>(&self) -> Result<SqliteTransaction<'t>, sqlx::Error> {
        let transaction = self.pool.begin().await?;
        Ok(SqliteTransaction::new(transaction))
    }

    pub fn get_pool(&self) -> Arc<sqlx::Pool<sqlx::Sqlite>> {
        Arc::new(self.pool.clone())
    }
}

pub struct SqlitePool (Arc<TransactionStarter>);

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

        SqlitePool(Arc::new(TransactionStarter {
            pool
        }))
    }

    /// Pool implementation for Sqlite
    pub fn get(&self) -> Arc<sqlx::Pool<sqlx::Sqlite>> {
        self.0.get_pool()
    }

    /// Returns new transaction starter
    pub fn get_transaction_starter(&self) -> Arc<TransactionStarter> {
        self.0.clone()
    }
}

/// Build a comma-separated SQL SET clause by evaluating boolean conditions.
///
/// Example:
/// let set = sqlx_build_set!(
///     item.name.is_some() => "name = ?",
///     item.description.is_some() => "description = ?",
///     item.icon.is_some() => "icon = ?",
///     item.is_public.is_some() => "is_public = ?"
/// );
/// if set.is_empty() { /* error: no fields */ }
/// TODO extend to handle binding values
#[macro_export]
macro_rules! sqlx_build_set {
    ( $( $cond:expr => $frag:expr ),+ $(,)? ) => {{
        let mut parts = ::std::vec::Vec::new();
        $( if $cond { parts.push($frag); } )+
        parts.join(", ")
    }}
}

#[cfg(test)]
mod tests {
    #[test]
    fn build_set_none() {
        let set = crate::sqlx_build_set!( false => "name = ?", false => "icon = ?" );
        assert!(set.is_empty());
    }

    #[test]
    fn build_set_some() {
        let set = crate::sqlx_build_set!( true => "name = ?", false => "icon = ?", true => "is_public = ?" );
        assert_eq!(set, "name = ?, is_public = ?");
    }
}
