use sqlx::{Database, Executor};

use crate::execute::{InstrumentedExecute, InstrumentedFetch, InstrumentedPool};

pub struct InstrumentedQueryBuilder<Q> {
    pub query: Q,
    pub sql: &'static str,
}

impl<Q> InstrumentedQueryBuilder<Q> {
    pub async fn execute<P, DB>(self, pool: &P) -> Result<DB::QueryResult, sqlx::Error>
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        DB: Database,
        Q: InstrumentedExecute<'static, DB>,
        for<'c> &'c P: Executor<'c, Database = DB>,
    {
        self.query.execute_instrumented(pool, self.sql).await
    }

    pub async fn fetch_one<P, DB>(self, pool: &P) -> Result<Q::Output, sqlx::Error>
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        DB: Database,
        Q: InstrumentedFetch<'static, DB>,
        for<'c> &'c P: Executor<'c, Database = DB>,
    {
        self.query.fetch_one_instrumented(pool, self.sql).await
    }

    pub async fn fetch_optional<P, DB>(self, pool: &P) -> Result<Option<Q::Output>, sqlx::Error>
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        DB: Database,
        Q: InstrumentedFetch<'static, DB>,
        for<'c> &'c P: Executor<'c, Database = DB>,
    {
        self.query.fetch_optional_instrumented(pool, self.sql).await
    }

    pub async fn fetch_all<P, DB>(self, pool: &P) -> Result<Vec<Q::Output>, sqlx::Error>
    where
        P: InstrumentedPool<Database = DB> + Send + Sync,
        DB: Database,
        Q: InstrumentedFetch<'static, DB>,
        for<'c> &'c P: Executor<'c, Database = DB>,
    {
        self.query.fetch_all_instrumented(pool, self.sql).await
    }
}
