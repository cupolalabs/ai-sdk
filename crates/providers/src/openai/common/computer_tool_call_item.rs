use serde::{Deserialize, Serialize};

use crate::openai::common::status::Status;
use crate::openai::errors::InputError;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClickAction {
    pub button: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub x: usize,
    pub y: usize,
}

impl ClickAction {
    const BUTTON: [&'static str; 5] = ["left", "right", "wheel", "back", "forward"];

    pub fn new(button: impl Into<String>, x: usize, y: usize) -> Result<Self, InputError> {
        let button_str = button.into();
        if Self::BUTTON.contains(&button_str.as_str()) {
            Ok(Self {
                button: button_str,
                type_field: "click".to_string(),
                x,
                y,
            })
        } else {
            Err(InputError::InvalidButton(button_str))
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DoubleClickAction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub x: usize,
    pub y: usize,
}

impl DoubleClickAction {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            type_field: "double_click".to_string(),
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
pub struct DragAction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub path: Vec<DragActionPath>,
}

impl DragAction {
    pub fn new(path: Vec<DragActionPath>) -> Self {
        Self {
            type_field: "drag".to_string(),
            path,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyPressAction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub keys: Vec<String>,
}

impl KeyPressAction {
    pub fn new(keys: Vec<impl Into<String>>) -> Self {
        Self {
            type_field: "keypress".to_string(),
            keys: keys.into_iter().map(|k| k.into()).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveAction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub x: usize,
    pub y: usize,
}

impl MoveAction {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            type_field: "move".to_string(),
            x,
            y,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScreenshotAction {
    #[serde(rename = "type")]
    pub type_field: String,
}

impl ScreenshotAction {
    pub fn new() -> Self {
        Self {
            type_field: "screenshot".to_string(),
        }
    }
}

impl Default for ScreenshotAction {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScrollAction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub scroll_x: usize,
    pub scroll_y: usize,
    pub x: usize,
    pub y: usize,
}

impl ScrollAction {
    pub fn new(scroll_x: usize, scroll_y: usize, x: usize, y: usize) -> Self {
        Self {
            type_field: "scroll".to_string(),
            scroll_x,
            scroll_y,
            x,
            y,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TypeAction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: String,
}

impl TypeAction {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            type_field: "type".to_string(),
            text: text.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WaitAction {
    #[serde(rename = "type")]
    pub type_field: String,
}

impl WaitAction {
    pub fn new() -> Self {
        Self {
            type_field: "wait".to_string(),
        }
    }
}

impl Default for WaitAction {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComputerToolAction {
    Click(ClickAction),
    DoubleClick(DoubleClickAction),
    Drag(DragAction),
    KeyPress(KeyPressAction),
    Move(MoveAction),
    Screenshot(ScreenshotAction),
    Scroll(ScrollAction),
    Type(TypeAction),
    Wait(WaitAction),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PendingSafetyChecks {
    pub code: String,
    pub id: String,
    pub message: String,
}

impl PendingSafetyChecks {
    pub fn new(code: impl Into<String>, id: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            id: id.into(),
            message: message.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputerToolCallItem {
    pub action: ComputerToolAction,
    pub call_id: String,
    pub id: String,
    pub pending_safety_checks: Vec<PendingSafetyChecks>,
    pub status: Status,
    #[serde(rename = "type")]
    pub type_field: String,
}

impl ComputerToolCallItem {
    pub fn new(
        action: ComputerToolAction,
        call_id: impl Into<String>,
        id: impl Into<String>,
        pending_safety_checks: Vec<PendingSafetyChecks>,
        status: Status,
    ) -> Self {
        Self {
            action,
            call_id: call_id.into(),
            id: id.into(),
            pending_safety_checks,
            status,
            type_field: "computer_call".to_string(),
        }
    }
}
