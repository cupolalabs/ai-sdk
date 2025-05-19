use crate::openai::errors::ConversionError;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

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
pub struct TextContent {
    #[serde(rename = "type")]
    pub type_field: ContentType, // always InputText
    pub text: String,
}

impl Default for TextContent {
    fn default() -> Self {
        Self {
            type_field: ContentType::InputText,
            text: String::new(),
        }
    }
}

impl TextContent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
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
pub struct ImageContent {
    #[serde(rename = "type")]
    pub type_field: ContentType, // always InputImage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    pub detail: ImageDetail,
}

impl Default for ImageContent {
    fn default() -> Self {
        Self {
            type_field: ContentType::InputImage,
            image_url: None,
            file_id: None,
            detail: ImageDetail::Auto,
        }
    }
}

impl ImageContent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn detail(mut self, value: impl AsRef<str>) -> Result<Self, ConversionError> {
        self.detail = ImageDetail::from_str(value.as_ref())?;
        Ok(self)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FileContent {
    #[serde(rename = "type")]
    pub type_field: ContentType, // always InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

impl Default for FileContent {
    fn default() -> Self {
        Self {
            type_field: ContentType::InputFile,
            file_id: None,
            file_data: None,
            filename: None,
        }
    }
}

impl FileContent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn file_data(mut self, value: impl Into<String>) -> Self {
        self.file_data = Some(value.into());
        self
    }

    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
        self
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Content {
    Text(TextContent),
    Image(ImageContent),
    File(FileContent),
}

impl From<TextContent> for Content {
    fn from(text_content: TextContent) -> Self {
        Self::Text(text_content)
    }
}

impl From<ImageContent> for Content {
    fn from(image_content: ImageContent) -> Self {
        Self::Image(image_content)
    }
}

impl From<FileContent> for Content {
    fn from(file_content: FileContent) -> Self {
        Self::File(file_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // let's compare the json output of the default values
    #[test]
    fn test_default_values() {
        let text_content = TextContent::default();
        let image_content = ImageContent::default();
        let file_content = FileContent::default();

        let text_content_json = serde_json::to_value(&text_content).unwrap();
        let image_content_json = serde_json::to_value(&image_content).unwrap();
        let file_content_json = serde_json::to_value(&file_content).unwrap();

        assert_eq!(text_content_json, json!({"type": "input_text", "text": ""}));
        assert_eq!(
            image_content_json,
            json!({"type": "input_image", "detail": "auto"})
        );
        assert_eq!(file_content_json, json!({"type": "input_file"}));
    }

    #[test]
    fn test_text_content() {
        let text = "Hello, world!";
        let text_content = TextContent::new().text(text);
        let text_content_json = serde_json::to_value(&text_content).unwrap();
        assert_eq!(
            text_content_json,
            json!({"type": "input_text", "text": text})
        );
    }

    #[test]
    fn test_image_content() {
        let image_url = "https://example.com/image.png";
        let file_id = "1234567890";
        let detail = "auto";

        let image_content = ImageContent::new()
            .image_url(image_url)
            .file_id(file_id)
            .detail(detail)
            .unwrap();

        let image_content_json = serde_json::to_value(&image_content).unwrap();
        assert_eq!(
            image_content_json,
            json!({"type": "input_image", "image_url": image_url, "file_id": file_id, "detail": detail})
        );
    }

    #[test]
    fn test_file_content() {
        let file_id = "1234567890";
        let file_data = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+ip1sAAAAASUVORK5CYII=";
        let filename = "image.png";

        let file_content = FileContent::new()
            .file_id(file_id)
            .file_data(file_data)
            .filename(filename);

        let file_content_json = serde_json::to_value(&file_content).unwrap();
        assert_eq!(
            file_content_json,
            json!({"type": "input_file", "file_id": file_id, "file_data": file_data, "filename": filename})
        );
    }

    #[test]
    fn test_role_from_str() {
        assert_eq!(Role::from_str("user").unwrap(), Role::User);
        assert_eq!(Role::from_str("assistant").unwrap(), Role::Assistant);
        assert_eq!(Role::from_str("system").unwrap(), Role::System);
        assert_eq!(Role::from_str("developer").unwrap(), Role::Developer);
    }

    #[test]
    fn test_image_detail_from_str() {
        assert_eq!(ImageDetail::from_str("high").unwrap(), ImageDetail::High);
        assert_eq!(ImageDetail::from_str("low").unwrap(), ImageDetail::Low);
        assert_eq!(ImageDetail::from_str("auto").unwrap(), ImageDetail::Auto);
    }

    #[test]
    fn test_from_specific_content_to_content() {
        let text = "Hello, world!";
        let image_url = "https://example.com/image.png";
        let file_id = "1234567890";

        let text_content_builder = TextContent::new().text(text);
        let text_content: Content = text_content_builder.into();

        let image_content_builder = ImageContent::new().image_url(image_url);
        let image_content: Content = image_content_builder.into();

        let file_content_builder = FileContent::new().file_id(file_id);
        let file_content: Content = file_content_builder.into();

        assert_eq!(text_content, Content::Text(TextContent::new().text(text)));
        assert_eq!(
            image_content,
            Content::Image(ImageContent::new().image_url(image_url))
        );
        assert_eq!(
            file_content,
            Content::File(FileContent::new().file_id(file_id))
        );
    }
}
