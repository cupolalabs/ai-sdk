pub mod common {
    pub mod computer_tool_call_item;
    pub mod file_search_tool_item;
    pub mod function_tool_call_item;
    pub mod output_message_item;
    pub mod reasoning;
    pub mod reasoning_item;
    pub mod service_tier;
    pub mod status;
    pub mod text;
    pub mod tool;
    pub mod tool_choice;
    pub mod truncation;
    pub mod web_search_tool_call_item;
}

pub mod constants;
pub mod errors;

pub mod response {
    pub mod incomplete_details;
    pub mod response_error;
    pub mod response_output;
    pub mod usage;
    pub mod events {
        pub mod streaming;
    }
}

pub mod request {
    pub mod include;
    pub mod input;
}

const OPENAI_API_URL: &str = "https://api.openai.com/v1";

use crate::openai::constants::OpenAIModelId;
use async_trait::async_trait;
use futures::stream::StreamExt;
use serde::{de::DeserializeOwned, Serialize};
use std::pin::Pin;
use tokio_stream::Stream;
use utils::provider::{Provider, ProviderError};

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
    fn get_base_url(&self) -> String {
        OPENAI_API_URL.to_string()
    }

    fn get_api_key(&self) -> String {
        self.api_key.clone()
    }

    async fn generate<P: Serialize + Send + Sync, R: DeserializeOwned + Send>(
        &self,
        request: &P,
    ) -> Result<R, ProviderError> {
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

        response
            .json::<R>()
            .await
            .map_err(|e| ProviderError::DeserializationError(e.to_string()))
    }

    async fn stream<P: Serialize + Send + Sync, R: DeserializeOwned + Send>(
        &self,
        request: &P,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<R, ProviderError>> + Send>>, ProviderError> {
        let client = reqwest::Client::new();
        let url = format!("{}/responses", self.get_base_url());

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.get_api_key()))
            .header("Content-Type", "application/json")
            .header("Accept", "text/event-stream")
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

        let stream = response.bytes_stream();

        let parsed_stream = stream.map(|chunk_result| match chunk_result {
            Ok(chunk) => match serde_json::from_slice::<R>(&chunk) {
                Ok(parsed) => Ok(parsed),
                Err(e) => Err(ProviderError::DeserializationError(e.to_string())),
            },
            Err(e) => Err(ProviderError::NetworkError(e.to_string())),
        });

        Ok(Box::pin(parsed_stream))
    }
}
