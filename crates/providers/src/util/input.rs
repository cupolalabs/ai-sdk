use crate::errors::ContentError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
    type Err = ContentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user" => Ok(Role::User),
            "assistant" => Ok(Role::Assistant),
            "system" => Ok(Role::System),
            "developer" => Ok(Role::Developer),
            _ => Err(ContentError::InvalidRole(s.to_string())),
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
    type Err = ContentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "input_text" => Ok(ContentType::InputText),
            "input_image" => Ok(ContentType::InputImage),
            "input_file" => Ok(ContentType::InputFile),
            _ => Err(ContentError::InvalidContentType(s.to_string())),
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
    pub fn build(text: &'a str) -> Self {
        Self {
            type_field: ContentType::InputText,
            text,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImageContent<'a> {
    #[serde(rename = "type")]
    pub type_field: ContentType, // always InputImage
    pub image_url: &'a str,
}

impl<'a> ImageContent<'a> {
    pub fn build(image_url: &'a str) -> Self {
        Self {
            type_field: ContentType::InputImage,
            image_url,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FileWithIdContent<'a> {
    #[serde(rename = "type")]
    pub type_field: ContentType, // always InputFile,
    pub file_id: &'a str,
}

impl<'a> FileWithIdContent<'a> {
    pub fn build(file_id: &'a str) -> Self {
        Self {
            type_field: ContentType::InputFile,
            file_id,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FileWithBase64Content<'a> {
    #[serde(rename = "type")]
    pub type_field: ContentType, // always InputFile
    pub filename: &'a str,
    pub file_data: &'a str, // base64 goes here
}

impl<'a> FileWithBase64Content<'a> {
    pub fn build(filename: &'a str, file_data: &'a str) -> Self {
        Self {
            type_field: ContentType::InputFile,
            filename,
            file_data,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
#[serde(untagged)]
pub enum Content<'a> {
    Text(TextContent<'a>),
    Image(ImageContent<'a>),
    FileWithId(FileWithIdContent<'a>),
    FileWithBase64(FileWithBase64Content<'a>),
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

impl<'a> From<FileWithIdContent<'a>> for Content<'a> {
    fn from(file_with_id_content: FileWithIdContent<'a>) -> Self {
        Self::FileWithId(file_with_id_content)
    }
}

impl<'a> From<FileWithBase64Content<'a>> for Content<'a> {
    fn from(file_with_base64_content: FileWithBase64Content<'a>) -> Self {
        Self::FileWithBase64(file_with_base64_content)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct SingleContentInput<'a> {
    pub role: Role,
    pub content: &'a str,
}

impl<'a> SingleContentInput<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            role: Role::default(),
            content,
        }
    }

    pub fn role(mut self, role: &'a str) -> Self {
        self.role = Role::from_str(role).unwrap();
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct MultiContentInput<'a> {
    pub role: Role,
    pub content: Vec<Content<'a>>,
}

impl<'a> Default for MultiContentInput<'a> {
    fn default() -> Self {
        Self {
            role: Role::default(),
            content: vec![],
        }
    }
}

impl<'a> MultiContentInput<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn role(mut self, role: &'a str) -> Self {
        self.role = Role::from_str(role).unwrap();
        self
    }

    pub fn append_content(mut self, payload: Vec<Content<'a>>) -> Self {
        if self.content.len() > 0 {
            self.content.extend(payload);
        } else {
            self.content = payload;
        }
        self
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Input<'a> {
    Text(&'a str),
    SingleContent(SingleContentInput<'a>),
    MultiContent(MultiContentInput<'a>),
}

impl<'a> Input<'a> {
    pub fn empty() -> Self {
        Self::Text("")
    }
}

// NOTE: this is for bypassing the Request's default value
impl<'a> Default for Input<'a> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<'a> From<&'a str> for Input<'a> {
    fn from(value: &'a str) -> Self {
        Input::Text(value)
    }
}

impl<'a> TryFrom<Input<'a>> for &'a str {
    type Error = String;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        match input {
            Input::Text(inner) => Ok(inner),
            _ => Err("Unable to convert Input into string slice".to_string()),
        }
    }
}

// NOTE: we left here, check out the try_from implementations when you come back
impl<'a> From<SingleContentInput<'a>> for Input<'a> {
    fn from(value: SingleContentInput<'a>) -> Self {
        Self::SingleContent(value)
    }
}

impl<'a> TryFrom<Input<'a>> for SingleContentInput<'a> {
    type Error = String;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        match input {
            Input::SingleContent(inner) => Ok(inner),
            _ => Err("Unable to convert Input into SingleContentInput slice".to_string()),
        }
    }
}

impl<'a> From<MultiContentInput<'a>> for Input<'a> {
    fn from(value: MultiContentInput<'a>) -> Self {
        Self::MultiContent(value)
    }
}

impl<'a> TryFrom<Input<'a>> for MultiContentInput<'a> {
    type Error = String;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        match input {
            Input::MultiContent(inner) => Ok(inner),
            _ => Err("Unable to convert Input into MultiContentInput slice".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_a_string_slice_to_a_role() {
        let roles = ["user", "assistant", "system", "developer"];
        let expected = [Role::User, Role::Assistant, Role::System, Role::Developer];

        for (index, role) in roles.iter().enumerate() {
            assert_eq!(Role::from_str(role).unwrap(), expected[index]);
        }
    }

    #[test]
    fn it_returns_error_when_wrong_role_is_given() {
        let wrong_role = "wrong_role";

        assert_eq!(
            Role::from_str(wrong_role),
            Err(ContentError::InvalidRole(wrong_role.to_string()))
        );
    }

    #[test]
    fn it_converts_a_string_slice_to_a_content_type() {
        let content_types = ["input_text", "input_image", "input_file"];
        let expected = [
            ContentType::InputText,
            ContentType::InputImage,
            ContentType::InputFile,
        ];

        for (index, content_type) in content_types.iter().enumerate() {
            assert_eq!(
                ContentType::from_str(content_type).unwrap(),
                expected[index]
            );
        }
    }

    #[test]
    fn it_returns_error_when_wrong_content_type_is_given() {
        let wrong_content_type = "wrong_content_type";

        assert_eq!(
            ContentType::from_str(wrong_content_type),
            Err(ContentError::InvalidContentType(
                wrong_content_type.to_string()
            ))
        );
    }

    #[test]
    fn it_builds_single_content_input() {
        let content = "test content";
        let result = SingleContentInput::new(content).role("assistant");

        let expected = SingleContentInput {
            role: Role::Assistant,
            content,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn it_builds_multi_content_input() {
        let payload = vec![
            TextContent::build("test content").into(),
            ImageContent::build("http://image.url").into(),
            FileWithIdContent::build("file_id").into(),
            FileWithBase64Content::build("filename", "file_data").into(),
        ];

        let expected = MultiContentInput {
            role: Role::User,
            content: payload.clone(),
        };
        let result = MultiContentInput::new().append_content(payload);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_appends_content_to_multi_content_input() {
        let payload = vec![
            TextContent::build("test content").into(),
            ImageContent::build("http://image.url").into(),
            FileWithIdContent::build("file_id").into(),
        ];

        let multi_content_input = MultiContentInput::new().append_content(payload);

        let content_to_add: Content = FileWithBase64Content::build("filename", "file_data").into();

        let multi_content_input = multi_content_input.append_content(vec![content_to_add.clone()]);

        let last_content = multi_content_input.content.last().unwrap();

        assert_eq!(*last_content, content_to_add);
    }

    #[test]
    fn it_builds_text_input() {
        let content = "test content";
        let expected = Input::Text(content);

        let result: Input = content.into();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_builds_single_content_input_wrapper() {
        let content = "test content";
        let role = Role::System;
        let expected = Input::SingleContent(SingleContentInput { content, role });

        let result: Input = SingleContentInput::new(content).role("system").into();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_builds_multi_content_input_wrapper() {
        let payload = vec![
            TextContent::build("test content").into(),
            ImageContent::build("http://image.url").into(),
        ];

        let role = Role::Assistant;

        let result: Input = MultiContentInput::new()
            .role("assistant")
            .append_content(payload.clone())
            .into();

        let expected = Input::MultiContent(MultiContentInput {
            role,
            content: payload,
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn it_appends_content_to_multi_content_input_wrapper() {
        let payload = vec![TextContent::build("test_content").into()];
        let role = Role::System;

        let input = MultiContentInput::new()
            .append_content(payload.clone())
            .role("system");

        let content_to_add = vec![ImageContent::build("http://image.url").into()];
        let input = input.append_content(content_to_add.clone());

        let multi_content_input: MultiContentInput = input.clone().try_into().unwrap();

        assert_eq!(multi_content_input.content.len(), 2);
        assert_eq!(multi_content_input.content[1], content_to_add[0]);

        let mut combined = payload.clone();
        combined.extend(content_to_add.clone());
        let expected_input = MultiContentInput {
            role,
            content: combined,
        };

        assert_eq!(input, expected_input);
    }
}
