use opentelemetry_datadog::ModelConfig;
use opentelemetry_sdk::trace::SpanData;

// Datadog uses some magic tags in their models. There is no recommended mapping defined in
// opentelemetry spec. Below is default mapping we gonna uses. Users can override it by providing
// their own implementations.
pub(super) fn default_service_name_mapping<'a>(
    _span: &'a SpanData,
    config: &'a ModelConfig,
) -> &'a str {
    config.service_name.as_str()
}

pub(super) fn default_name_mapping<'a>(span: &'a SpanData, _config: &'a ModelConfig) -> &'a str {
    span.instrumentation_scope.name()
}

pub(super) fn default_resource_mapping<'a>(
    span: &'a SpanData,
    _config: &'a ModelConfig,
) -> &'a str {
    span.name.as_ref()
}
