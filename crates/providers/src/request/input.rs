pub mod common;
pub mod input_message;
pub mod input_reference;
pub mod item;

pub use common::*;
pub use input_message::*;
pub use input_reference::*;
pub use item::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
#[serde(untagged)]
pub enum InputItemList<'a> {
    InputMessage(Vec<InputMessage<'a>>),
    Item(Item<'a>),
    ItemReference(InputReference<'a>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Input<'a> {
    TextInput(&'a str),
    InputItemList(InputItemList<'a>),
}

impl<'a> Input<'a> {
    pub fn from_text(text: &'a str) -> Self {
        Input::TextInput(text)
    }

    pub fn from_messages(messages: Vec<InputMessage<'a>>) -> Self {
        Input::InputItemList(InputItemList::InputMessage(messages))
    }

    pub fn from_item(item: Item<'a>) -> Self {
        Input::InputItemList(InputItemList::Item(item))
    }

    pub fn from_item_reference(reference: InputReference<'a>) -> Self {
        Input::InputItemList(InputItemList::ItemReference(reference))
    }
}
