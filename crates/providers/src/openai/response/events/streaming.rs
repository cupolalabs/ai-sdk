use crate::openai::common::output_message_item::{Annotation, OutputContent, OutputText};
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StreamingEvent<'a> {
    #[serde(rename = "response.created")]
    Created { response: StreamingResponse<'a> },
    #[serde(rename = "response.in_progress")]
    InProgress { response: StreamingResponse<'a> },
    #[serde(rename = "response.completed")]
    Completed { response: StreamingResponse<'a> },
    #[serde(rename = "response.failed")]
    Failed { response: StreamingResponse<'a> },
    #[serde(rename = "response.incomplete")]
    Incomplete { response: StreamingResponse<'a> },
    #[serde(rename = "response.output_item.added")]
    OutputItemAdded {
        output_index: usize,
        item: ResponseOutput<'a>,
    },
    #[serde(rename = "response.output_item.done")]
    OutputItemDone {
        output_index: usize,
        item: ResponseOutput<'a>,
    },
    #[serde(rename = "response.content_part.added")]
    ContentPartAdded {
        item_id: &'a str,
        output_index: usize,
        content_index: usize,
        part: OutputContent<'a>,
    },
    #[serde(rename = "response.content_part.done")]
    ContentPartDone {
        item_id: &'a str,
        output_index: usize,
        content_index: usize,
        part: OutputContent<'a>,
    },
    #[serde(rename = "response.output_text.delta")]
    OutputTextDelta {
        item_id: &'a str,
        output_index: usize,
        content_index: usize,
        delta: &'a str,
    },
    #[serde(rename = "response.output_text.annotation.added")]
    OutputTextAnnotationAdded {
        item_id: &'a str,
        output_index: usize,
        content_index: usize,
        annotation_index: usize,
        annotation: Annotation<'a>,
    },
    #[serde(rename = "response.output_text.done")]
    OutputTextDone {
        item_id: &'a str,
        output_index: usize,
        content_index: usize,
        text: &'a str,
    },
    #[serde(rename = "response.refusal.delta")]
    RefusalDelta {
        item_id: &'a str,
        output_index: usize,
        content_index: usize,
        delta: &'a str,
    },
    #[serde(rename = "response.refusal.done")]
    RefusalDone {
        item_id: &'a str,
        output_index: usize,
        content_index: usize,
        refusal: &'a str,
    },
    #[serde(rename = "response.function_call_arguments.delta")]
    FunctionCallArgumentsDelta {
        item_id: &'a str,
        output_index: usize,
        delta: &'a str,
    },
    #[serde(rename = "response.function_call_arguments.done")]
    FunctionCallArgumentsDone {
        item_id: &'a str,
        output_index: usize,
        arguments: &'a str,
    },
    #[serde(rename = "response.file_search_call.in_progress")]
    FileSearchCallInProgress {
        item_id: &'a str,
        output_index: usize,
    },
    #[serde(rename = "response.file_search_call.searching")]
    FileSearchCallSearching {
        item_id: &'a str,
        output_index: usize,
    },
    #[serde(rename = "response.file_search_call.completed")]
    FileSearchCallCompleted {
        item_id: &'a str,
        output_index: usize,
    },
    #[serde(rename = "response.web_search_call.in_progress")]
    WebSearchCallInProgress {
        item_id: &'a str,
        output_index: usize,
    },
    #[serde(rename = "response.web_search_call.searching")]
    WebSearchCallSearching {
        item_id: &'a str,
        output_index: usize,
    },
    #[serde(rename = "response.web_search_call.completed")]
    WebSearchCallCompleted {
        item_id: &'a str,
        output_index: usize,
    },
    #[serde(rename = "response.reasoning_summary_part.added")]
    ReasoningSummaryPartAdded {
        item_id: &'a str,
        output_index: usize,
        part: ReasoningPart<'a>,
        summary_index: usize,
    },
    #[serde(rename = "response.reasoning_summary_part.done")]
    ReasoningSummaryPartDone {
        item_id: &'a str,
        output_index: usize,
        part: ReasoningPart<'a>,
        summary_index: usize,
    },
    #[serde(rename = "response.reasoning_summary_text.delta")]
    ReasoningSummaryTextDelta {
        delta: &'a str,
        item_id: &'a str,
        output_index: usize,
        summary_index: usize,
    },
    #[serde(rename = "response.reasoning_summary_text.done")]
    ReasoningSummaryTextDone {
        item_id: &'a str,
        output_index: usize,
        summary_index: usize,
        text: &'a str,
    },
    #[serde(rename = "error")]
    Error {
        #[serde(skip_serializing_if = "Option::is_none")]
        code: Option<&'a str>,
        message: &'a str,
        #[serde(skip_serializing_if = "Option::is_none")]
        param: Option<&'a str>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ReasoningPart<'a> {
    #[serde(rename = "type")]
    pub type_field: ReasoningPartType,
    pub text: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningPartType {
    #[serde(rename = "summary_text")]
    SummaryText,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamingResponse<'a> {
    pub created_at: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResponseError<'a>>,
    pub id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<IncompleteDetails<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<&'a str>,
    pub metadata: HashMap<String, String>,
    pub model: &'a str,
    // NOTE: this field is always "response" value
    pub object: &'a str,
    pub output: Vec<ResponseOutput<'a>>,
    pub parallel_tool_calls: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    pub tool_choice: ToolChoice<'a>,
    pub tools: Vec<Tool<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<Reasoning>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,
    pub status: Status,
    pub text: Text<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<Truncation>,
    pub usage: Usage,
    pub user: &'a str,
}
