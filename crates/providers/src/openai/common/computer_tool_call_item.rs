use serde::{Deserialize, Serialize};

use crate::openai::common::status::Status;
use crate::openai::errors::InputError;

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
