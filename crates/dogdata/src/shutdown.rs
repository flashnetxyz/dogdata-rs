use opentelemetry_sdk::trace::SdkTracerProvider;

pub struct TracerShutdown {
    provider: Option<SdkTracerProvider>,
}

impl TracerShutdown {
    pub fn new(provider: Option<SdkTracerProvider>) -> Self {
        Self { provider }
    }

    pub fn shutdown(&self) {
        if let Some(provider) = &self.provider {
            let _ = provider.shutdown();
        }
    }
}
