use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::openai::common::status::Status;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FileSearchToolCallResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl FileSearchToolCallResult {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_attribute(mut self, key: String, value: String) -> Self {
        if self.attributes.is_none() {
            self.attributes = Some(HashMap::new());
        }

        if let Some(attrs) = &mut self.attributes {
            attrs.insert(key, value);
        }

        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
        self
    }

    pub fn score(mut self, value: usize) -> Self {
        self.score = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileSearchToolCallItem {
    pub id: String,
    pub queries: Vec<String>,
    pub status: Status,
    pub results: Vec<FileSearchToolCallResult>,
}

impl FileSearchToolCallItem {
    pub fn new(
        id: impl Into<String>,
        queries: Vec<String>,
        status: Status,
        results: Vec<FileSearchToolCallResult>,
    ) -> Self {
        Self {
            id: id.into(),
            queries,
            status,
            results,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn it_builds_file_search_tool_call_item() {
        let item = FileSearchToolCallItem::new(
            "test-id".to_string(),
            vec!["test-query-1".to_string(), "test-query-2".to_string()],
            Status::InProgress,
            vec![FileSearchToolCallResult::new()
                .file_id("file-id")
                .filename("filename")
                .score(5)
                .insert_attribute("location".to_string(), "heaven".to_string())],
        );

        let expected = json!({
            "id": "test-id",
            "queries": ["test-query-1", "test-query-2"],
            "status": "in_progress",
            "results": [
                {
                    "attributes": { "location": "heaven" },
                    "file_id": "file-id",
                    "filename": "filename",
                    "score": 5
                }
            ]
        });

        assert_eq!(serde_json::to_value(item).unwrap(), expected);
    }
}
