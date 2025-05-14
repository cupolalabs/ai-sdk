use crate::request::input::common::{Content, Role};
use std::str::FromStr;

use crate::errors::ConversionError;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct TextInput<'a> {
    pub role: Role,
    pub content: &'a str,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<&'a str>,
}

impl<'a> TextInput<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            role: Role::default(),
            content,
            type_field: None,
        }
    }

    pub fn role(mut self, role: &'a str) -> Result<Self, ConversionError> {
        self.role = Role::from_str(role)?;
        Ok(self)
    }

    pub fn insert_type(mut self) -> Self {
        self.type_field = Some("message");
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct InputItemContentList<'a> {
    pub role: Role,
    pub content: Vec<Content<'a>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<&'a str>,
}

impl<'a> InputItemContentList<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn role(mut self, role: &'a str) -> Result<Self, ConversionError> {
        self.role = Role::from_str(role)?;
        Ok(self)
    }

    pub fn insert_type(mut self) -> Self {
        self.type_field = Some("message");
        self
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
#[serde(untagged)]
pub enum InputMessage<'a> {
    TextInput(TextInput<'a>),
    InputItemContentList(InputItemContentList<'a>),
}

impl<'a> From<TextInput<'a>> for InputMessage<'a> {
    fn from(text_input: TextInput<'a>) -> Self {
        InputMessage::TextInput(text_input)
    }
}

impl<'a> From<InputItemContentList<'a>> for InputMessage<'a> {
    fn from(input_item_content_list: InputItemContentList<'a>) -> Self {
        InputMessage::InputItemContentList(input_item_content_list)
    }
}

#[cfg(test)]
mod tests {
    use crate::input::TextContent;

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
