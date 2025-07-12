use dogdata_reqwest_middleware::{SpanBackendWithUrl, TracingMiddleware};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_guard, shutdown) = dogdata::init()?;

    let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
        .with(TracingMiddleware::<SpanBackendWithUrl>::new())
        .build();

    let response = client
        .get("https://api.sparkscan.io/v1/stats/tpv")
        .send()
        .await?;

    tracing::info!("Response: {:?}", response);

    shutdown.shutdown();

    Ok(())
}
