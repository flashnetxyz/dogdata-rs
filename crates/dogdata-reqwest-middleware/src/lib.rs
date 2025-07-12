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

//! Opentracing middleware implementation for [`reqwest_middleware`].
//!
//! Attach [`TracingMiddleware`] to your client to automatically trace HTTP requests.
//!
//! The simplest possible usage:
//! ```no_run
//! # use reqwest_middleware::Result;
//! use reqwest_middleware::{ClientBuilder};
//! use dogdata_reqwest_middleware::TracingMiddleware;
//!
//! # async fn example() -> Result<()> {
//! let reqwest_client = reqwest::Client::builder().build().unwrap();
//! let client = ClientBuilder::new(reqwest_client)
//!    // Insert the tracing middleware
//!    .with(TracingMiddleware::default())
//!    .build();
//!
//! let resp = client.get("https://truelayer.com").send().await.unwrap();
//! # Ok(())
//! # }
//! ```
//!
//! To customise the span names use [`OtelName`].
//! ```no_run
//! # use reqwest_middleware::Result;
//! use reqwest_middleware::{ClientBuilder, Extension};
//! use dogdata_reqwest_middleware::{
//!     TracingMiddleware, OtelName
//! };
//! # async fn example() -> Result<()> {
//! let reqwest_client = reqwest::Client::builder().build().unwrap();
//! let client = ClientBuilder::new(reqwest_client)
//!    // Inserts the extension before the request is started
//!    .with_init(Extension(OtelName("my-client".into())))
//!    // Makes use of that extension to specify the otel name
//!    .with(TracingMiddleware::default())
//!    .build();
//!
//! let resp = client.get("https://truelayer.com").send().await.unwrap();
//!
//! // Or specify it on the individual request (will take priority)
//! let resp = client.post("https://api.truelayer.com/payment")
//!     .with_extension(OtelName("POST /payment".into()))
//!    .send()
//!    .await
//!    .unwrap();
//! # Ok(())
//! # }
//! ```
//!
//! In this example we define a custom span builder to calculate the request time elapsed and we register the [`TracingMiddleware`].
//!
//! Note that Opentelemetry tracks start and stop already, there is no need to have a custom builder like this.
//! ```rust
//! use reqwest_middleware::Result;
//! use http::Extensions;
//! use reqwest::{Request, Response};
//! use reqwest_middleware::ClientBuilder;
//! use dogdata_reqwest_middleware::{
//!     default_on_request_end, reqwest_otel_span, ReqwestOtelSpanBackend, TracingMiddleware
//! };
//! use tracing::Span;
//! use std::time::{Duration, Instant};
//!
//! pub struct TimeTrace;
//!
//! impl ReqwestOtelSpanBackend for TimeTrace {
//!     fn on_request_start(req: &Request, extension: &mut Extensions) -> Span {
//!         extension.insert(Instant::now());
//!         reqwest_otel_span!(name="example-request", req, time_elapsed = tracing::field::Empty)
//!     }
//!
//!     fn on_request_end(span: &Span, outcome: &Result<Response>, extension: &mut Extensions) {
//!         let time_elapsed = extension.get::<Instant>().unwrap().elapsed().as_millis() as i64;
//!         default_on_request_end(span, outcome);
//!         span.record("time_elapsed", &time_elapsed);
//!     }
//! }
//!
//! let http = ClientBuilder::new(reqwest::Client::new())
//!     .with(TracingMiddleware::<TimeTrace>::new())
//!     .build();
//! ```

mod middleware;
mod otel;
mod reqwest_otel_span_builder;
pub use middleware::TracingMiddleware;
pub use reqwest_otel_span_builder::{
    DefaultSpanBackend, DisableOtelPropagation, ERROR_CAUSE_CHAIN, ERROR_MESSAGE,
    HTTP_REQUEST_METHOD, HTTP_RESPONSE_STATUS_CODE, OTEL_KIND, OTEL_NAME, OTEL_STATUS_CODE,
    OtelName, OtelPathNames, ReqwestOtelSpanBackend, SERVER_ADDRESS, SERVER_PORT,
    SpanBackendWithUrl, URL_FULL, URL_SCHEME, USER_AGENT_ORIGINAL, default_on_request_end,
    default_on_request_failure, default_on_request_success, default_span_name,
};

#[doc(hidden)]
pub mod reqwest_otel_span_macro;
