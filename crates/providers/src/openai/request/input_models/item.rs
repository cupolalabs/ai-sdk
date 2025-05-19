use crate::openai::errors::InputError;
use crate::openai::request::input_models::common::{Content, Role};

use crate::openai::common::{
    computer_tool_call_item::ComputerToolCallItem, file_search_tool_item::FileSearchToolCallItem,
    function_tool_call_item::FunctionToolCallItem, output_message_item::OutputMessageItem,
    reasoning_item::ReasoningItem, status::Status,
    web_search_tool_call_item::WebSearchToolCallItem,
};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InputMessageItem {
    pub content: Vec<Content>,
    pub role: Role,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<String>,
}

impl InputMessageItem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn role(mut self, role: impl AsRef<str>) -> Result<Self, InputError> {
        if role.as_ref().eq("assistant") {
            Err(InputError::InvalidRole("assistant".to_string()))
        } else {
            self.role = Role::from_str(role.as_ref()).map_err(InputError::ConversionError)?;
            Ok(self)
        }
    }

    pub fn insert_type(mut self) -> Self {
        self.type_field = Some("message".to_string());
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputerToolCallOutputItemOutput {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

impl Default for ComputerToolCallOutputItemOutput {
    fn default() -> Self {
        Self {
            type_field: "computer_screenshot".to_string(),
            image_url: None,
            file_id: None,
        }
    }
}

impl ComputerToolCallOutputItemOutput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcknowledgedSafetyChecks {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl AcknowledgedSafetyChecks {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            code: None,
            message: None,
        }
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputerToolCallOutputItem {
    pub call_id: String,
    pub output: ComputerToolCallOutputItemOutput,
    #[serde(rename = "type")]
    pub type_field: String,
    pub acknowledged_safety_checks: Option<Vec<AcknowledgedSafetyChecks>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl ComputerToolCallOutputItem {
    pub fn new(call_id: impl Into<String>, output: ComputerToolCallOutputItemOutput) -> Self {
        Self {
            call_id: call_id.into(),
            output,
            type_field: "computer_call_output".to_string(),
            acknowledged_safety_checks: None,
            id: None,
            status: None,
        }
    }

    pub fn acknowledged_safety_checks(mut self, value: Vec<AcknowledgedSafetyChecks>) -> Self {
        self.acknowledged_safety_checks = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: Status) -> Self {
        self.status = Some(value);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionToolCallOutputItem {
    pub call_id: String,
    pub output: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl FunctionToolCallOutputItem {
    pub fn new(call_id: impl Into<String>, output: impl Into<String>) -> Self {
        Self {
            call_id: call_id.into(),
            output: output.into(),
            type_field: "function_call_output".to_string(),
            id: None,
            status: None,
        }
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: Status) -> Self {
        self.status = Some(value);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Summary {
    pub text: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

impl Summary {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            type_field: "summary_text".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Item {
    InputMessage(InputMessageItem),
    OutputMessage(OutputMessageItem),
    FileSearchToolCall(FileSearchToolCallItem),
    ComputerToolCall(ComputerToolCallItem),
    ComputerToolCallOutput(ComputerToolCallOutputItem),
    WebSearchToolCall(WebSearchToolCallItem),
    FunctionToolCall(FunctionToolCallItem),
    FunctionToolCallOutput(FunctionToolCallOutputItem),
    Reasoning(ReasoningItem),
}

impl From<InputMessageItem> for Item {
    fn from(item: InputMessageItem) -> Self {
        Item::InputMessage(item)
    }
}

impl From<OutputMessageItem> for Item {
    fn from(item: OutputMessageItem) -> Self {
        Item::OutputMessage(item)
    }
}

impl From<FileSearchToolCallItem> for Item {
    fn from(item: FileSearchToolCallItem) -> Self {
        Item::FileSearchToolCall(item)
    }
}

impl From<ComputerToolCallItem> for Item {
    fn from(item: ComputerToolCallItem) -> Self {
        Item::ComputerToolCall(item)
    }
}

impl From<ComputerToolCallOutputItem> for Item {
    fn from(item: ComputerToolCallOutputItem) -> Self {
        Item::ComputerToolCallOutput(item)
    }
}

impl From<WebSearchToolCallItem> for Item {
    fn from(item: WebSearchToolCallItem) -> Self {
        Item::WebSearchToolCall(item)
    }
}

impl From<FunctionToolCallItem> for Item {
    fn from(item: FunctionToolCallItem) -> Self {
        Item::FunctionToolCall(item)
    }
}

impl From<FunctionToolCallOutputItem> for Item {
    fn from(item: FunctionToolCallOutputItem) -> Self {
        Item::FunctionToolCallOutput(item)
    }
}

impl From<ReasoningItem> for Item {
    fn from(item: ReasoningItem) -> Self {
        Item::Reasoning(item)
    }
}
