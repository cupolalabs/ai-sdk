use crate::openai::common::{
    computer_tool_call_item::ComputerToolCallItem, file_search_tool_item::FileSearchToolCallItem,
    function_tool_call_item::FunctionToolCallItem, output_message_item::OutputMessageItem,
    reasoning_item::ReasoningItem, web_search_tool_call_item::WebSearchToolCallItem,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum ResponseOutput {
    #[serde(rename = "message")]
    OutputMessage(OutputMessageItem),
    #[serde(rename = "file_search_call")]
    FileSearchToolCall(FileSearchToolCallItem),
    #[serde(rename = "computer_call")]
    ComputerToolCall(ComputerToolCallItem),
    #[serde(rename = "web_search_call")]
    WebSearchToolCall(WebSearchToolCallItem),
    #[serde(rename = "function_call")]
    FunctionToolCall(FunctionToolCallItem),
    #[serde(rename = "reasoning")]
    Reasoning(ReasoningItem),
}
