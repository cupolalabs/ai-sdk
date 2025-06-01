use serde::{Deserialize, Serialize};

use crate::openai::errors::BuilderError;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSearchToolCallItem {
    pub id: String,
    pub status: String,
}

impl WebSearchToolCallItem {
    pub fn new(id: impl Into<String>, status: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            status: status.into(),
        }
    }
}

pub struct WebSearchToolCallItemBuilder {
    id: String,
    status: String,
}

impl WebSearchToolCallItemBuilder {
    pub fn new(id: impl Into<String>, status: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            status: status.into(),
        }
    }

    pub fn build(self) -> Result<WebSearchToolCallItem, BuilderError> {
        Ok(WebSearchToolCallItem {
            id: self.id,
            status: self.status,
        })
    }
}
