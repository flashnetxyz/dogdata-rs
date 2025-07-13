[![codecov](https://codecov.io/gh/flashnetxyz/dogdata-rs/graph/badge.svg?token=Q5Kt8eIuDK)](https://codecov.io/gh/flashnetxyz/dogdata-rs)

# dogdata-rs


# Configuration

The lib is configurable via environment variables as following:

| env var                | default value                                | description                                               |
|------------------------|----------------------------------------------|-----------------------------------------------------------|
| DD_SERVICE             | unnamed-rust-service                         | Datadog service name                                      |
| DD_TRACE_AGENT_URL     | http://localhost:8126                        | Datadog agent host                                        |
| RUST_LOG               | info                                         |                                                           |
| OTEL_LOG_LEVEL         | debug                                        |                                                           |


# Further Context and Rationale

## Inspiration

This crate is a fork of [datadog-tracing](https://github.com/will-bank/datadog-tracing) with updated dependencies and adjusted span base names for greater compatibility.

The original datadog-tracing lib was highly inspired on [ddtrace](https://github.com/Validus-Risk-Management/ddtrace) crate,
which is also a glue between tracing + opentelemetry + datadog.
The **main difference** is that it exportes using the `opentelemetry_otlp` exporter, and this one uses `opentelemetry_datadog`,
so there is no need to configure your datadog agent to receive traces via OTLP and the default datadog APM works as expected! 
