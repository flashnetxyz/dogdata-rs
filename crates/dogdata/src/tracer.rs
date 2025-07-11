//! Trace and layer builders to export traces to the Datadog agent.
//!
//! This module contains a function that builds a tracer with an exporter
//! to send traces to the Datadog agent in batches over gRPC.
//!
//! It also contains a convenience function to build a layer with the tracer.
use opentelemetry::InstrumentationScope;
use opentelemetry::global;
pub use opentelemetry::trace::TraceId;
use opentelemetry::trace::TracerProvider;
use opentelemetry_datadog::{ApiVersion, DatadogPropagator};
use opentelemetry_sdk::trace::{self, SdkTracerProvider};
use opentelemetry_sdk::trace::{RandomIdGenerator, Sampler, TraceError, TraceResult, Tracer};
use opentelemetry_semantic_conventions as semcov;
use std::env;
use std::time::Duration;
use tracing::Subscriber;
use tracing_opentelemetry::{OpenTelemetryLayer, PreSampledTracer};
use tracing_subscriber::registry::LookupSpan;

pub fn build_tracer_provider() -> TraceResult<SdkTracerProvider> {
    let service_name = env::var("DD_SERVICE")
        .map_err(|_| <&str as Into<TraceError>>::into("missing DD_SERVICE"))?;

    let dd_host = env::var("DD_AGENT_HOST").unwrap_or("localhost".to_string());
    let dd_port = env::var("DD_AGENT_PORT")
        .ok()
        .and_then(|it| it.parse::<i32>().ok())
        .unwrap_or(8126);

    // disabling connection reuse with dd-agent to avoid "connection closed from server" errors
    let dd_http_client = reqwest::ClientBuilder::new()
        .pool_idle_timeout(Duration::from_millis(1))
        .build()
        .expect("Could not init datadog http_client");

    let mut config = trace::Config::default();
    config.sampler = Box::new(Sampler::AlwaysOn);
    config.id_generator = Box::new(RandomIdGenerator::default());

    let provider = opentelemetry_datadog::new_pipeline()
        .with_http_client(dd_http_client)
        .with_service_name(service_name)
        .with_api_version(ApiVersion::Version05)
        .with_agent_endpoint(format!("http://{dd_host}:{dd_port}"))
        .with_trace_config(config)
        .install_batch()?;
    global::set_tracer_provider(provider.clone());

    global::set_text_map_propagator(DatadogPropagator::default());

    Ok(provider)
}

pub fn build_tracer() -> TraceResult<(Tracer, SdkTracerProvider)> {
    let provider = build_tracer_provider()?;

    let scope = InstrumentationScope::builder("opentelemetry-datadog")
        .with_version(env!("CARGO_PKG_VERSION"))
        .with_schema_url(semcov::SCHEMA_URL)
        .with_attributes(None)
        .build();

    let tracer = provider.tracer_with_scope(scope);

    Ok((tracer, provider))
}

pub fn build_layer<S>() -> TraceResult<OpenTelemetryLayer<S, Tracer>>
where
    Tracer: opentelemetry::trace::Tracer + PreSampledTracer + 'static,
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    let (tracer, _) = build_tracer()?;
    Ok(tracing_opentelemetry::layer().with_tracer(tracer))
}
