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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, to_value};

    #[test]
    fn test_input_message_item_json() {
        let item = Item::from(InputMessageItem {
            content: vec![],
            role: Role::User,
            status: Some(Status::InProgress),
            type_field: Some("message"),
        });
        let value = to_value(&item).unwrap();
        assert_eq!(
            value,
            json!({
                "content": [],
                "role": "user",
                "status": "in_progress",
                "type": "message"
            })
        );
    }

    #[test]
    fn test_output_message_item_json() {
        let item = Item::from(OutputMessageItem {
            content: vec![],
            id: "id123",
            role: Role::Assistant,
            status: Status::Completed,
            type_field: "message",
        });
        let value = to_value(&item).unwrap();
        assert_eq!(
            value,
            json!({
                "content": [],
                "id": "id123",
                "role": "assistant",
                "status": "completed",
                "type": "message"
            })
        );
    }

    #[test]
    fn test_file_search_tool_call_item_json() {
        let item = Item::from(FileSearchToolCallItem {
            id: "search1",
            queries: vec!["foo", "bar"],
            status: Status::InProgress,
            type_field: "file_search_call",
            results: vec![],
        });
        let value = to_value(&item).unwrap();
        assert_eq!(
            value,
            json!({
                "id": "search1",
                "queries": ["foo", "bar"],
                "status": "in_progress",
                "type": "file_search_call",
                "results": []
            })
        );
    }

    #[test]
    fn test_function_tool_call_output_item_json() {
        let item = Item::from(FunctionToolCallOutputItem {
            call_id: "call42",
            output: "{\"result\":42}",
            type_field: "function_call_output",
            id: Some("id42"),
            status: Some(Status::Completed),
        });
        let value = to_value(&item).unwrap();
        assert_eq!(
            value,
            json!({
                "call_id": "call42",
                "output": "{\"result\":42}",
                "type": "function_call_output",
                "id": "id42",
                "status": "completed"
            })
        );
    }

    #[test]
    fn test_file_citation_json() {
        let fc = FileCitation {
            file_id: "file1",
            index: 1,
            type_field: "file_citation",
        };
        let value = to_value(&fc).unwrap();
        assert_eq!(
            value,
            json!({"file_id": "file1", "index": 1, "type": "file_citation"})
        );
    }

    #[test]
    fn test_url_citation_json() {
        let uc = UrlCitation {
            end_index: "10",
            start_index: "5",
            title: "Example",
            type_field: "url_citation",
            url: "https://example.com",
        };
        let value = to_value(&uc).unwrap();
        assert_eq!(
            value,
            json!({"end_index": "10", "start_index": "5", "title": "Example", "type": "url_citation", "url": "https://example.com"})
        );
    }

    #[test]
    fn test_file_path_json() {
        let fp = FilePath {
            file_id: "file2",
            index: 2,
            type_field: "file_path",
        };
        let value = to_value(&fp).unwrap();
        assert_eq!(
            value,
            json!({"file_id": "file2", "index": 2, "type": "file_path"})
        );
    }

    #[test]
    fn test_annotation_json() {
        let ann = Annotation::FileCitation(FileCitation {
            file_id: "file1",
            index: 1,
            type_field: "file_citation",
        });
        let value = to_value(&ann).unwrap();
        assert_eq!(
            value,
            json!({"file_id": "file1", "index": 1, "type": "file_citation"})
        );
    }

    #[test]
    fn test_output_text_json() {
        let ot = OutputText {
            annotations: vec![],
            text: "output",
            type_field: "output_text",
        };
        let value = to_value(&ot).unwrap();
        assert_eq!(
            value,
            json!({"annotations": [], "text": "output", "type": "output_text"})
        );
    }

    #[test]
    fn test_refusal_json() {
        let r = Refusal {
            refusal: "no",
            type_field: "refusal",
        };
        let value = to_value(&r).unwrap();
        assert_eq!(value, json!({"refusal": "no", "type": "refusal"}));
    }

    #[test]
    fn test_output_content_json() {
        let oc = OutputContent::Refusal(Refusal {
            refusal: "no",
            type_field: "refusal",
        });
        let value = to_value(&oc).unwrap();
        assert_eq!(value, json!({"refusal": "no", "type": "refusal"}));
    }

    #[test]
    fn test_file_search_tool_call_result_json() {
        let mut map = std::collections::HashMap::new();
        map.insert("k".to_string(), "v".to_string());
        let res = FileSearchToolCallResult {
            attributes: Some(map),
            file_id: Some("f"),
            filename: Some("n"),
            score: Some(1),
            text: Some("t"),
        };
        let value = to_value(&res).unwrap();
        assert_eq!(
            value,
            json!({"attributes": {"k": "v"}, "file_id": "f", "filename": "n", "score": 1, "text": "t"})
        );
    }

    #[test]
    fn test_click_action_json() {
        let ca = ClickAction {
            button: "left",
            type_field: "click",
            x: 1,
            y: 2,
        };
        let value = to_value(&ca).unwrap();
        assert_eq!(
            value,
            json!({"button": "left", "type": "click", "x": 1, "y": 2})
        );
    }

    #[test]
    fn test_double_click_action_json() {
        let dca = DoubleClickAction {
            type_field: "double_click",
            x: 3,
            y: 4,
        };
        let value = to_value(&dca).unwrap();
        assert_eq!(value, json!({"type": "double_click", "x": 3, "y": 4}));
    }

    #[test]
    fn test_drag_action_path_json() {
        let dap = DragActionPath { x: 5, y: 6 };
        let value = to_value(&dap).unwrap();
        assert_eq!(value, json!({"x": 5, "y": 6}));
    }

    #[test]
    fn test_drag_action_json() {
        let da = DragAction {
            type_field: "drag",
            path: vec![DragActionPath { x: 1, y: 2 }],
        };
        let value = to_value(&da).unwrap();
        assert_eq!(value, json!({"type": "drag", "path": [{"x": 1, "y": 2}]}));
    }

    #[test]
    fn test_key_press_action_json() {
        let kpa = KeyPressAction {
            type_field: "keypress",
            keys: vec!["a", "b"],
        };
        let value = to_value(&kpa).unwrap();
        assert_eq!(value, json!({"type": "keypress", "keys": ["a", "b"]}));
    }

    #[test]
    fn test_move_action_json() {
        let ma = MoveAction {
            type_field: "move",
            x: 7,
            y: 8,
        };
        let value = to_value(&ma).unwrap();
        assert_eq!(value, json!({"type": "move", "x": 7, "y": 8}));
    }

    #[test]
    fn test_screenshot_action_json() {
        let sa = ScreenshotAction {
            type_field: "screenshot",
        };
        let value = to_value(&sa).unwrap();
        assert_eq!(value, json!({"type": "screenshot"}));
    }

    #[test]
    fn test_scroll_action_json() {
        let sa = ScrollAction {
            type_field: "scroll",
            scroll_x: 1,
            scroll_y: 2,
            x: 3,
            y: 4,
        };
        let value = to_value(&sa).unwrap();
        assert_eq!(
            value,
            json!({"type": "scroll", "scroll_x": 1, "scroll_y": 2, "x": 3, "y": 4})
        );
    }

    #[test]
    fn test_type_action_json() {
        let ta = TypeAction {
            type_field: "type",
            text: "abc",
        };
        let value = to_value(&ta).unwrap();
        assert_eq!(value, json!({"type": "type", "text": "abc"}));
    }

    #[test]
    fn test_wait_action_json() {
        let wa = WaitAction { type_field: "wait" };
        let value = to_value(&wa).unwrap();
        assert_eq!(value, json!({"type": "wait"}));
    }

    #[test]
    fn test_computer_tool_action_json() {
        let cta = ComputerToolAction::Click(ClickAction {
            button: "left",
            type_field: "click",
            x: 1,
            y: 2,
        });
        let value = to_value(&cta).unwrap();
        assert_eq!(
            value,
            json!({"button": "left", "type": "click", "x": 1, "y": 2})
        );
    }

    #[test]
    fn test_pending_safety_checks_json() {
        let psc = PendingSafetyChecks {
            code: "c",
            id: "i",
            message: "m",
        };
        let value = to_value(&psc).unwrap();
        assert_eq!(value, json!({"code": "c", "id": "i", "message": "m"}));
    }

    #[test]
    fn test_computer_tool_call_item_json() {
        let ctc = ComputerToolCallItem {
            action: ComputerToolAction::Click(ClickAction {
                button: "left",
                type_field: "click",
                x: 1,
                y: 2,
            }),
            call_id: "cid",
            id: "id",
            pending_safety_checks: vec![PendingSafetyChecks {
                code: "c",
                id: "i",
                message: "m",
            }],
            status: Status::Completed,
            type_field: "computer_call",
        };
        let value = to_value(&ctc).unwrap();
        assert_eq!(
            value,
            json!({
                "action": {"button": "left", "type": "click", "x": 1, "y": 2},
                "call_id": "cid",
                "id": "id",
                "pending_safety_checks": [{"code": "c", "id": "i", "message": "m"}],
                "status": "completed",
                "type": "computer_call"
            })
        );
    }

    #[test]
    fn test_output_json() {
        let o = Output {
            type_field: "computer_screenshot",
            file_id: Some("fid"),
            image_url: Some("url"),
        };
        let value = to_value(&o).unwrap();
        assert_eq!(
            value,
            json!({"type": "computer_screenshot", "file_id": "fid", "image_url": "url"})
        );
    }

    #[test]
    fn test_acknowledged_safety_checks_json() {
        let asc = AcknowledgedSafetyChecks {
            id: "id",
            code: Some("c"),
            message: Some("m"),
        };
        let value = to_value(&asc).unwrap();
        assert_eq!(value, json!({"id": "id", "code": "c", "message": "m"}));
    }

    #[test]
    fn test_computer_tool_call_output_item_json() {
        let cto = ComputerToolCallOutputItem {
            call_id: "cid",
            output: Output {
                type_field: "computer_screenshot",
                file_id: Some("fid"),
                image_url: Some("url"),
            },
            type_field: "computer_call_output",
            acknowledged_safety_checks: Some(vec![AcknowledgedSafetyChecks {
                id: "id",
                code: Some("c"),
                message: Some("m"),
            }]),
            id: Some("oid"),
            status: Some(Status::Completed),
        };
        let value = to_value(&cto).unwrap();
        assert_eq!(
            value,
            json!({
                "call_id": "cid",
                "output": {"type": "computer_screenshot", "file_id": "fid", "image_url": "url"},
                "type": "computer_call_output",
                "acknowledged_safety_checks": [{"id": "id", "code": "c", "message": "m"}],
                "id": "oid",
                "status": "completed"
            })
        );
    }

    #[test]
    fn test_web_search_tool_call_item_json() {
        let wstc = WebSearchToolCallItem {
            id: "wid",
            status: "completed",
            type_field: "web_search_call",
        };
        let value = to_value(&wstc).unwrap();
        assert_eq!(
            value,
            json!({"id": "wid", "status": "completed", "type": "web_search_call"})
        );
    }

    #[test]
    fn test_function_tool_call_item_json() {
        let ftc = FunctionToolCallItem {
            arguments: "{}",
            call_id: "cid",
            name: "func",
            type_field: "function_call",
            id: Some("fid"),
            status: Some(Status::Completed),
        };
        let value = to_value(&ftc).unwrap();
        assert_eq!(
            value,
            json!({"arguments": "{}", "call_id": "cid", "name": "func", "type": "function_call", "id": "fid", "status": "completed"})
        );
    }

    #[test]
    fn test_summary_json() {
        let s = Summary {
            text: "sum",
            type_field: "summary_text",
        };
        let value = to_value(&s).unwrap();
        assert_eq!(value, json!({"text": "sum", "type": "summary_text"}));
    }

    #[test]
    fn test_reasoning_item_json() {
        let r = ReasoningItem {
            id: "rid",
            summary: vec![Summary {
                text: "sum",
                type_field: "summary_text",
            }],
            type_field: "reasoning",
            encrypted_content: Some("enc"),
            status: Some(Status::Completed),
        };
        let value = to_value(&r).unwrap();
        assert_eq!(
            value,
            json!({
                "id": "rid",
                "summary": [{"text": "sum", "type": "summary_text"}],
                "type": "reasoning",
                "encrypted_content": "enc",
                "status": "completed"
            })
        );
    }
}
