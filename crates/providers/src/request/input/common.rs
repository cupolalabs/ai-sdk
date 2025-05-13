use crate::errors::ConversionError;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    InProgress,
    Completed,
    Incomplete,
    Failed,
}

impl FromStr for Status {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "in_progress" => Ok(Status::InProgress),
            "completed" => Ok(Status::Completed),
            "incomplete" => Ok(Status::Incomplete),
            "failed" => Ok(Status::Failed),
            _ => Err(ConversionError::FromStr(s.to_string())),
        }
    }
}

#[derive(Debug, Default, PartialEq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    #[default]
    User,
    Assistant,
    System,
    Developer,
}

impl FromStr for Role {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user" => Ok(Role::User),
            "assistant" => Ok(Role::Assistant),
            "system" => Ok(Role::System),
            "developer" => Ok(Role::Developer),
            _ => Err(ConversionError::FromStr(s.to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    InputText,
    InputImage,
    InputFile,
}

impl FromStr for ContentType {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "input_text" => Ok(ContentType::InputText),
            "input_image" => Ok(ContentType::InputImage),
            "input_file" => Ok(ContentType::InputFile),
            _ => Err(ConversionError::FromStr(s.to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TextContent<'a> {
    #[serde(rename = "type")]
    pub type_field: ContentType, // always InputText
    pub text: &'a str,
}

impl<'a> TextContent<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            type_field: ContentType::InputText,
            text,
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageDetail {
    High,
    Low,
    #[default]
    Auto,
}

impl FromStr for ImageDetail {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "high" => Ok(ImageDetail::High),
            "low" => Ok(ImageDetail::Low),
            "auto" => Ok(ImageDetail::Auto),
            _ => Err(ConversionError::FromStr(s.to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImageContent<'a> {
    #[serde(rename = "type")]
    pub type_field: ContentType, // always InputImage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<&'a str>,
    pub detail: ImageDetail,
}

impl Default for ImageContent<'_> {
    fn default() -> Self {
        Self {
            type_field: ContentType::InputImage,
            image_url: None,
            file_id: None,
            detail: ImageDetail::Auto,
        }
    }
}

impl<'a> ImageContent<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn image_url(mut self, value: &'a str) -> Self {
        self.image_url = Some(value);
        self
    }

    pub fn file_id(mut self, value: &'a str) -> Self {
        self.file_id = Some(value);
        self
    }

    pub fn detail(mut self, value: &'a str) -> Result<Self, ConversionError> {
        self.detail = ImageDetail::from_str(value)?;
        Ok(self)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FileContent<'a> {
    #[serde(rename = "type")]
    pub type_field: ContentType, // always InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<&'a str>,
}

impl Default for FileContent<'_> {
    fn default() -> Self {
        Self {
            type_field: ContentType::InputFile,
            file_id: None,
            file_data: None,
            filename: None,
        }
    }
}

impl<'a> FileContent<'a> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn file_id(mut self, value: &'a str) -> Self {
        self.file_id = Some(value);
        self
    }

    pub fn file_data(mut self, value: &'a str) -> Self {
        self.file_data = Some(value);
        self
    }

    pub fn filename(mut self, value: &'a str) -> Self {
        self.filename = Some(value);
        self
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
#[serde(untagged)]
pub enum Content<'a> {
    Text(TextContent<'a>),
    Image(ImageContent<'a>),
    File(FileContent<'a>),
}

impl<'a> From<TextContent<'a>> for Content<'a> {
    fn from(text_content: TextContent<'a>) -> Self {
        Self::Text(text_content)
    }
}

impl<'a> From<ImageContent<'a>> for Content<'a> {
    fn from(image_content: ImageContent<'a>) -> Self {
        Self::Image(image_content)
    }
}

impl<'a> From<FileContent<'a>> for Content<'a> {
    fn from(file_content: FileContent<'a>) -> Self {
        Self::File(file_content)
    }
}
