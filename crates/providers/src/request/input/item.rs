use crate::errors::InputError;
use crate::request::input::common::{Content, Role, Status};
use crate::ConversionError;
use std::{collections::HashMap, str::FromStr};

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
pub struct FileCitation<'a> {
    pub file_id: &'a str,
    pub index: usize,
    #[serde(rename = "type")]
    pub type_field: &'a str,
}

impl<'a> FileCitation<'a> {
    pub fn new(file_id: &'a str, index: usize) -> Self {
        Self {
            file_id,
            index,
            type_field: "file_citation",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlCitation<'a> {
    pub end_index: &'a str,
    pub start_index: &'a str,
    pub title: &'a str,
    #[serde(rename = "type")]
    pub type_field: &'a str,
    pub url: &'a str,
}

impl<'a> UrlCitation<'a> {
    pub fn new(end_index: &'a str, start_index: &'a str, title: &'a str, url: &'a str) -> Self {
        Self {
            end_index,
            start_index,
            title,
            url,
            type_field: "url_citation",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilePath<'a> {
    pub file_id: &'a str,
    pub index: usize,
    #[serde(rename = "type")]
    pub type_field: &'a str,
}

impl<'a> FilePath<'a> {
    pub fn new(file_id: &'a str, index: usize) -> Self {
        Self {
            file_id,
            index,
            type_field: "file_path",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
#[serde(untagged)]
pub enum Annotation<'a> {
    FileCitation(FileCitation<'a>),
    UrlCitation(UrlCitation<'a>),
    FilePath(FilePath<'a>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputText<'a> {
    pub annotations: Vec<Annotation<'a>>,
    pub text: &'a str,
    #[serde(rename = "type")]
    pub type_field: &'a str,
}

impl<'a> OutputText<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            annotations: vec![],
            text,
            type_field: "output_text",
        }
    }

    pub fn extend_annotations(mut self, annotation: Vec<Annotation<'a>>) -> Self {
        self.annotations.extend(annotation);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Refusal<'a> {
    pub refusal: &'a str,
    #[serde(rename = "type")]
    pub type_field: &'a str,
}

impl<'a> Refusal<'a> {
    pub fn new(refusal: &'a str) -> Self {
        Self {
            refusal,
            type_field: "refusal",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
#[serde(untagged)]
pub enum OutputContent<'a> {
    OutputText(OutputText<'a>),
    Refusal(Refusal<'a>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct OutputMessageItem<'a> {
    pub content: Vec<OutputContent<'a>>,
    pub id: &'a str,
    pub role: Role,
    pub status: Status,
    #[serde(rename = "type")]
    pub type_field: &'a str,
}

impl<'a> OutputMessageItem<'a> {
    pub fn new(id: &'a str, status: &'a str) -> Result<Self, ConversionError> {
        Ok(Self {
            content: vec![],
            id,
            role: Role::Assistant,
            status: Status::from_str(status)?,
            type_field: "message",
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FileSearchToolCallResult<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<&'a str>,
}

impl<'a> FileSearchToolCallResult<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_attribute(mut self, key: String, value: String) -> Self {
        if self.attributes.is_none() {
            self.attributes = Some(HashMap::new());
        }

        if let Some(attrs) = &mut self.attributes {
            attrs.insert(key, value);
        }

        self
    }

    pub fn file_id(mut self, value: &'a str) -> Self {
        self.file_id = Some(value);
        self
    }

    pub fn filename(mut self, value: &'a str) -> Self {
        self.filename = Some(value);
        self
    }

    pub fn score(mut self, value: usize) -> Self {
        self.score = Some(value);
        self
    }

    pub fn text(mut self, value: &'a str) -> Self {
        self.text = Some(value);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct FileSearchToolCallItem<'a> {
    pub id: &'a str,
    pub queries: Vec<&'a str>,
    pub status: Status,
    #[serde(rename = "type")]
    pub type_field: &'a str,
    pub results: Vec<FileSearchToolCallResult<'a>>,
}

impl<'a> FileSearchToolCallItem<'a> {
    pub fn new(id: &'a str, status: &'a str) -> Result<Self, ConversionError> {
        Ok(Self {
            id,
            queries: vec![],
            status: Status::from_str(status)?,
            type_field: "file_search_call",
            results: vec![],
        })
    }

    pub fn extend_queries(mut self, queries: Vec<&'a str>) -> Self {
        self.queries.extend(queries);
        self
    }

    pub fn extend_results(mut self, results: Vec<FileSearchToolCallResult<'a>>) -> Self {
        self.results.extend(results);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClickAction<'a> {
    pub button: &'a str,
    #[serde(rename = "type")]
    pub type_field: &'a str,
    pub x: usize,
    pub y: usize,
}

impl<'a> ClickAction<'a> {
    const BUTTON: [&'a str; 5] = ["left", "right", "wheel", "back", "forward"];

    pub fn new(button: &'a str, x: usize, y: usize) -> Result<Self, InputError> {
        if Self::BUTTON.contains(&button) {
            Ok(Self {
                button,
                type_field: "click",
                x,
                y,
            })
        } else {
            Err(InputError::InvalidButton(button.to_string()))
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DoubleClickAction<'a> {
    #[serde(rename = "type")]
    pub type_field: &'a str,
    pub x: usize,
    pub y: usize,
}

impl DoubleClickAction<'_> {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            type_field: "double_click",
            x,
            y,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DragActionPath {
    pub x: usize,
    pub y: usize,
}

impl DragActionPath {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DragAction<'a> {
    #[serde(rename = "type")]
    pub type_field: &'a str,
    pub path: Vec<DragActionPath>,
}

impl DragAction<'_> {
    pub fn new(path: Vec<DragActionPath>) -> Self {
        Self {
            type_field: "drag",
            path,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyPressAction<'a> {
    #[serde(rename = "type")]
    pub type_field: &'a str,
    pub keys: Vec<&'a str>,
}

impl<'a> KeyPressAction<'a> {
    pub fn new(keys: Vec<&'a str>) -> Self {
        Self {
            type_field: "keypress",
            keys,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveAction<'a> {
    #[serde(rename = "type")]
    pub type_field: &'a str,
    pub x: usize,
    pub y: usize,
}

impl MoveAction<'_> {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            type_field: "move",
            x,
            y,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScreenshotAction<'a> {
    #[serde(rename = "type")]
    pub type_field: &'a str,
}

impl ScreenshotAction<'_> {
    pub fn new() -> Self {
        Self {
            type_field: "screenshot",
        }
    }
}

impl Default for ScreenshotAction<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScrollAction<'a> {
    #[serde(rename = "type")]
    pub type_field: &'a str,
    pub scroll_x: usize,
    pub scroll_y: usize,
    pub x: usize,
    pub y: usize,
}

impl ScrollAction<'_> {
    pub fn new(scroll_x: usize, scroll_y: usize, x: usize, y: usize) -> Self {
        Self {
            type_field: "scroll",
            scroll_x,
            scroll_y,
            x,
            y,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TypeAction<'a> {
    #[serde(rename = "type")]
    pub type_field: &'a str,
    pub text: &'a str,
}

impl<'a> TypeAction<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            type_field: "type",
            text,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WaitAction<'a> {
    #[serde(rename = "type")]
    pub type_field: &'a str,
}

impl WaitAction<'_> {
    pub fn new() -> Self {
        Self { type_field: "wait" }
    }
}

impl Default for WaitAction<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
#[serde(untagged)]
pub enum ComputerToolAction<'a> {
    Click(ClickAction<'a>),
    DoubleClick(DoubleClickAction<'a>),
    Drag(DragAction<'a>),
    KeyPress(KeyPressAction<'a>),
    Move(MoveAction<'a>),
    Screenshot(ScreenshotAction<'a>),
    Scroll(ScrollAction<'a>),
    Type(TypeAction<'a>),
    Wait(WaitAction<'a>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct PendingSafetyChecks<'a> {
    pub code: &'a str,
    pub id: &'a str,
    pub message: &'a str,
}

impl<'a> PendingSafetyChecks<'a> {
    pub fn new(code: &'a str, id: &'a str, message: &'a str) -> Self {
        Self { code, id, message }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct ComputerToolCallItem<'a> {
    pub action: ComputerToolAction<'a>,
    pub call_id: &'a str,
    pub id: &'a str,
    pub pending_safety_checks: Vec<PendingSafetyChecks<'a>>,
    pub status: Status,
    #[serde(rename = "type")]
    pub type_field: &'a str,
}

impl<'a> ComputerToolCallItem<'a> {
    pub fn new(
        action: ComputerToolAction<'a>,
        call_id: &'a str,
        id: &'a str,
        pending_safety_checks: Vec<PendingSafetyChecks<'a>>,
        status: Status,
    ) -> Self {
        Self {
            action,
            call_id,
            id,
            pending_safety_checks,
            status,
            type_field: "computer_call",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Output<'a> {
    #[serde(rename = "type")]
    pub type_field: &'a str,
    pub file_id: Option<&'a str>,
    pub image_url: Option<&'a str>,
}

impl Default for Output<'_> {
    fn default() -> Self {
        Self {
            type_field: "computer_screenshot",
            image_url: None,
            file_id: None,
        }
    }
}

impl<'a> Output<'a> {
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
    pub code: Option<&'a str>,
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
    pub output: Output<'a>,
    #[serde(rename = "type")]
    pub type_field: &'a str,
    pub acknowledged_safety_checks: Option<Vec<AcknowledgedSafetyChecks<'a>>>,
    pub id: Option<&'a str>,
    pub status: Option<Status>,
}

impl<'a> ComputerToolCallOutputItem<'a> {
    pub fn new(call_id: &'a str, output: Output<'a>) -> Self {
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
pub struct WebSearchToolCallItem<'a> {
    pub id: &'a str,
    pub status: &'a str,
    #[serde(rename = "type")]
    pub type_field: &'a str,
}

impl<'a> WebSearchToolCallItem<'a> {
    pub fn new(id: &'a str, status: &'a str) -> Self {
        Self {
            id,
            status,
            type_field: "web_search_call",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionToolCallItem<'a> {
    pub arguments: &'a str,
    pub call_id: &'a str,
    pub name: &'a str,
    #[serde(rename = "type")]
    pub type_field: &'a str,
    pub id: Option<&'a str>,
    pub status: Option<Status>,
}

impl<'a> FunctionToolCallItem<'a> {
    pub fn new(arguments: &'a str, call_id: &'a str, name: &'a str) -> Self {
        Self {
            arguments,
            call_id,
            name,
            type_field: "function_call",
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
pub struct FunctionToolCallOutputItem<'a> {
    pub call_id: &'a str,
    pub output: &'a str,
    #[serde(rename = "type")]
    pub type_field: &'a str,
    pub id: Option<&'a str>,
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReasoningItem<'a> {
    pub id: &'a str,
    pub summary: Vec<Summary<'a>>,
    #[serde(rename = "type")]
    pub type_field: &'a str,
    pub encrypted_content: Option<&'a str>,
    pub status: Option<Status>,
}

impl<'a> ReasoningItem<'a> {
    pub fn new(id: &'a str, summary: Vec<Summary<'a>>) -> Self {
        Self {
            id,
            summary,
            type_field: "reasoning",
            encrypted_content: None,
            status: None,
        }
    }

    pub fn encrypted_content(mut self, value: &'a str) -> Self {
        self.encrypted_content = Some(value);
        self
    }

    pub fn status(mut self, value: Status) -> Self {
        self.status = Some(value);
        self
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
