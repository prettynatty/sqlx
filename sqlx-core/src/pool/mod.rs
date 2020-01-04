//! **Pool** for SQLx database connections.

use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
    time::{Duration, Instant},
};

use async_std::sync::Sender;
use futures_util::future::FutureExt;
use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;

use crate::Database;
use crate::transaction::Transaction;
use crate::describe::Describe;
use crate::executor::Executor;

use self::inner::SharedPool;
pub use self::options::Builder;
use self::options::Options;

mod executor;
mod inner;
mod options;

/// A pool of database connections.
pub struct Pool<DB>
where
    DB: Database,
{
    inner: Arc<SharedPool<DB>>,
    pool_tx: Sender<Idle<DB>>,
}

pub struct PoolConnection<DB: Database> {
    raw: Option<Raw<DB>>,
    pool_tx: Sender<Idle<DB>>,
}

struct Raw<DB: Database> {
    inner: DB::Connection,
    created: Instant,
}

struct Idle<DB: Database> {
    raw: Raw<DB>,
    since: Instant,
}

impl<DB> Pool<DB>
where
    DB: Database,
    DB::Connection: crate::Connection<Database = DB>,
{
    /// Creates a connection pool with the default configuration.
    pub async fn new(url: &str) -> crate::Result<Self> {
        Self::builder().build(url).await
    }

    async fn with_options(url: &str, options: Options) -> crate::Result<Self> {
        let (inner, pool_tx) = SharedPool::new_arc(url, options).await?;

        Ok(Pool { inner, pool_tx })
    }

    /// Returns a [Builder] to configure a new connection pool.
    pub fn builder() -> Builder<DB> {
        Builder::new()
    }

    /// Retrieves a connection from the pool.
    ///
    /// Waits for at most the configured connection timeout before returning an error.
    pub async fn acquire(&self) -> crate::Result<PoolConnection<DB>> {
        self.inner.acquire().await.map(|conn| PoolConnection {
            raw: Some(conn),
            pool_tx: self.pool_tx.clone(),
        })
    }

    /// Attempts to retrieve a connection from the pool if there is one available.
    ///
    /// Returns `None` immediately if there are no idle connections available in the pool.
    pub fn try_acquire(&self) -> Option<PoolConnection<DB>> {
        self.inner.try_acquire().map(|conn| PoolConnection {
            raw: Some(conn),
            pool_tx: self.pool_tx.clone(),
        })
    }

    /// Retrieves a new connection and immediately begins a new transaction.
    pub async fn begin(&self) -> crate::Result<Transaction<PoolConnection<DB>>> {
        Ok(Transaction::new(0, self.acquire().await?).await?)
    }

    /// Ends the use of a connection pool. Prevents any new connections
    /// and will close all active connections when they are returned to the pool.
    ///
    /// Does not resolve until all connections are closed.
    pub async fn close(&self) {
        self.inner.close().await;
    }

    /// Returns the number of connections currently being managed by the pool.
    pub fn size(&self) -> u32 {
        self.inner.size()
    }

    /// Returns the number of idle connections.
    pub fn idle(&self) -> usize {
        self.inner.num_idle()
    }

    /// Returns the configured maximum pool size.
    pub fn max_size(&self) -> u32 {
        self.inner.options().max_size
    }

    /// Returns the maximum time spent acquiring a new connection before an error is returned.
    pub fn connect_timeout(&self) -> Duration {
        self.inner.options().connect_timeout
    }

    /// Returns the configured minimum idle connection count.
    pub fn min_size(&self) -> u32 {
        self.inner.options().min_size
    }

    /// Returns the configured maximum connection lifetime.
    pub fn max_lifetime(&self) -> Option<Duration> {
        self.inner.options().max_lifetime
    }

    /// Returns the configured idle connection timeout.
    pub fn idle_timeout(&self) -> Option<Duration> {
        self.inner.options().idle_timeout
    }
}

/// Returns a new [Pool] tied to the same shared connection pool.
impl<DB> Clone for Pool<DB>
where
    DB: Database,
{
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
            pool_tx: self.pool_tx.clone(),
        }
    }
}

const DEREF_ERR: &str = "(bug) connection already released to pool";

impl<DB: Database> Deref for PoolConnection<DB> {
    type Target = DB::Connection;

    fn deref(&self) -> &Self::Target {
        &self.raw.as_ref().expect(DEREF_ERR).inner
    }
}

impl<DB: Database> DerefMut for PoolConnection<DB> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.raw.as_mut().expect(DEREF_ERR).inner
    }
}

impl<DB> Executor for PoolConnection<DB>
where
    DB: Database
{
    type Database = DB;

    fn send<'e, 'q: 'e>(&'e mut self, commands: &'q str) -> BoxFuture<'e, crate::Result<()>> {
        self.deref_mut().send(commands)
    }

    fn execute<'e, 'q: 'e>(
        &'e mut self,
        query: &'q str,
        args: DB::Arguments,
    ) -> BoxFuture<'e, crate::Result<u64>> {
        self.deref_mut().execute(query, args)
    }

    fn fetch<'e, 'q: 'e>(
        &'e mut self,
        query: &'q str,
        args: DB::Arguments,
    ) -> BoxStream<'e, crate::Result<DB::Row>> {
        self.deref_mut().fetch(query, args)
    }

    fn fetch_optional<'e, 'q: 'e>(
        &'e mut self,
        query: &'q str,
        args: DB::Arguments,
    ) -> BoxFuture<'e, crate::Result<Option<DB::Row>>> {
        self.deref_mut().fetch_optional(query, args)
    }

    fn describe<'e, 'q: 'e>(
        &'e mut self,
        query: &'q str,
    ) -> BoxFuture<'e, crate::Result<Describe<Self::Database>>> {
        self.deref_mut().describe(query)
    }
}

impl<DB: Database> Drop for PoolConnection<DB> {
    fn drop(&mut self) {
        if let Some(conn) = self.raw.take() {
            self.pool_tx
                .send(Idle {
                    raw: conn,
                    since: Instant::now(),
                })
                .now_or_never()
                .expect("(bug) connection released into a full pool")
        }
    }
}
