pub(crate) mod sqlx_otel_span_macro;

pub mod execute;
pub mod query_builder;

#[macro_export]
macro_rules! instrumented_query {
    ($sql:expr $(, $arg:expr)* $(,)?) => {{
        $crate::InstrumentedQueryBuilder {
            query: ::sqlx::query!($sql $(, $arg)*),
            sql: $sql,
        }
    }};
}

#[macro_export]
macro_rules! instrumented_query_as {
    ($out_ty:ty, $sql:expr $(, $arg:expr)* $(,)?) => {{
        $crate::InstrumentedQueryBuilder {
            query: ::sqlx::query_as!($out_ty, $sql $(, $arg)*),
            sql: $sql,
        }
    }};
}

#[macro_export]
macro_rules! instrumented_query_scalar {
    ($sql:expr $(, $arg:expr)* $(,)?) => {{
        $crate::InstrumentedQueryBuilder {
            query: ::sqlx::query_scalar!($sql $(, $arg)*),
            sql: $sql,
        }
    }};
}

#[macro_export]
macro_rules! instrumented_query_file {
    ($path:literal $(, $arg:expr)* $(,)?) => {{
        $crate::InstrumentedQueryBuilder {
            query: ::sqlx::query_file!($path $(, $arg)*),
            sql: include_str!($path),
        }
    }};
}

#[macro_export]
macro_rules! instrumented_query_file_as {
    ($out_ty:ty, $path:literal $(, $arg:expr)* $(,)?) => {{
        $crate::InstrumentedQueryBuilder {
            query: ::sqlx::query_file_as!($out_ty, $path $(, $arg)*),
            sql: include_str!($path),
        }
    }};
}

#[macro_export]
macro_rules! instrumented_query_file_scalar {
    ($path:literal $(, $arg:expr)* $(,)?) => {{
        $crate::InstrumentedQueryBuilder {
            query: ::sqlx::query_file_scalar!($path $(, $arg)*),
            sql: include_str!($path),
        }
    }};
}
