use crate::openai::common::status::Status;
use crate::openai::errors::ConversionError;
use crate::openai::request::input::common::Role;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileCitation<'a> {
    pub file_id: &'a str,
    pub index: usize,
    #[serde(rename = "type")]
    pub type_field: &'a str,
}

impl<'a> FileCitation<'a> {
    pub fn new(file_id: &'a str, index: usize) -> Self {
        Self {
            file_id,
            index,
            type_field: "file_citation",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlCitation<'a> {
    pub end_index: &'a str,
    pub start_index: &'a str,
    pub title: &'a str,
    #[serde(rename = "type")]
    pub type_field: &'a str,
    pub url: &'a str,
}

impl<'a> UrlCitation<'a> {
    pub fn new(end_index: &'a str, start_index: &'a str, title: &'a str, url: &'a str) -> Self {
        Self {
            end_index,
            start_index,
            title,
            url,
            type_field: "url_citation",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilePath<'a> {
    pub file_id: &'a str,
    pub index: usize,
    #[serde(rename = "type")]
    pub type_field: &'a str,
}

impl<'a> FilePath<'a> {
    pub fn new(file_id: &'a str, index: usize) -> Self {
        Self {
            file_id,
            index,
            type_field: "file_path",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
#[serde(untagged)]
pub enum Annotation<'a> {
    FileCitation(FileCitation<'a>),
    UrlCitation(UrlCitation<'a>),
    FilePath(FilePath<'a>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputText<'a> {
    pub annotations: Vec<Annotation<'a>>,
    pub text: &'a str,
    #[serde(rename = "type")]
    pub type_field: &'a str,
}

impl<'a> OutputText<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            annotations: vec![],
            text,
            type_field: "output_text",
        }
    }

    pub fn extend_annotations(mut self, annotation: Vec<Annotation<'a>>) -> Self {
        self.annotations.extend(annotation);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Refusal<'a> {
    pub refusal: &'a str,
    #[serde(rename = "type")]
    pub type_field: &'a str,
}

impl<'a> Refusal<'a> {
    pub fn new(refusal: &'a str) -> Self {
        Self {
            refusal,
            type_field: "refusal",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
#[serde(untagged)]
pub enum OutputContent<'a> {
    OutputText(OutputText<'a>),
    Refusal(Refusal<'a>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct OutputMessageItem<'a> {
    pub content: Vec<OutputContent<'a>>,
    pub id: &'a str,
    pub role: Role,
    pub status: Status,
    #[serde(rename = "type")]
    pub type_field: &'a str,
}

impl<'a> OutputMessageItem<'a> {
    pub fn new(id: &'a str, status: &'a str) -> Result<Self, ConversionError> {
        Ok(Self {
            content: vec![],
            id,
            role: Role::Assistant,
            status: Status::from_str(status)?,
            type_field: "message",
        })
    }
}
