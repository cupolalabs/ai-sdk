use crate::openai::common::{
    reasoning::Reasoning, service_tier::ServiceTier, status::Status, text::Text, tool::Tool,
    tool_choice::ToolChoice, truncation::Truncation,
};
use crate::openai::response::{
    incomplete_details::IncompleteDetails, response_error::ResponseError,
    response_output::ResponseOutput, usage::Usage,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Response<'a> {
    created_at: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ResponseError<'a>>,
    id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    incomplete_details: Option<IncompleteDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instructions: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<HashMap<String, String>>,
    model: &'a str,
    // This is always "response"
    object: &'a str,
    output: Vec<ResponseOutput<'a>>,
    parallel_tool_calls: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    previous_response_id: Option<&'a str>,
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
    user: &'a str,
}
