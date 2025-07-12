//! Trace and layer builders to export traces to the Datadog agent.
//!
//! This module contains a function that builds a tracer with an exporter
//! to send traces to the Datadog agent in batches over gRPC.
//!
//! It also contains a convenience function to build a layer with the tracer.

use opentelemetry::InstrumentationScope;
use opentelemetry::global;
use opentelemetry::trace::TraceError;
pub use opentelemetry::trace::TraceId;
use opentelemetry::trace::TraceResult;
use opentelemetry::trace::TracerProvider;
use opentelemetry_sdk::runtime;
use opentelemetry_sdk::trace::span_processor_with_async_runtime;
use opentelemetry_sdk::trace::{self, SdkTracerProvider};
use opentelemetry_sdk::trace::{RandomIdGenerator, Sampler, Tracer};
use opentelemetry_semantic_conventions as semcov;
use std::env;
use std::time::Duration;
use tracing::Subscriber;
use tracing_opentelemetry::{OpenTelemetryLayer, PreSampledTracer};
use tracing_subscriber::registry::LookupSpan;

pub fn build_tracer_provider() -> SdkTracerProvider {
    let config = dd_trace::Config::builder().build();

    let tracer_provider = datadog_opentelemetry::init_datadog(
        config,
        SdkTracerProvider::builder(),
    );

    tracer_provider
}

pub fn build_tracer() -> (Tracer, SdkTracerProvider) {
    let provider = build_tracer_provider();

    let scope = InstrumentationScope::builder(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
        .with_schema_url(semcov::SCHEMA_URL)
        .with_attributes(None)
        .build();

    let tracer = provider.tracer_with_scope(scope);

    (tracer, provider)
}

pub fn build_layer<S>() -> OpenTelemetryLayer<S, Tracer>
where
    Tracer: opentelemetry::trace::Tracer + PreSampledTracer + 'static,
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    let (tracer, _) = build_tracer();
    tracing_opentelemetry::layer().with_tracer(tracer)
}
