//! Utilities to integrate Rust services with Datadog using [`opentelemetry`],
//! [`tracing`], and other open source libraries.

pub mod formatter;
pub mod init;
pub mod shutdown;
pub mod tracer;

#[cfg(feature = "axum")]
pub mod axum;

pub use init::init;
