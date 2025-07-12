use opentelemetry_sdk::{error::OTelSdkError, trace::SdkTracerProvider};

pub struct TracerShutdown {
    provider: Option<SdkTracerProvider>,
}

impl TracerShutdown {
    pub fn new(provider: Option<SdkTracerProvider>) -> Self {
        Self { provider }
    }

    pub fn shutdown(&self) -> Result<(), OTelSdkError> {
        if let Some(provider) = &self.provider {
            provider.shutdown().map_err(OTelSdkError::from)
        } else {
            Ok(())
        }
    }
}
