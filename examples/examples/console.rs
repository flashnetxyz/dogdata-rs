#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        std::env::set_var("RUST_LOG", "trace");
    }

    let (_guard, shutdown) = dogdata::init()?;

    tracing::trace!("This is a trace message");
    tracing::debug!("This is a debug message");
    tracing::info!("This is an info message");
    tracing::warn!("This is a warn message");
    tracing::error!("This is an error message");

    shutdown.shutdown();

    Ok(())
}
