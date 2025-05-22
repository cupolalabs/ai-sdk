use async_trait::async_trait;
use serde::{de::Deserialize, Serialize};
use std::fmt;
use std::pin::Pin;
use tokio_stream::Stream;

#[derive(Debug)]
pub enum ProviderError {
    NetworkError(String),
    ApiError { status: u16, message: String },
    DeserializationError(String),
    ValidationError(String),
    CapabilityError(String),
    NotSupported(String),
    InternalError(String),
    Other(String),
}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProviderError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ProviderError::ApiError { status, message } => {
                write!(f, "API error (status {}): {}", status, message)
            }
            ProviderError::DeserializationError(msg) => {
                write!(f, "Deserialization error: {}", msg)
            }
            ProviderError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ProviderError::CapabilityError(msg) => write!(f, "Capability error: {}", msg),
            ProviderError::NotSupported(msg) => write!(f, "Operation not supported: {}", msg),
            ProviderError::InternalError(msg) => write!(f, "Internal provider error: {}", msg),
            ProviderError::Other(msg) => write!(f, "An unexpected error occurred: {}", msg),
        }
    }
}

impl std::error::Error for ProviderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ProviderError::NetworkError(_) => None,
            ProviderError::ApiError { .. } => None,
            ProviderError::DeserializationError(_) => None,
            ProviderError::ValidationError(_) => None,
            ProviderError::CapabilityError(_) => None,
            ProviderError::NotSupported(_) => None,
            ProviderError::InternalError(_) => None,
            ProviderError::Other(_) => None,
        }
    }
}

/// Provider trait defines a strategy pattern interface for API providers.
///
/// This trait acts as the Strategy interface in the Strategy pattern:
/// - Concrete providers implement this trait to provide specific API endpoints and authentication
/// - Client code can use any provider that implements this trait interchangeably
/// - Allows for runtime switching between different API providers without changing client code
///
/// Each provider must implement methods to get the base URL and API key for their specific service.
#[async_trait]
pub trait Provider {
    type GenerationRequest: Serialize + Send + Sync;
    type StreamingRequest: Serialize + Send + Sync;
    type GenerationResponse: for<'de> Deserialize<'de> + Send;
    type StreamingResponse: for<'de> Deserialize<'de> + Send;

    fn get_base_url(&self) -> String;
    fn get_api_key(&self) -> String;
    async fn generate(
        &self,
        request: &Self::GenerationRequest,
    ) -> Result<Self::GenerationResponse, ProviderError>;
    async fn stream(
        &self,
        request: &Self::StreamingRequest,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<Self::StreamingResponse, ProviderError>> + Send>>,
        ProviderError,
    >;
}
