// MIT License
//
// Copyright (c) 2021 TrueLayer
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

use reqwest::Request;
use reqwest::header::{HeaderName, HeaderValue};
use std::str::FromStr;
use tracing::Span;

/// Injects the given OpenTelemetry Context into a reqwest::Request headers to allow propagation downstream.
pub fn inject_opentelemetry_context_into_request(mut request: Request) -> Request {
    #[cfg(feature = "opentelemetry_0_30")]
    opentelemetry_0_30_pkg::global::get_text_map_propagator(|injector| {
        use tracing_opentelemetry_0_31_pkg::OpenTelemetrySpanExt;
        let context = Span::current().context();
        injector.inject_context(&context, &mut RequestCarrier::new(&mut request))
    });

    request
}

// "traceparent" => https://www.w3.org/TR/trace-context/#trace-context-http-headers-format

/// Injector used via opentelemetry propagator to tell the extractor how to insert the "traceparent" header value
/// This will allow the propagator to inject opentelemetry context into a standard data structure. Will basically
/// insert a "traceparent" string value "{version}-{trace_id}-{span_id}-{trace-flags}" of the spans context into the headers.
/// Listeners can then re-hydrate the context to add additional spans to the same trace.
struct RequestCarrier<'a> {
    request: &'a mut Request,
}

impl<'a> RequestCarrier<'a> {
    pub fn new(request: &'a mut Request) -> Self {
        RequestCarrier { request }
    }
}

impl RequestCarrier<'_> {
    fn set_inner(&mut self, key: &str, value: String) {
        let header_name = HeaderName::from_str(key).expect("Must be header name");
        let header_value = HeaderValue::from_str(&value).expect("Must be a header value");
        self.request.headers_mut().insert(header_name, header_value);
    }
}

#[cfg(feature = "opentelemetry_0_30")]
impl opentelemetry_0_30_pkg::propagation::Injector for RequestCarrier<'_> {
    fn set(&mut self, key: &str, value: String) {
        self.set_inner(key, value)
    }
}

#[cfg(test)]
mod test {
    use std::sync::OnceLock;

    use crate::{DisableOtelPropagation, TracingMiddleware};
    use reqwest::Response;
    use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Extension};
    use tracing::{Instrument, Level, info_span};

    use tracing_subscriber::{Registry, filter, layer::SubscriberExt};
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers::any};

    async fn make_echo_request_in_otel_context(client: ClientWithMiddleware) -> Response {
        static TELEMETRY: OnceLock<()> = OnceLock::new();

        TELEMETRY.get_or_init(|| {
            let subscriber = Registry::default().with(
                filter::Targets::new()
                    .with_target("dogdata_reqwest_middleware::otel::test", Level::DEBUG),
            );

            #[cfg(feature = "opentelemetry_0_30")]
            let subscriber = {
                use opentelemetry_0_30_pkg::trace::TracerProvider;

                let provider = opentelemetry_sdk_0_30::trace::SdkTracerProvider::builder().build();

                let tracer = provider.tracer("reqwest");
                let _ = opentelemetry_0_30_pkg::global::set_tracer_provider(provider);
                opentelemetry_0_30_pkg::global::set_text_map_propagator(
                    opentelemetry_sdk_0_30::propagation::TraceContextPropagator::new(),
                );

                let telemetry = tracing_opentelemetry_0_31_pkg::layer().with_tracer(tracer);
                subscriber.with(telemetry)
            };

            tracing::subscriber::set_global_default(subscriber).unwrap();
        });

        // Mock server - sends all request headers back in the response
        let server = MockServer::start().await;
        Mock::given(any())
            .respond_with(|req: &wiremock::Request| {
                req.headers
                    .iter()
                    .fold(ResponseTemplate::new(200), |resp, (k, v)| {
                        resp.append_header(k.clone(), v.clone())
                    })
            })
            .mount(&server)
            .await;

        client
            .get(server.uri())
            .send()
            .instrument(info_span!("some_span"))
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn tracing_middleware_propagates_otel_data_even_when_the_span_is_disabled() {
        let client = ClientBuilder::new(reqwest::Client::new())
            .with(TracingMiddleware::default())
            .build();

        let resp = make_echo_request_in_otel_context(client).await;

        assert!(
            resp.headers().contains_key("traceparent"),
            "by default, the tracing middleware will propagate otel contexts"
        );
    }

    #[tokio::test]
    async fn context_no_propagated() {
        let client = ClientBuilder::new(reqwest::Client::new())
            .with_init(Extension(DisableOtelPropagation))
            .with(TracingMiddleware::default())
            .build();

        let resp = make_echo_request_in_otel_context(client).await;

        assert!(
            !resp.headers().contains_key("traceparent"),
            "request should not contain traceparent if context propagation is disabled"
        );
    }
}
