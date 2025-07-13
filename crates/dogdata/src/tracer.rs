use opentelemetry::Context;
use opentelemetry::KeyValue;
use opentelemetry::baggage::BaggageExt as _;
use opentelemetry::global::ObjectSafeSpan as _;
pub use opentelemetry::trace::TraceId;
use opentelemetry_sdk::error::OTelSdkResult;
use opentelemetry_sdk::trace::SdkTracerProvider;
use opentelemetry_sdk::trace::SpanProcessor;
use opentelemetry_stdout::SpanExporter;

/// A custom span processor that enriches spans with baggage attributes. Baggage
/// information is not added automatically without this processor.
#[derive(Debug)]
struct EnrichWithBaggageSpanProcessor;
impl SpanProcessor for EnrichWithBaggageSpanProcessor {
    fn force_flush(&self) -> OTelSdkResult {
        Ok(())
    }

    fn shutdown(&self) -> OTelSdkResult {
        Ok(())
    }

    fn on_start(&self, span: &mut opentelemetry_sdk::trace::Span, cx: &Context) {
        for (kk, vv) in cx.baggage().iter() {
            span.set_attribute(KeyValue::new(kk.clone(), vv.0.clone()));
        }
    }

    fn on_end(&self, _span: opentelemetry_sdk::trace::SpanData) {}
}

pub fn build_tracer_provider() -> SdkTracerProvider {
    let config = dd_trace::Config::builder().build();
    let tracer_provider_builder = SdkTracerProvider::builder()
        .with_span_processor(EnrichWithBaggageSpanProcessor)
        .with_simple_exporter(SpanExporter::default());

    datadog_opentelemetry::init_datadog(config, tracer_provider_builder)
}
