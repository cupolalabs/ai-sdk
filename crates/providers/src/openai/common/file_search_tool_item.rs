use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

use crate::openai::common::status::Status;
use crate::openai::errors::ConversionError;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FileSearchToolCallResult<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<&'a str>,
}

impl<'a> FileSearchToolCallResult<'a> {
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

    pub fn file_id(mut self, value: &'a str) -> Self {
        self.file_id = Some(value);
        self
    }

    pub fn filename(mut self, value: &'a str) -> Self {
        self.filename = Some(value);
        self
    }

    pub fn score(mut self, value: usize) -> Self {
        self.score = Some(value);
        self
    }

    pub fn text(mut self, value: &'a str) -> Self {
        self.text = Some(value);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct FileSearchToolCallItem<'a> {
    pub id: &'a str,
    pub queries: Vec<&'a str>,
    pub status: Status,
    #[serde(rename = "type")]
    pub type_field: &'a str,
    pub results: Vec<FileSearchToolCallResult<'a>>,
}

impl<'a> FileSearchToolCallItem<'a> {
    pub fn new(id: &'a str, status: &'a str) -> Result<Self, ConversionError> {
        Ok(Self {
            id,
            queries: vec![],
            status: Status::from_str(status)?,
            type_field: "file_search_call",
            results: vec![],
        })
    }

    pub fn extend_queries(mut self, queries: Vec<&'a str>) -> Self {
        self.queries.extend(queries);
        self
    }

    pub fn extend_results(mut self, results: Vec<FileSearchToolCallResult<'a>>) -> Self {
        self.results.extend(results);
        self
    }
}
