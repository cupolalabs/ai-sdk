use serde::{Deserialize, Serialize};

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
