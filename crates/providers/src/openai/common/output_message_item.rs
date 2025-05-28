use crate::openai::common::status::Status;
use crate::openai::request::input_models::common::Role;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Annotation {
    #[serde(rename = "file_citation")]
    FileCitation { file_id: String, index: usize },
    #[serde(rename = "url_citation")]
    UrlCitation {
        end_index: usize,
        start_index: usize,
        title: String,
        url: String,
    },
    #[serde(rename = "file_path")]
    FilePath { file_id: String, index: usize },
}

impl Annotation {
    pub fn file_citation(file_id: String, index: usize) -> Self {
        Self::FileCitation { file_id, index }
    }

    pub fn url_citation(end_index: usize, start_index: usize, title: String, url: String) -> Self {
        Self::UrlCitation {
            end_index,
            start_index,
            title,
            url,
        }
    }

    pub fn file_path(file_id: String, index: usize) -> Self {
        Self::FilePath { file_id, index }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OutputContent {
    #[serde(rename = "output_text")]
    OutputText {
        annotations: Vec<Annotation>,
        text: String,
    },
    #[serde(rename = "refusal")]
    Refusal { refusal: String },
}

impl OutputContent {
    pub fn output_text(annotations: Vec<Annotation>, text: String) -> Self {
        Self::OutputText { annotations, text }
    }

    pub fn refusal(refusal: String) -> Self {
        Self::Refusal { refusal }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputMessageItem {
    pub content: Vec<OutputContent>,
    pub id: String,
    pub role: Role,
    pub status: Status,
}

impl OutputMessageItem {
    pub fn new(content: Vec<OutputContent>, id: impl Into<String>, status: Status) -> Self {
        Self {
            content,
            id: id.into(),
            role: Role::Assistant,
            status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn build_output_message_item() {
        let output_contents = vec![
            OutputContent::output_text(
                vec![
                    Annotation::file_citation("file-id".to_string(), 8),
                    Annotation::url_citation(
                        64,
                        64,
                        "test-title".to_string(),
                        "test-url".to_string(),
                    ),
                    Annotation::file_path("test-id".to_string(), 16),
                ],
                "test-output-text-1".to_string(),
            ),
            OutputContent::refusal("refusal-explanation".to_string()),
        ];

        let output_message_item =
            OutputMessageItem::new(output_contents, "test-id", Status::InProgress);

        let expected = json!({
            "content": [
                {
                    "type": "output_text",
                    "annotations": [
                        {
                            "type": "file_citation",
                            "file_id": "file-id",
                            "index": 8
                        },
                        {
                            "type": "url_citation",
                            "end_index": 64,
                            "start_index": 64,
                            "title": "test-title",
                            "url": "test-url"
                        },
                        {
                            "type": "file_path",
                            "file_id": "test-id",
                            "index": 16
                        }
                    ],
                    "text": "test-output-text-1".to_string()
                },
                {
                    "type": "refusal",
                    "refusal": "refusal-explanation"
                }
            ],
            "id": "test-id",
            "role": "assistant",
            "status": "in_progress",
        });

        assert_eq!(serde_json::to_value(output_message_item).unwrap(), expected);
    }
}
