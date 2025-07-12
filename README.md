[![codecov](https://codecov.io/gh/flashnetxyz/dogdata-rs/graph/badge.svg?token=Q5Kt8eIuDK)](https://codecov.io/gh/flashnetxyz/dogdata-rs)

# dogdata-rs

## Inspiration

This crate is a fork of [@will-bank/datadog-tracing](https://github.com/will-bank/datadog-tracing) with updated dependencies and adjusted span base names for greater compatibility.

The original datadog-tracing lib was highly inspired on [ddtrace](https://github.com/Validus-Risk-Management/ddtrace) crate,
which is also a glue between tracing + opentelemetry + datadog.
The **main difference** is that it exportes using the `opentelemetry_otlp` exporter, and this one uses `opentelemetry_datadog`,
so there is no need to configure your datadog agent to receive traces via OTLP and the default datadog APM works as expected! 
