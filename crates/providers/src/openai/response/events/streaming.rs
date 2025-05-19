use crate::openai::common::output_message_item::{Annotation, OutputContent};
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

#[derive(Debug, Serialize)]
#[serde(bound(deserialize = ""))]
#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum OpenAIStreamingEvent {
    #[serde(rename = "response.created")]
    Created { response: StreamingResponse },
    #[serde(rename = "response.in_progress")]
    InProgress { response: StreamingResponse },
    #[serde(rename = "response.completed")]
    Completed { response: StreamingResponse },
    #[serde(rename = "response.failed")]
    Failed { response: StreamingResponse },
    #[serde(rename = "response.incomplete")]
    Incomplete { response: StreamingResponse },
    #[serde(rename = "response.output_item.added")]
    OutputItemAdded {
        output_index: usize,
        item: ResponseOutput,
    },
    #[serde(rename = "response.output_item.done")]
    OutputItemDone {
        output_index: usize,
        item: ResponseOutput,
    },
    #[serde(rename = "response.content_part.added")]
    ContentPartAdded {
        item_id: String,
        output_index: usize,
        content_index: usize,
        part: OutputContent,
    },
    #[serde(rename = "response.content_part.done")]
    ContentPartDone {
        item_id: String,
        output_index: usize,
        content_index: usize,
        part: OutputContent,
    },
    #[serde(rename = "response.output_text.delta")]
    OutputTextDelta {
        item_id: String,
        output_index: usize,
        content_index: usize,
        delta: String,
    },
    #[serde(rename = "response.output_text.annotation.added")]
    OutputTextAnnotationAdded {
        item_id: String,
        output_index: usize,
        content_index: usize,
        annotation_index: usize,
        annotation: Annotation,
    },
    #[serde(rename = "response.output_text.done")]
    OutputTextDone {
        item_id: String,
        output_index: usize,
        content_index: usize,
        text: String,
    },
    #[serde(rename = "response.refusal.delta")]
    RefusalDelta {
        item_id: String,
        output_index: usize,
        content_index: usize,
        delta: String,
    },
    #[serde(rename = "response.refusal.done")]
    RefusalDone {
        item_id: String,
        output_index: usize,
        content_index: usize,
        refusal: String,
    },
    #[serde(rename = "response.function_call_arguments.delta")]
    FunctionCallArgumentsDelta {
        item_id: String,
        output_index: usize,
        delta: String,
    },
    #[serde(rename = "response.function_call_arguments.done")]
    FunctionCallArgumentsDone {
        item_id: String,
        output_index: usize,
        arguments: String,
    },
    #[serde(rename = "response.file_search_call.in_progress")]
    FileSearchCallInProgress {
        item_id: String,
        output_index: usize,
    },
    #[serde(rename = "response.file_search_call.searching")]
    FileSearchCallSearching {
        item_id: String,
        output_index: usize,
    },
    #[serde(rename = "response.file_search_call.completed")]
    FileSearchCallCompleted {
        item_id: String,
        output_index: usize,
    },
    #[serde(rename = "response.web_search_call.in_progress")]
    WebSearchCallInProgress {
        item_id: String,
        output_index: usize,
    },
    #[serde(rename = "response.web_search_call.searching")]
    WebSearchCallSearching {
        item_id: String,
        output_index: usize,
    },
    #[serde(rename = "response.web_search_call.completed")]
    WebSearchCallCompleted {
        item_id: String,
        output_index: usize,
    },
    #[serde(rename = "response.reasoning_summary_part.added")]
    ReasoningSummaryPartAdded {
        item_id: String,
        output_index: usize,
        part: ReasoningPart,
        summary_index: usize,
    },
    #[serde(rename = "response.reasoning_summary_part.done")]
    ReasoningSummaryPartDone {
        item_id: String,
        output_index: usize,
        part: ReasoningPart,
        summary_index: usize,
    },
    #[serde(rename = "response.reasoning_summary_text.delta")]
    ReasoningSummaryTextDelta {
        delta: String,
        item_id: String,
        output_index: usize,
        summary_index: usize,
    },
    #[serde(rename = "response.reasoning_summary_text.done")]
    ReasoningSummaryTextDone {
        item_id: String,
        output_index: usize,
        summary_index: usize,
        text: String,
    },
    #[serde(rename = "error")]
    Error {
        #[serde(skip_serializing_if = "Option::is_none")]
        code: Option<String>,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        param: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ReasoningPart {
    #[serde(rename = "type")]
    pub type_field: ReasoningPartType,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningPartType {
    #[serde(rename = "summary_text")]
    SummaryText,
}

#[derive(Debug, Serialize)]
#[serde(bound(deserialize = ""))]
#[derive(Deserialize)]
pub struct StreamingResponse {
    pub created_at: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResponseError>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<IncompleteDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    pub metadata: HashMap<String, String>,
    pub model: String,
    pub object: String,
    pub output: Vec<ResponseOutput>,
    pub parallel_tool_calls: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    pub tool_choice: ToolChoice,
    pub tools: Vec<Tool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<Reasoning>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,
    pub status: Status,
    pub text: Text,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<Truncation>,
    pub usage: Usage,
    pub user: String,
}
