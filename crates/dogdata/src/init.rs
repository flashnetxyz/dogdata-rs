use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use std::env;
use tracing::Subscriber;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, Registry};

use crate::logs::build_logging_provider;
use crate::shutdown::Shutdown;
use crate::tracer::build_tracer_provider;

fn loglevel_filter_layer(_dd_enabled: bool) -> EnvFilter {
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

    // `otel::setup` set to debug to log detected resources, configuration read and infered
    let otel_log_level = env::var("OTEL_LOG_LEVEL").unwrap_or_else(|_| "debug".to_string());

    unsafe {
        env::set_var("RUST_LOG", format!("{log_level},otel={otel_log_level}"));
    }

    EnvFilter::from_default_env()
}

pub fn init() -> Shutdown {
    let tracer_provider = build_tracer_provider();
    let logger_provider = build_logging_provider();

    let otel_layer = OpenTelemetryTracingBridge::new(&logger_provider);
    tracing_subscriber::registry().with(otel_layer).init();

    Shutdown::new(Some((tracer_provider, logger_provider)))
}
