use crate::openai::errors::ConversionError;
use crate::openai::request::input_models::common::{Content, Role};
use crate::openai::request::input_models::input_reference::InputReference;
use crate::openai::request::input_models::item::Item;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextInput {
    pub role: Role,
    pub content: String,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<String>,
}

impl TextInput {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            role: Role::default(),
            content: content.into(),
            type_field: None,
        }
    }

    pub fn role(mut self, role: impl AsRef<str>) -> Result<Self, ConversionError> {
        self.role = Role::from_str(role.as_ref())?;
        Ok(self)
    }

    pub fn insert_type(mut self) -> Self {
        self.type_field = Some("message".to_string());
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InputItemContentList {
    pub role: Role,
    pub content: Vec<Content>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<String>,
}

impl InputItemContentList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn role(mut self, role: impl AsRef<str>) -> Result<Self, ConversionError> {
        self.role = Role::from_str(role.as_ref())?;
        Ok(self)
    }

    pub fn insert_type(mut self) -> Self {
        self.type_field = Some("message".to_string());
        self
    }
}

impl From<Item> for InputItemContentList {
    fn from(_item: Item) -> Self {
        Self {
            role: Role::default(),
            content: Vec::new(),
            type_field: Some("message".to_string()),
        }
    }
}

impl From<InputReference> for InputItemContentList {
    fn from(_reference: InputReference) -> Self {
        Self {
            role: Role::default(),
            content: Vec::new(),
            type_field: Some("message".to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputMessage {
    TextInput(TextInput),
    InputItemContentList(InputItemContentList),
}

impl From<TextInput> for InputMessage {
    fn from(text_input: TextInput) -> Self {
        InputMessage::TextInput(text_input)
    }
}

impl From<InputItemContentList> for InputMessage {
    fn from(content_list: InputItemContentList) -> Self {
        InputMessage::InputItemContentList(content_list)
    }
}

#[cfg(test)]
mod tests {
    use crate::openai::request::input_models::common::TextContent;

    use super::*;

    #[test]
    fn test_json_values() {
        let text_input = TextInput::new("Hello, world!");
        let input_message: InputMessage = text_input.clone().into();
        assert_eq!(input_message, InputMessage::TextInput(text_input));

        let json_value = serde_json::to_value(&input_message).unwrap();
        assert_eq!(
            json_value,
            serde_json::json!({
                "role": "user",
                "content": "Hello, world!"
            })
        );
    }

    #[test]
    fn test_json_values_input_item_content_list() {
        let mut input_item_content_list = InputItemContentList::new()
            .insert_type()
            .role("developer")
            .unwrap();

        input_item_content_list
            .content
            .push(Content::Text(TextContent::new().text("Hello, world!")));

        let input_message: InputMessage = input_item_content_list.clone().into();
        assert_eq!(
            input_message,
            InputMessage::InputItemContentList(input_item_content_list)
        );

        let json_value = serde_json::to_value(&input_message).unwrap();
        assert_eq!(
            json_value,
            serde_json::json!({
                "role": "developer",
                "content": [
                    {
                        "type": "input_text",
                        "text": "Hello, world!"
                    }
                ],
                "type": "message"
            })
        );
    }
}
