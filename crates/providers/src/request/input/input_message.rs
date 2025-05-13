use crate::request::input::common::{Content, Role};
use std::str::FromStr;

use crate::errors::ConversionError;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
