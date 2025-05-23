use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

use crate::openai::common::status::Status;
use crate::openai::errors::ConversionError;

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
    #[serde(rename = "type")]
    pub results: Vec<FileSearchToolCallResult>,
}

impl FileSearchToolCallItem {
    pub fn new(id: impl Into<String>, status: impl AsRef<str>) -> Result<Self, ConversionError> {
        Ok(Self {
            id: id.into(),
            queries: vec![],
            status: Status::from_str(status.as_ref())?,
            results: vec![],
        })
    }

    pub fn extend_queries(mut self, queries: Vec<impl Into<String>>) -> Self {
        self.queries.extend(queries.into_iter().map(|q| q.into()));
        self
    }

    pub fn extend_results(mut self, results: Vec<FileSearchToolCallResult>) -> Self {
        self.results.extend(results);
        self
    }
}
