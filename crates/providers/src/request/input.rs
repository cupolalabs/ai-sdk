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
