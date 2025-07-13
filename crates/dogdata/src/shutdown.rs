use opentelemetry_sdk::{error::OTelSdkError, logs::SdkLoggerProvider, trace::SdkTracerProvider};

pub struct Shutdown {
    providers: Option<(SdkTracerProvider, SdkLoggerProvider)>,
}

impl Shutdown {
    pub fn new(providers: Option<(SdkTracerProvider, SdkLoggerProvider)>) -> Self {
        Self { providers }
    }

    pub fn shutdown(&self) -> Result<(), OTelSdkError> {
        if let Some((tracer_provider, logger_provider)) = &self.providers {
            tracer_provider.shutdown()?;
            logger_provider.shutdown()?;
            Ok(())
        } else {
            Ok(())
        }
    }
}
