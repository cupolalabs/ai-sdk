use crate::openai::common::status::Status;
use crate::openai::request::input::item::Summary;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReasoningItem<'a> {
    pub id: &'a str,
    pub summary: Vec<Summary<'a>>,
    #[serde(rename = "type")]
    pub type_field: &'a str,
    pub encrypted_content: Option<&'a str>,
    pub status: Option<Status>,
}

impl<'a> ReasoningItem<'a> {
    pub fn new(id: &'a str, summary: Vec<Summary<'a>>) -> Self {
        Self {
            id,
            summary,
            type_field: "reasoning",
            encrypted_content: None,
            status: None,
        }
    }

    pub fn encrypted_content(mut self, value: &'a str) -> Self {
        self.encrypted_content = Some(value);
        self
    }

    pub fn status(mut self, value: Status) -> Self {
        self.status = Some(value);
        self
    }
}
