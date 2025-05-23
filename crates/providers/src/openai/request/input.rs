use crate::openai::request::input_models::{
    input_message::InputMessage, input_reference::InputReference, item::Item,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputItemList {
    InputMessage(InputMessage),
    Item(Item),
    ItemReference(InputReference),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Input {
    Messages(Vec<InputItemList>),
    Message(String),
}

impl From<String> for Input {
    fn from(value: String) -> Self {
        Self::Message(value)
    }
}

impl From<Vec<InputItemList>> for Input {
    fn from(input_item_list: Vec<InputItemList>) -> Self {
        Self::Messages(input_item_list)
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::Message("".into())
    }
}

impl Input {
    pub fn from_text(value: impl Into<String>) -> Self {
        Self::Message(value.into())
    }

    pub fn from_input_item_list(input_item_list: Vec<InputItemList>) -> Self {
        Self::Messages(input_item_list)
    }
}
