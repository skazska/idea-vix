/// Common module for database infrastructure.

use std::{ pin::Pin, sync::Arc, time::SystemTime };

use crate::error::ModelError;

/// returns unix timestamp from SystemTime
pub fn to_unix_timestamp(system_time: SystemTime) -> u64 {
    system_time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

/// converts unix timestamp to SystemTime
pub fn from_unix_timestamp(timestamp: u64) -> SystemTime {
    SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(timestamp)
}

//***** SQLite */
pub type DbErr = sqlx::Error;


pub trait TrxTrait<T> {
    fn get_mut(&mut self) -> &mut T;
    async fn commit(self) -> Result<(), DbErr>;
    async fn rollback(self) -> Result<(), DbErr>;
}

pub struct Trx {
    transaction: sqlx::Transaction<'static, sqlx::Sqlite>,
}

impl Trx {
    pub fn new(transaction: sqlx::Transaction<'static, sqlx::Sqlite>) -> Self {
        Self { transaction }
    }
}

impl TrxTrait<sqlx::Transaction<'static, sqlx::Sqlite>> for Trx {
    fn get_mut(&mut self) -> &mut sqlx::Transaction<'static, sqlx::Sqlite> {
        &mut self.transaction
    }
    async fn commit(self) -> Result<(), DbErr> {
        self.transaction.commit().await
    }
    async fn rollback(self) -> Result<(), DbErr> {
        self.transaction.rollback().await
    }
}

pub trait QueriesIntId<'q> {
    type Id: Send + Sync + Unpin;

    fn get_id(&self, id: Self::Id) -> impl sqlx::Encode<'q, sqlx::Sqlite> + sqlx::Type<sqlx::Sqlite> + 'q;
}

pub struct TrxRun<F>
{
    trx: Trx,
    f: F
}


impl<F, R> TrxRun<F>
where
    F: AsyncFnOnce(&mut Trx) -> Result<R, ModelError>,
    R: Send,
{
    pub fn new(transaction: Trx, f: F) -> Self {
        Self { trx: transaction, f }
    }

    pub async fn run(self) -> Result<R, ModelError> {
        let mut tx = self.trx;
        let res = (self.f)(&mut tx).await;

        match res {
            Ok(val) => {
                tx.commit().await?;
                Ok(val)
            },
            Err(err) => {
                tx.rollback().await?;
                Err(err)
            }
        }
    }
}


pub struct TransactionStarter {
    pool: sqlx::Pool<sqlx::Sqlite>,
}

impl TransactionStarter {
    /// Starts a new transaction
    pub fn get_pool(&self) -> Arc<sqlx::Pool<sqlx::Sqlite>> {
        Arc::new(self.pool.clone())
    }

    pub fn run_in_transaction<'c, F, R>(&'c self, f: F) -> impl std::future::Future<Output=Result<R, ModelError>> + 'c
    where
        F: AsyncFnOnce(&mut Trx) -> Result<R, ModelError> + 'c,
        R: Send + 'c,
    {
        let pool = self.pool.clone();
        async move {
            let transaction = pool.begin().await?;
            TrxRun::new(Trx { transaction }, f).run().await
        }
    }

    // pub fn run_in_transaction<'c, F, R>(&'c self, f: F) -> impl std::future::Future<Output=Result<R, ModelError>> + 'c
    // where
    //     F: Fn(&mut Trx<'c>) -> Pin<Box<dyn Future<Output=Result<R, ModelError>> + 'c>> + 'c,
    //     R: Send + 'c,
    // {
    //     let pool = self.pool.clone();
    //     async move {
    //         let transaction = pool.begin().await?;
    //         TrxRun::new(transaction, f).run().await
    //     }
    // }
}

pub struct SqlitePool (Arc<TransactionStarter>);

impl SqlitePool {
    /// Creates a new SqlitePool with the given URL and size
    pub async fn new(url: &str, size: u8) -> Self {
        let opts = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(url)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal) // Use WAL mode for better concurrency
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
