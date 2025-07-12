use sqlx::{Database, Executor};

use crate::execute::{InstrumentedExecute, InstrumentedFetch, InstrumentedPool};

pub struct InstrumentedQueryBuilder<Q> {
    pub query: Q,
    pub sql: &'static str,
}

impl<'q, DB, A> InstrumentedQueryBuilder<sqlx::query::Query<'q, DB, A>>
where
    DB: Database,
    A: 'q + Send + sqlx::IntoArguments<'q, DB>,
{
    pub async fn execute<P>(self, pool: &P) -> Result<DB::QueryResult, sqlx::Error>
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        for<'c> &'c P: Executor<'c, Database = DB>,
    {
        self.query.execute_instrumented(pool, self.sql).await
    }

    pub async fn fetch_one<P>(self, pool: &P) -> Result<DB::Row, sqlx::Error>
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        for<'c> &'c P: Executor<'c, Database = DB>,
    {
        self.query.fetch_one_instrumented(pool, self.sql).await
    }

    pub async fn fetch_optional<P>(self, pool: &P) -> Result<Option<DB::Row>, sqlx::Error>
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        for<'c> &'c P: Executor<'c, Database = DB>,
    {
        self.query.fetch_optional_instrumented(pool, self.sql).await
    }

    pub async fn fetch_all<P>(self, pool: &P) -> Result<Vec<DB::Row>, sqlx::Error>
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        for<'c> &'c P: Executor<'c, Database = DB>,
    {
        self.query.fetch_all_instrumented(pool, self.sql).await
    }
}

// Implementation for sqlx::query::Map<'q, DB, F, A> (from query_as! macro)
impl<'q, DB, F, A, O> InstrumentedQueryBuilder<sqlx::query::Map<'q, DB, F, A>>
where
    DB: Database,
    F: FnMut(DB::Row) -> Result<O, sqlx::Error> + Send,
    O: Send + Unpin,
    A: 'q + Send + sqlx::IntoArguments<'q, DB>,
{
    pub async fn fetch_one<P>(self, pool: &P) -> Result<O, sqlx::Error>
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        for<'c> &'c P: Executor<'c, Database = DB>,
    {
        self.query.fetch_one_instrumented(pool, self.sql).await
    }

    pub async fn fetch_optional<P>(self, pool: &P) -> Result<Option<O>, sqlx::Error>
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        for<'c> &'c P: Executor<'c, Database = DB>,
    {
        self.query.fetch_optional_instrumented(pool, self.sql).await
    }

    pub async fn fetch_all<P>(self, pool: &P) -> Result<Vec<O>, sqlx::Error>
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        for<'c> &'c P: Executor<'c, Database = DB>,
    {
        self.query.fetch_all_instrumented(pool, self.sql).await
    }
}

// Implementation for sqlx::query::QueryScalar<'q, DB, O, A> (from query_scalar! macro)
impl<'q, DB, O, A> InstrumentedQueryBuilder<sqlx::query::QueryScalar<'q, DB, O, A>>
where
    DB: Database,
    O: Send + Unpin + for<'r> sqlx::Decode<'r, DB> + sqlx::Type<DB>,
    A: 'q + Send + sqlx::IntoArguments<'q, DB>,
    usize: sqlx::ColumnIndex<DB::Row>,
{
    pub async fn fetch_one<P>(self, pool: &P) -> Result<O, sqlx::Error>
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        for<'c> &'c P: Executor<'c, Database = DB>,
    {
        self.query.fetch_one_instrumented(pool, self.sql).await
    }

    pub async fn fetch_optional<P>(self, pool: &P) -> Result<Option<O>, sqlx::Error>
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        for<'c> &'c P: Executor<'c, Database = DB>,
    {
        self.query.fetch_optional_instrumented(pool, self.sql).await
    }

    pub async fn fetch_all<P>(self, pool: &P) -> Result<Vec<O>, sqlx::Error>
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        for<'c> &'c P: Executor<'c, Database = DB>,
    {
        self.query.fetch_all_instrumented(pool, self.sql).await
    }
}
