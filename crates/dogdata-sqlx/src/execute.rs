#[cfg(feature = "mysql")]
use sqlx::MySqlPool;
#[cfg(feature = "postgres")]
use sqlx::PgPool;
use sqlx::{Database, Execute, Executor};
use tracing::Instrument;

use crate::sqlx_otel_span_macro::query_span_with_metadata;

#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub system: &'static str,
}

pub trait InstrumentedPool: Sized {
    type Database: Database;

    fn connection_info(&self) -> ConnectionInfo;

    fn as_executor(&self) -> &Self;
}

#[cfg(feature = "postgres")]
impl InstrumentedPool for PgPool {
    type Database = sqlx::Postgres;

    fn connection_info(&self) -> ConnectionInfo {
        let options = self.connect_options();
        ConnectionInfo {
            host: options.get_host().to_string(),
            port: options.get_port(),
            database: options.get_database().unwrap_or("postgres").to_string(),
            system: "postgresql",
        }
    }

    fn as_executor(&self) -> &Self {
        self
    }
}

#[cfg(feature = "mysql")]
impl InstrumentedPool for MySqlPool {
    type Database = sqlx::MySql;

    fn connection_info(&self) -> ConnectionInfo {
        let options = self.connect_options();
        ConnectionInfo {
            host: options.get_host().to_string(),
            port: options.get_port(),
            database: options.get_database().unwrap_or("mysql").to_string(),
            system: "mysql",
        }
    }

    fn as_executor(&self) -> &Self {
        self
    }
}

pub trait InstrumentedFetch<'q, DB: Database>: Sized + Send {
    type Output: Send;

    fn fetch_one_instrumented<P>(
        self,
        pool: &P,
        sql: impl AsRef<str> + Send,
    ) -> impl Future<Output = Result<Self::Output, sqlx::Error>> + Send
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        for<'c> &'c P: Executor<'c, Database = DB>;

    fn fetch_optional_instrumented<P>(
        self,
        pool: &P,
        sql: impl AsRef<str> + Send,
    ) -> impl Future<Output = Result<Option<Self::Output>, sqlx::Error>> + Send
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        for<'c> &'c P: Executor<'c, Database = DB>;

    fn fetch_all_instrumented<P>(
        self,
        pool: &P,
        sql: impl AsRef<str> + Send,
    ) -> impl Future<Output = Result<Vec<Self::Output>, sqlx::Error>> + Send
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        for<'c> &'c P: Executor<'c, Database = DB>;
}

pub trait InstrumentedExecute<'q, DB: Database>: Sized + Execute<'q, DB> + Send {
    fn execute_instrumented<P>(
        self,
        pool: &P,
        sql: impl AsRef<str> + Send,
    ) -> impl Future<Output = Result<DB::QueryResult, sqlx::Error>> + Send
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        for<'c> &'c P: Executor<'c, Database = DB>;
}

impl<'q, DB, A> InstrumentedFetch<'q, DB> for sqlx::query::Query<'q, DB, A>
where
    DB: Database,
    A: 'q + Send + sqlx::IntoArguments<'q, DB>,
{
    type Output = DB::Row;

    async fn fetch_one_instrumented<P>(
        self,
        pool: &P,
        sql: impl AsRef<str> + Send,
    ) -> Result<Self::Output, sqlx::Error>
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        for<'c> &'c P: Executor<'c, Database = DB>,
    {
        let span = query_span_with_metadata(sql.as_ref(), pool);
        self.fetch_one(pool.as_executor()).instrument(span).await
    }

    async fn fetch_optional_instrumented<P>(
        self,
        pool: &P,
        sql: impl AsRef<str> + Send,
    ) -> Result<Option<Self::Output>, sqlx::Error>
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        for<'c> &'c P: Executor<'c, Database = DB>,
    {
        let span = query_span_with_metadata(sql.as_ref(), pool);
        self.fetch_optional(pool.as_executor())
            .instrument(span)
            .await
    }

    async fn fetch_all_instrumented<P>(
        self,
        pool: &P,
        sql: impl AsRef<str> + Send,
    ) -> Result<Vec<Self::Output>, sqlx::Error>
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        for<'c> &'c P: Executor<'c, Database = DB>,
    {
        let span = query_span_with_metadata(sql.as_ref(), pool);
        self.fetch_all(pool.as_executor()).instrument(span).await
    }
}

impl<'q, DB, F, A, O> InstrumentedFetch<'q, DB> for sqlx::query::Map<'q, DB, F, A>
where
    DB: Database,
    F: FnMut(DB::Row) -> Result<O, sqlx::Error> + Send,
    O: Send + Unpin,
    A: 'q + Send + sqlx::IntoArguments<'q, DB>,
{
    type Output = O;

    async fn fetch_one_instrumented<P>(
        self,
        pool: &P,
        sql: impl AsRef<str> + Send,
    ) -> Result<Self::Output, sqlx::Error>
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        for<'c> &'c P: Executor<'c, Database = DB>,
    {
        let span = query_span_with_metadata(sql.as_ref(), pool);
        self.fetch_one(pool.as_executor()).instrument(span).await
    }

    async fn fetch_optional_instrumented<P>(
        self,
        pool: &P,
        sql: impl AsRef<str> + Send,
    ) -> Result<Option<Self::Output>, sqlx::Error>
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        for<'c> &'c P: Executor<'c, Database = DB>,
    {
        let span = query_span_with_metadata(sql.as_ref(), pool);
        self.fetch_optional(pool.as_executor())
            .instrument(span)
            .await
    }

    async fn fetch_all_instrumented<P>(
        self,
        pool: &P,
        sql: impl AsRef<str> + Send,
    ) -> Result<Vec<Self::Output>, sqlx::Error>
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        for<'c> &'c P: Executor<'c, Database = DB>,
    {
        let span = query_span_with_metadata(sql.as_ref(), pool);
        self.fetch_all(pool.as_executor()).instrument(span).await
    }
}

impl<'q, DB, O, A> InstrumentedFetch<'q, DB> for sqlx::query::QueryScalar<'q, DB, O, A>
where
    DB: Database,
    O: Send + Unpin + for<'r> sqlx::Decode<'r, DB> + sqlx::Type<DB>,
    A: 'q + Send + sqlx::IntoArguments<'q, DB>,
    usize: sqlx::ColumnIndex<DB::Row>,
{
    type Output = O;

    async fn fetch_one_instrumented<P>(
        self,
        pool: &P,
        sql: impl AsRef<str> + Send,
    ) -> Result<Self::Output, sqlx::Error>
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        for<'c> &'c P: Executor<'c, Database = DB>,
    {
        let span = query_span_with_metadata(sql.as_ref(), pool);
        self.fetch_one(pool.as_executor()).instrument(span).await
    }

    async fn fetch_optional_instrumented<P>(
        self,
        pool: &P,
        sql: impl AsRef<str> + Send,
    ) -> Result<Option<Self::Output>, sqlx::Error>
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        for<'c> &'c P: Executor<'c, Database = DB>,
    {
        let span = query_span_with_metadata(sql.as_ref(), pool);
        self.fetch_optional(pool.as_executor())
            .instrument(span)
            .await
    }

    async fn fetch_all_instrumented<P>(
        self,
        pool: &P,
        sql: impl AsRef<str> + Send,
    ) -> Result<Vec<Self::Output>, sqlx::Error>
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        for<'c> &'c P: Executor<'c, Database = DB>,
    {
        let span = query_span_with_metadata(sql.as_ref(), pool);
        self.fetch_all(pool.as_executor()).instrument(span).await
    }
}

impl<'q, DB, A> InstrumentedExecute<'q, DB> for sqlx::query::Query<'q, DB, A>
where
    DB: Database,
    A: 'q + Send + sqlx::IntoArguments<'q, DB>,
{
    async fn execute_instrumented<P>(
        self,
        pool: &P,
        sql: impl AsRef<str> + Send,
    ) -> Result<DB::QueryResult, sqlx::Error>
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        for<'c> &'c P: Executor<'c, Database = DB>,
    {
        let span = query_span_with_metadata(sql.as_ref(), pool);
        self.execute(pool.as_executor()).instrument(span).await
    }
}
