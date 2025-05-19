use crate::openai::request::input_models::{
    input_message::{InputMessage, TextInput},
    input_reference::InputReference,
    item::Item,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputItemList {
    InputMessage(Vec<InputMessage>),
    Item(Item),
    ItemReference(InputReference),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Input {
    Messages(Vec<InputMessage>),
    Message(InputMessage),
}

impl From<InputMessage> for Input {
    fn from(message: InputMessage) -> Self {
        Self::Message(message)
    }
}

impl From<Vec<InputMessage>> for Input {
    fn from(messages: Vec<InputMessage>) -> Self {
        Self::Messages(messages)
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::Messages(Vec::new())
    }
}

impl Input {
    pub fn from_text(text: impl Into<String>) -> Self {
        Self::Message(InputMessage::TextInput(TextInput::new(text)))
    }

    pub fn from_messages(messages: Vec<InputMessage>) -> Self {
        Self::Messages(messages)
    }

    pub fn from_item(item: Item) -> Self {
        Self::Message(InputMessage::InputItemContentList(item.into()))
    }

    pub fn from_item_reference(reference: InputReference) -> Self {
        Self::Message(InputMessage::InputItemContentList(reference.into()))
    }
}
