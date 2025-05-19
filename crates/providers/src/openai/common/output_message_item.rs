use crate::openai::common::status::Status;
use crate::openai::errors::ConversionError;
use crate::openai::request::input_models::common::Role;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileCitation {
    pub file_id: String,
    pub index: usize,
    #[serde(rename = "type")]
    pub type_field: String,
}

impl FileCitation {
    pub fn new(file_id: impl Into<String>, index: usize) -> Self {
        Self {
            file_id: file_id.into(),
            index,
            type_field: "file_citation".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlCitation {
    pub end_index: String,
    pub start_index: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
}

impl UrlCitation {
    pub fn new(
        end_index: impl Into<String>,
        start_index: impl Into<String>,
        title: impl Into<String>,
        url: impl Into<String>,
    ) -> Self {
        Self {
            end_index: end_index.into(),
            start_index: start_index.into(),
            title: title.into(),
            url: url.into(),
            type_field: "url_citation".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilePath {
    pub file_id: String,
    pub index: usize,
    #[serde(rename = "type")]
    pub type_field: String,
}

impl FilePath {
    pub fn new(file_id: impl Into<String>, index: usize) -> Self {
        Self {
            file_id: file_id.into(),
            index,
            type_field: "file_path".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Annotation {
    FileCitation(FileCitation),
    UrlCitation(UrlCitation),
    FilePath(FilePath),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputText {
    pub annotations: Vec<Annotation>,
    pub text: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

impl OutputText {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            annotations: vec![],
            text: text.into(),
            type_field: "output_text".to_string(),
        }
    }

    pub fn extend_annotations(mut self, annotation: Vec<Annotation>) -> Self {
        self.annotations.extend(annotation);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Refusal {
    pub refusal: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

impl Refusal {
    pub fn new(refusal: impl Into<String>) -> Self {
        Self {
            refusal: refusal.into(),
            type_field: "refusal".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OutputContent {
    OutputText(OutputText),
    Refusal(Refusal),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputMessageItem {
    pub content: Vec<OutputContent>,
    pub id: String,
    pub role: Role,
    pub status: Status,
    #[serde(rename = "type")]
    pub type_field: String,
}

impl OutputMessageItem {
    pub fn new(id: impl Into<String>, status: impl AsRef<str>) -> Result<Self, ConversionError> {
        Ok(Self {
            content: vec![],
            id: id.into(),
            role: Role::Assistant,
            status: Status::from_str(status.as_ref())?,
            type_field: "message".to_string(),
        })
    }
}
