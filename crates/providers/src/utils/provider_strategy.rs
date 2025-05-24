use crate::utils::errors::ProviderError;
use async_trait::async_trait;
use serde::{de::Deserialize, Serialize};
use std::pin::Pin;
use tokio_stream::Stream;

/// Provider trait defines a strategy pattern interface for API providers.
///
/// This trait acts as the Strategy interface in the Strategy pattern:
/// - Concrete providers implement this trait to provide specific API endpoints and authentication
/// - Client code can use any provider that implements this trait interchangeably
/// - Allows for runtime switching between different API providers without changing client code
///
/// Each provider must implement methods to get the base URL and API key for their specific service.
#[async_trait]
pub trait ProviderStrategy {
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
