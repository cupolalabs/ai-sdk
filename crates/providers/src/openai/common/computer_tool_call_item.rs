use serde::{Deserialize, Serialize};

use crate::openai::common::status::Status;
use crate::openai::errors::InputError;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DragActionPath {
    x: usize,
    y: usize,
}

impl DragActionPath {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ComputerToolAction {
    #[serde(rename = "click")]
    Click { button: String, x: usize, y: usize },
    #[serde(rename = "double_click")]
    DoubleClick { x: usize, y: usize },
    #[serde(rename = "drag")]
    Drag { path: Vec<DragActionPath> },
    #[serde(rename = "keypress")]
    KeyPress { keys: Vec<String> },
    #[serde(rename = "move")]
    Move { x: usize, y: usize },
    #[serde(rename = "screenshot")]
    Screenshot,
    #[serde(rename = "scroll")]
    Scroll {
        scroll_x: usize,
        scroll_y: usize,
        x: usize,
        y: usize,
    },
    #[serde(rename = "type")]
    Type { text: String },
    #[serde(rename = "wait")]
    Wait,
}

impl ComputerToolAction {
    const BUTTON: [&'static str; 5] = ["left", "right", "wheel", "back", "forward"];

    pub fn click(button: impl Into<String>, x: usize, y: usize) -> Result<Self, InputError> {
        let button_str = button.into();
        if Self::BUTTON.contains(&button_str.as_str()) {
            Ok(Self::Click {
                button: button_str,
                x,
                y,
            })
        } else {
            Err(InputError::InvalidButtonForClickAction(button_str))
        }
    }

    pub fn double_click(x: usize, y: usize) -> Self {
        Self::DoubleClick { x, y }
    }

    pub fn drag(path: Vec<DragActionPath>) -> Self {
        Self::Drag { path }
    }

    pub fn keypress(keys: Vec<String>) -> Self {
        Self::KeyPress { keys }
    }

    pub fn move_action(x: usize, y: usize) -> Self {
        Self::Move { x, y }
    }

    pub fn screenshot() -> Self {
        Self::Screenshot
    }

    pub fn scroll(scroll_x: usize, scroll_y: usize, x: usize, y: usize) -> Self {
        Self::Scroll {
            scroll_x,
            scroll_y,
            x,
            y,
        }
    }

    pub fn type_action(text: String) -> Self {
        Self::Type { text }
    }

    pub fn wait() -> Self {
        Self::Wait
    }
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
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn it_builds_computer_tool_action() {
        let actions: Vec<ComputerToolAction> = vec![
            ComputerToolAction::click("left", 64, 64).unwrap(),
            ComputerToolAction::double_click(64, 64),
            ComputerToolAction::drag(vec![
                DragActionPath::new(64, 64),
                DragActionPath::new(128, 128),
            ]),
            ComputerToolAction::keypress(vec!["Alt".into(), "Ctrl".into()]),
            ComputerToolAction::move_action(64, 64),
            ComputerToolAction::screenshot(),
            ComputerToolAction::scroll(64, 64, 64, 64),
            ComputerToolAction::type_action("action".into()),
            ComputerToolAction::wait(),
        ];

        let expected = vec![
            json!({
                "type": "click",
                "button": "left",
                "x": 64,
                "y": 64,
            }),
            json!({
                "type": "double_click",
                "x": 64,
                "y": 64,
            }),
            json!({
                "type": "drag",
                "path": [
                    { "x": 64, "y": 64 },
                    { "x": 128, "y": 128 }
                ]
            }),
            json!({
                "type": "keypress",
                "keys": ["Alt", "Ctrl"]
            }),
            json!({
                "type": "move",
                "x": 64,
                "y": 64
            }),
            json!({
                "type": "screenshot"
            }),
            json!({
                "type": "scroll",
                "scroll_x": 64,
                "scroll_y": 64,
                "x": 64,
                "y": 64,
            }),
            json!({
                "type": "type",
                "text": "action"
            }),
            json!({
                "type": "wait"
            }),
        ];

        for (index, computer_tool_action) in actions.iter().enumerate() {
            assert_eq!(
                serde_json::to_value(computer_tool_action).unwrap(),
                expected[index]
            );
        }
    }

    #[test]
    fn it_builds_computer_tool_call_item() {
        let item = ComputerToolCallItem::new(
            ComputerToolAction::Screenshot,
            "test-call-id".to_string(),
            "test-id".to_string(),
            vec![PendingSafetyChecks::new(
                "test-code",
                "test-id",
                "test-message",
            )],
            Status::InProgress,
        );

        let expected = json!({
            "action": {
                "type": "screenshot",
            },
            "call_id": "test-call-id",
            "id": "test-id",
            "pending_safety_checks": [
                {
                    "code": "test-code",
                    "id": "test-id",
                    "message": "test-message"
                }
            ],
            "status": "in_progress"
        });

        assert_eq!(serde_json::to_value(item).unwrap(), expected);
    }
}
