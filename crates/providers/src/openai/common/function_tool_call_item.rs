use serde::{Deserialize, Serialize};

use crate::openai::common::status::Status;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionToolCallItem<'a> {
    pub arguments: &'a str,
    pub call_id: &'a str,
    pub name: &'a str,
    #[serde(rename = "type")]
    pub type_field: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl<'a> FunctionToolCallItem<'a> {
    pub fn new(arguments: &'a str, call_id: &'a str, name: &'a str) -> Self {
        Self {
            arguments,
            call_id,
            name,
            type_field: "function_call",
            id: None,
            status: None,
        }
    }

    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    pub fn status(mut self, value: Status) -> Self {
        self.status = Some(value);
        self
    }
}
