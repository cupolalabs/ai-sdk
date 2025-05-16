use crate::openai::errors::InputError;
use crate::openai::request::input::common::{Content, Role};

use crate::openai::common::{
    computer_tool_call_item::ComputerToolCallItem, file_search_tool_item::FileSearchToolCallItem,
    function_tool_call_item::FunctionToolCallItem, output_message_item::OutputMessageItem,
    reasoning_item::ReasoningItem, status::Status,
    web_search_tool_call_item::WebSearchToolCallItem,
};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct InputMessageItem<'a> {
    pub content: Vec<Content<'a>>,
    pub role: Role,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<&'a str>,
}

impl<'a> InputMessageItem<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn role(mut self, role: &'a str) -> Result<Self, InputError> {
        if role.eq("assistant") {
            Err(InputError::InvalidRole("assistant".to_string()))
        } else {
            self.role = Role::from_str(role).map_err(InputError::ConversionError)?;
            Ok(self)
        }
    }

    pub fn insert_type(mut self) -> Self {
        self.type_field = Some("message");
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputerToolCallOutputItemOutput<'a> {
    #[serde(rename = "type")]
    pub type_field: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<&'a str>,
}

impl Default for ComputerToolCallOutputItemOutput<'_> {
    fn default() -> Self {
        Self {
            type_field: "computer_screenshot",
            image_url: None,
            file_id: None,
        }
    }
}

impl<'a> ComputerToolCallOutputItemOutput<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn file_id(mut self, value: &'a str) -> Self {
        self.file_id = Some(value);
        self
    }

    pub fn image_url(mut self, value: &'a str) -> Self {
        self.image_url = Some(value);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct AcknowledgedSafetyChecks<'a> {
    pub id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<&'a str>,
}

impl<'a> AcknowledgedSafetyChecks<'a> {
    pub fn new(id: &'a str) -> Self {
        Self {
            id,
            code: None,
            message: None,
        }
    }

    pub fn code(mut self, value: &'a str) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: &'a str) -> Self {
        self.message = Some(value);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct ComputerToolCallOutputItem<'a> {
    pub call_id: &'a str,
    pub output: ComputerToolCallOutputItemOutput<'a>,
    #[serde(rename = "type")]
    pub type_field: &'a str,
    pub acknowledged_safety_checks: Option<Vec<AcknowledgedSafetyChecks<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl<'a> ComputerToolCallOutputItem<'a> {
    pub fn new(call_id: &'a str, output: ComputerToolCallOutputItemOutput<'a>) -> Self {
        Self {
            call_id,
            output,
            type_field: "computer_call_output",
            acknowledged_safety_checks: None,
            id: None,
            status: None,
        }
    }

    pub fn acknowledged_safety_checks(mut self, value: Vec<AcknowledgedSafetyChecks<'a>>) -> Self {
        self.acknowledged_safety_checks = Some(value);
        self
    }

    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    pub fn status(mut self, value: Status) -> Self {
        self.status = Some(value);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionToolCallOutputItem<'a> {
    pub call_id: &'a str,
    pub output: &'a str,
    #[serde(rename = "type")]
    pub type_field: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl<'a> FunctionToolCallOutputItem<'a> {
    pub fn new(call_id: &'a str, output: &'a str) -> Self {
        Self {
            call_id,
            output,
            type_field: "function_call_output",
            id: None,
            status: None,
        }
    }

    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    pub fn status(mut self, value: Status) -> Self {
        self.status = Some(value);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Summary<'a> {
    pub text: &'a str,
    #[serde(rename = "type")]
    pub type_field: &'a str,
}

impl<'a> Summary<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            type_field: "summary_text",
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
#[serde(untagged)]
pub enum Item<'a> {
    InputMessage(InputMessageItem<'a>),
    OutputMessage(OutputMessageItem<'a>),
    FileSearchToolCall(FileSearchToolCallItem<'a>),
    ComputerToolCall(ComputerToolCallItem<'a>),
    ComputerToolCallOutput(ComputerToolCallOutputItem<'a>),
    WebSearchToolCall(WebSearchToolCallItem<'a>),
    FunctionToolCall(FunctionToolCallItem<'a>),
    FunctionToolCallOutput(FunctionToolCallOutputItem<'a>),
    Reasoning(ReasoningItem<'a>),
}

impl<'a> From<InputMessageItem<'a>> for Item<'a> {
    fn from(item: InputMessageItem<'a>) -> Self {
        Item::InputMessage(item)
    }
}

impl<'a> From<OutputMessageItem<'a>> for Item<'a> {
    fn from(item: OutputMessageItem<'a>) -> Self {
        Item::OutputMessage(item)
    }
}

impl<'a> From<FileSearchToolCallItem<'a>> for Item<'a> {
    fn from(item: FileSearchToolCallItem<'a>) -> Self {
        Item::FileSearchToolCall(item)
    }
}

impl<'a> From<ComputerToolCallItem<'a>> for Item<'a> {
    fn from(item: ComputerToolCallItem<'a>) -> Self {
        Item::ComputerToolCall(item)
    }
}

impl<'a> From<ComputerToolCallOutputItem<'a>> for Item<'a> {
    fn from(item: ComputerToolCallOutputItem<'a>) -> Self {
        Item::ComputerToolCallOutput(item)
    }
}

impl<'a> From<WebSearchToolCallItem<'a>> for Item<'a> {
    fn from(item: WebSearchToolCallItem<'a>) -> Self {
        Item::WebSearchToolCall(item)
    }
}

impl<'a> From<FunctionToolCallItem<'a>> for Item<'a> {
    fn from(item: FunctionToolCallItem<'a>) -> Self {
        Item::FunctionToolCall(item)
    }
}

impl<'a> From<FunctionToolCallOutputItem<'a>> for Item<'a> {
    fn from(item: FunctionToolCallOutputItem<'a>) -> Self {
        Item::FunctionToolCallOutput(item)
    }
}

impl<'a> From<ReasoningItem<'a>> for Item<'a> {
    fn from(item: ReasoningItem<'a>) -> Self {
        Item::Reasoning(item)
    }
}
