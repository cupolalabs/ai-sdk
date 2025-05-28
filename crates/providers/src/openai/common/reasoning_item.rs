use crate::openai::common::status::Status;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ReasoningItemSummary {
    #[serde(rename = "summary_text")]
    SummaryText { text: String },
}

impl ReasoningItemSummary {
    pub fn new(text: impl Into<String>) -> Self {
        Self::SummaryText { text: text.into() }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReasoningItem {
    pub id: String,
    pub summary: Vec<ReasoningItemSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl ReasoningItem {
    pub fn new(id: impl Into<String>, summary: Vec<ReasoningItemSummary>) -> Self {
        Self {
            id: id.into(),
            summary,
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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn it_builds_reasoning_item() {
        let item = ReasoningItem::new(
            "test-id",
            vec![
                ReasoningItemSummary::new("summary-1"),
                ReasoningItemSummary::new("summary-2"),
            ],
        )
        .encrypted_content("encrypted-content")
        .status(Status::InProgress);

        let expected = json!({
            "id": "test-id",
            "summary": [
                {
                    "text": "summary-1",
                    "type": "summary_text"
                },
                {
                    "text": "summary-2",
                    "type": "summary_text"
                }
            ],
            "encrypted_content": "encrypted-content",
            "status": "in_progress"
        });

        assert_eq!(serde_json::to_value(item).unwrap(), expected);
    }
}
