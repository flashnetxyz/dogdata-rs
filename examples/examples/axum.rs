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

use dogdata::axum::{OtelAxumLayer, OtelInResponseLayer};
use std::net::SocketAddr;
use std::time::Duration;

use axum::{Router, routing::get};
use dogdata::axum::shutdown_signal;
use tokio::net::TcpListener;
use tower_http::timeout::TimeoutLayer;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shutdown = dogdata::init();

    let app = Router::new()
        .route("/", get(root))
        // include trace context as header into the response
        .layer(OtelInResponseLayer)
        //start OpenTelemetry trace on incoming request
        .layer((
            OtelAxumLayer::default(),
            // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
            // requests don't hang forever.
            TimeoutLayer::new(Duration::from_secs(90)),
        ))
        .route("/health", get(health));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3025));
    let listener = TcpListener::bind(addr).await?;

    info!("listening on {}", addr);
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    shutdown.shutdown()?;

    Ok(())
}

async fn root() -> &'static str {
    do_something().await;
    "Hello, World!"
}

#[tracing::instrument]
async fn do_something() {
    tokio::time::sleep(Duration::from_millis(120)).await;
    do_something_else().await;
    tracing::info!("in the middle of doing something");
    tokio::time::sleep(Duration::from_millis(10)).await;
    do_something_else().await;
    tokio::time::sleep(Duration::from_millis(20)).await;
}

#[tracing::instrument]
async fn do_something_else() {
    tokio::time::sleep(Duration::from_millis(40)).await;
}

async fn health() -> &'static str {
    "healthy"
}
