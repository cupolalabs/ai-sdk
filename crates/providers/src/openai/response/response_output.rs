use crate::openai::common::{
    computer_tool_call_item::ComputerToolCallItem, file_search_tool_item::FileSearchToolCallItem,
    function_tool_call_item::FunctionToolCallItem, output_message_item::OutputMessageItem,
    reasoning_item::ReasoningItem, web_search_tool_call_item::WebSearchToolCallItem,
};

pub enum ResponseOutput<'a> {
    OutputMessage(OutputMessageItem<'a>),
    FileSearchToolCall(FileSearchToolCallItem<'a>),
    ComputerToolCall(ComputerToolCallItem<'a>),
    WebSearchToolCall(WebSearchToolCallItem<'a>),
    FunctionToolCall(FunctionToolCallItem<'a>),
    Reasoning(ReasoningItem<'a>),
}

// TODO: Implement the impl for ResponseOutput
