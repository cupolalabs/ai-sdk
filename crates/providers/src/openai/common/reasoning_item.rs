use crate::openai::common::status::Status;
use crate::openai::request::input_models::item::Summary;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReasoningItem {
    pub id: String,
    pub summary: Vec<Summary>,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl ReasoningItem {
    pub fn new(id: impl Into<String>, summary: Vec<Summary>) -> Self {
        Self {
            id: id.into(),
            summary,
            type_field: "reasoning".to_string(),
            encrypted_content: None,
            status: None,
        }
    }

    pub fn encrypted_content(mut self, value: impl Into<String>) -> Self {
        self.encrypted_content = Some(value.into());
        self
    }

    pub fn status(mut self, value: Status) -> Self {
        self.status = Some(value);
        self
    }
}
