use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSearchToolCallItem<'a> {
    pub id: &'a str,
    pub status: &'a str,
    #[serde(rename = "type")]
    pub type_field: &'a str,
}

impl<'a> WebSearchToolCallItem<'a> {
    pub fn new(id: &'a str, status: &'a str) -> Self {
        Self {
            id,
            status,
            type_field: "web_search_call",
        }
    }
}
