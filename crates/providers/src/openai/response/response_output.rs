use crate::openai::common::{
    computer_tool_call_item::ComputerToolCallItem, file_search_tool_item::FileSearchToolCallItem,
    function_tool_call_item::FunctionToolCallItem, output_message_item::OutputMessageItem,
    reasoning_item::ReasoningItem, web_search_tool_call_item::WebSearchToolCallItem,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ResponseOutput {
    OutputMessage(OutputMessageItem),
    FileSearchToolCall(FileSearchToolCallItem),
    ComputerToolCall(ComputerToolCallItem),
    WebSearchToolCall(WebSearchToolCallItem),
    FunctionToolCall(FunctionToolCallItem),
    Reasoning(ReasoningItem),
}

// TODO: Implement the impl for ResponseOutput
