use tracing::Span;

use crate::execute::InstrumentedPool;

#[macro_export]
macro_rules! query_span_with_metadata {
    ($sql:expr, $metadata:expr) => {{
        tracing::span!(
            tracing::Level::INFO,
            "query",
            span.kind = "client",
            span.type = "sql",
            operation = "query",
            db.statement = %$sql,
            peer.hostname = %$metadata.host,
            peer.service = %$metadata.database,
            out.host = %$metadata.host,
            out.port = %$metadata.port,
            db.system = %$metadata.system,
            db.instance = %$metadata.database,
            db.name = %$metadata.database,
        )
    }};
}

pub(crate) fn query_span_with_metadata<P: InstrumentedPool>(sql: &str, pool: &P) -> Span {
    let metadata = pool.connection_info();
    crate::query_span_with_metadata!(sql, metadata)
}
