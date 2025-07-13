use opentelemetry::Context;
use opentelemetry::InstrumentationScope;
pub use opentelemetry::trace::TraceId;
use opentelemetry_sdk::error::OTelSdkResult;
use opentelemetry_sdk::logs::LogProcessor;
use opentelemetry_sdk::logs::SdkLogRecord;
use opentelemetry_sdk::logs::SdkLoggerProvider;
use opentelemetry_stdout::LogExporter;

/// A custom span processor that enriches spans with baggage attributes. Baggage
/// information is not added automatically without this processor.
#[derive(Debug)]
struct EnrichWithBaggageLogProcessor;
impl LogProcessor for EnrichWithBaggageLogProcessor {
    fn emit(&self, _data: &mut SdkLogRecord, _instrumentation: &InstrumentationScope) {
        Context::map_current(|_cx| {
            // if let Some(propagation_data) = cx.get::<DatadogExtractData>() {
            //     data.add_attribute("dd_propagation_data", propagation_data.to_string());
            // }
        });
    }

    fn force_flush(&self) -> OTelSdkResult {
        Ok(())
    }

    fn shutdown(&self) -> OTelSdkResult {
        Ok(())
    }
}

pub fn build_logging_provider() -> SdkLoggerProvider {
    let logger_provider = SdkLoggerProvider::builder()
        .with_log_processor(EnrichWithBaggageLogProcessor)
        .with_simple_exporter(LogExporter::default())
        .build();

    logger_provider
}
