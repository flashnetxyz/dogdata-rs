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

use http::Extensions;
use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next, Result};
use tracing::Instrument;

use crate::{DefaultSpanBackend, ReqwestOtelSpanBackend};

/// Middleware for tracing requests using the current Opentelemetry Context.
pub struct TracingMiddleware<S: ReqwestOtelSpanBackend> {
    span_backend: std::marker::PhantomData<S>,
}

impl<S: ReqwestOtelSpanBackend> TracingMiddleware<S> {
    pub fn new() -> TracingMiddleware<S> {
        TracingMiddleware {
            span_backend: Default::default(),
        }
    }
}

impl<S: ReqwestOtelSpanBackend> Clone for TracingMiddleware<S> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl Default for TracingMiddleware<DefaultSpanBackend> {
    fn default() -> Self {
        TracingMiddleware::new()
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
impl<ReqwestOtelSpan> Middleware for TracingMiddleware<ReqwestOtelSpan>
where
    ReqwestOtelSpan: ReqwestOtelSpanBackend + Sync + Send + 'static,
{
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        let request_span = ReqwestOtelSpan::on_request_start(&req, extensions);

        let outcome_future = async {
            let req = if extensions.get::<crate::DisableOtelPropagation>().is_none() {
                // Adds tracing headers to the given request to propagate the OpenTelemetry context to downstream revivers of the request.
                // Spans added by downstream consumers will be part of the same trace.
                crate::otel::inject_opentelemetry_context_into_request(req)
            } else {
                req
            };

            // Run the request
            let outcome = next.run(req, extensions).await;
            ReqwestOtelSpan::on_request_end(&request_span, &outcome, extensions);
            outcome
        };

        outcome_future.instrument(request_span.clone()).await
    }
}
