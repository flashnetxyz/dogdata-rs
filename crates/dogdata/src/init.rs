// MIT License
//
// Copyright (c) 2023 willbank
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::formatter::DatadogFormatter;
use crate::shutdown::TracerShutdown;
use crate::tracer::build_tracer;
use opentelemetry_sdk::trace::TraceError;
use std::env;
use tracing::Subscriber;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, Registry};

fn loglevel_filter_layer(dd_enabled: bool) -> EnvFilter {
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

    // `axum_tracing_opentelemetry` should be a level info to emit opentelemetry trace & span
    let axum_tracing_log_level = env::var("AXUM_TRACING_LOG_LEVEL").unwrap_or_else(|_| {
        if dd_enabled {
            "trace".to_string()
        } else {
            "off".to_string()
        }
    });

    // `otel::setup` set to debug to log detected resources, configuration read and infered
    let otel_log_level = env::var("OTEL_LOG_LEVEL").unwrap_or_else(|_| "debug".to_string());

    unsafe {
        env::set_var(
            "RUST_LOG",
            format!("{log_level},otel::tracing={axum_tracing_log_level},otel={otel_log_level}"),
        );
    }

    EnvFilter::from_default_env()
}

fn log_layer<S>(
    dd_enabled: bool,
    non_blocking: NonBlocking,
) -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    if dd_enabled {
        Box::new(
            tracing_subscriber::fmt::layer()
                .json()
                .event_format(DatadogFormatter)
                .with_writer(non_blocking),
        )
    } else {
        Box::new(tracing_subscriber::fmt::layer().with_writer(non_blocking))
    }
}

pub fn init() -> Result<(WorkerGuard, TracerShutdown), TraceError> {
    let (non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());

    let dd_enabled = env::var("DD_ENABLED").map(|s| s == "true").unwrap_or(false);

    let (telemetry_layer, provider) = if dd_enabled {
        let (tracer, provider) = build_tracer()?;
        (
            Some(tracing_opentelemetry::layer().with_tracer(tracer)),
            Some(provider),
        )
    } else {
        (None, None)
    };

    Registry::default()
        .with(loglevel_filter_layer(dd_enabled))
        .with(log_layer(dd_enabled, non_blocking))
        .with(telemetry_layer)
        .init();

    Ok((guard, TracerShutdown::new(provider)))
}
