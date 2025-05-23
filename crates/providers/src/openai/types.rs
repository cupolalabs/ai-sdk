use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::openai::common::{
    reasoning::Reasoning, service_tier::ServiceTier, status::Status, text::Text, tool::Tool,
    tool_choice::ToolChoice, truncation::Truncation,
};
use crate::openai::constants::OpenAIModelId;
use crate::openai::request::{include::Include, input::Input};
use crate::openai::response::{
    incomplete_details::IncompleteDetails, response_error::ResponseError,
    response_output::ResponseOutput, usage::Usage,
};

use serde_json::{json, Value};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OpenAIRequest {
    input: Input,
    model: OpenAIModelId,
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<Vec<Include>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    previous_response_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning: Option<Reasoning>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_tier: Option<ServiceTier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    store: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    truncation: Option<Truncation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
}

impl OpenAIRequest {
    pub fn new(model: OpenAIModelId, input: Input) -> Self {
        Self {
            model,
            input,
            ..Default::default()
        }
    }

    pub fn include(mut self, value: Include) -> Self {
        match self.include {
            Some(ref mut include) => include.push(value),
            None => self.include = Some(vec![value]),
        }

        self
    }

    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }

    pub fn max_output_tokens(mut self, value: usize) -> Self {
        self.max_output_tokens = Some(value);
        self
    }

    pub fn insert_metadata(mut self, key: String, value: String) -> Self {
        match self.metadata {
            Some(ref mut metadata) => {
                metadata.insert(key, value);
            }
            None => {
                self.metadata = Some({
                    let mut new_map: HashMap<String, String> = HashMap::new();
                    new_map.insert(key, value);
                    new_map
                });
            }
        }

        self
    }

    pub fn parallel_tool_calls(mut self, value: bool) -> Self {
        self.parallel_tool_calls = Some(value);
        self
    }

    pub fn previous_response_id(mut self, value: impl Into<String>) -> Self {
        self.previous_response_id = Some(value.into());
        self
    }

    pub fn reasoning(mut self, value: Reasoning) -> Self {
        self.reasoning = Some(value);
        self
    }

    pub fn service_tier(mut self, value: ServiceTier) -> Self {
        self.service_tier = Some(value);
        self
    }

    pub fn store(mut self, value: bool) -> Self {
        self.store = Some(value);
        self
    }

    pub fn temperature(mut self, value: f32) -> Self {
        self.temperature = Some(value);
        self
    }

    pub fn text(mut self, value: Text) -> Self {
        self.text = Some(value);
        self
    }

    pub fn tool_choice(mut self, value: ToolChoice) -> Self {
        self.tool_choice = Some(value);
        self
    }

    pub fn add_tool(mut self, value: Tool) -> Self {
        match self.tools {
            Some(ref mut tools) => tools.push(value),
            None => self.tools = Some(vec![value]),
        }
        self
    }

    pub fn top_p(mut self, value: f32) -> Self {
        self.top_p = Some(value);
        self
    }

    pub fn truncation(mut self, value: Truncation) -> Self {
        self.truncation = Some(value);
        self
    }

    pub fn user(mut self, value: impl Into<String>) -> Self {
        self.user = Some(value.into());
        self
    }

    pub fn wrap_for_streaming<'a>(&'a self) -> impl Serialize + 'a {
        struct Wrapper<'a> {
            inner: &'a OpenAIRequest,
        }

        impl<'a> Serialize for Wrapper<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut original = serde_json::to_value(self.inner)
                    .map_err(|e| serde::ser::Error::custom(e.to_string()))?;

                if let Value::Object(ref mut map) = original {
                    map.insert("stream".to_string(), json!(true));

                    map.serialize(serializer)
                } else {
                    Err(serde::ser::Error::custom("Expected object"))
                }
            }
        }

        Wrapper { inner: self }
    }
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(bound(deserialize = ""))]
#[derive(Deserialize)]
pub struct OpenAIResponse {
    created_at: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ResponseError>,
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    incomplete_details: Option<IncompleteDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<HashMap<String, String>>,
    model: String,
    object: String,
    output: Vec<ResponseOutput>,
    parallel_tool_calls: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    previous_response_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning: Option<Reasoning>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_tier: Option<ServiceTier>,
    status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    text: Text,
    tool_choice: ToolChoice,
    tools: Vec<Tool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    truncation: Option<Truncation>,
    usage: Usage,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
}
