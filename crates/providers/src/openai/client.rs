use crate::openai::constants::OPENAI_API_URL;
use crate::openai::response::events::streaming::OpenAIStreamingEvent;
use async_trait::async_trait;
use futures::stream::StreamExt;
use std::pin::Pin;
use tokio_stream::Stream;
use utils::provider::{Provider, ProviderError};

use super::types::{OpenAIRequest, OpenAIResponse};

pub struct OpenAIProvider {
    api_key: String,
}

impl OpenAIProvider {
    pub fn new(api_key: String) -> Self {
        OpenAIProvider { api_key }
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    type GenerationRequest = OpenAIRequest;
    type StreamingRequest = OpenAIRequest;
    type GenerationResponse = OpenAIResponse;
    type StreamingResponse = OpenAIStreamingEvent;

    fn get_base_url(&self) -> String {
        OPENAI_API_URL.to_string()
    }

    fn get_api_key(&self) -> String {
        self.api_key.clone()
    }

    async fn generate(
        &self,
        request: &Self::GenerationRequest,
    ) -> Result<Self::GenerationResponse, ProviderError> {
        let client = reqwest::Client::new();
        let url = format!("{}/responses", self.get_base_url());

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.get_api_key()))
            .header("Content-Type", "application/json")
            .json(request)
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_message = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error response".to_string());

            return Err(ProviderError::ApiError {
                status,
                message: error_message,
            });
        }

        let response_bytes = response
            .bytes()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        serde_json::from_slice(&response_bytes)
            .map_err(|e| ProviderError::DeserializationError(e.to_string()))
    }

    async fn stream(
        &self,
        request: &Self::StreamingRequest,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<Self::StreamingResponse, ProviderError>> + Send>>,
        ProviderError,
    > {
        let client = reqwest::Client::new();
        let url = format!("{}/responses", self.get_base_url());

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.get_api_key()))
            .header("Content-Type", "application/json")
            .header("Accept", "text/event-stream")
            .json(&request.wrap_for_streaming())
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_message = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error response".to_string());

            return Err(ProviderError::ApiError {
                status,
                message: error_message,
            });
        }

        let stream = response.bytes_stream();
        let parsed_stream = stream.map(|chunk_result| {
            chunk_result
                .map_err(|e| ProviderError::NetworkError(e.to_string()))
                .and_then(|chunk| {
                    serde_json::from_slice(&chunk)
                        .map_err(|e| ProviderError::DeserializationError(e.to_string()))
                })
        });

        Ok(Box::pin(parsed_stream))
    }
}
