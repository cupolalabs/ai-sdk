use crate::errors::ContentError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

//------------------------------------------------------------------------------
// Basic Enums
//------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
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

//------------------------------------------------------------------------------
// Content Structs
//------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TextContent<'a> {
    #[serde(rename = "type")]
    pub type_field: ContentType, // always InputText
    pub text: &'a str,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImageContent<'a> {
    #[serde(rename = "type")]
    pub type_field: ContentType, // always InputImage
    pub image_url: &'a str,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FileWithFileIdContent<'a> {
    #[serde(rename = "type")]
    pub type_field: ContentType, // always InputFile,
    pub file_id: &'a str,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FileWithBase64Content<'a> {
    #[serde(rename = "type")]
    pub type_field: ContentType, // always InputFile
    pub filename: &'a str,
    pub file_data: &'a str, // base64 goes here
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
#[serde(untagged)]
pub enum Content<'a> {
    Text(TextContent<'a>),
    Image(ImageContent<'a>),
    FileWithFileId(FileWithFileIdContent<'a>),
    FileWithBase64(FileWithBase64Content<'a>),
}

//------------------------------------------------------------------------------
// Input Structs and Implementation
//------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct SingleContentInput<'a> {
    pub role: Role,
    pub content: &'a str,
}

impl<'a> SingleContentInput<'a> {
    fn build(role: Role, content: &'a str) -> Self {
        Self { role, content }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct MultiContentInput<'a> {
    pub role: Role,
    pub content: Vec<Content<'a>>,
}

pub enum ContentPayload<'a> {
    Text(&'a str),
    Image(&'a str),
    FileWithFileId(&'a str),
    FileWithBase64(&'a str, &'a str),
}

impl<'a> MultiContentInput<'a> {
    fn build(role: Role, payload: Vec<ContentPayload<'a>>) -> Self {
        let mut content_vec: Vec<Content<'a>> = Vec::new();

        payload.iter().for_each(|p| match p {
            ContentPayload::Text(text) => {
                content_vec.push(Content::Text(TextContent {
                    type_field: ContentType::InputText,
                    text,
                }));
            }
            ContentPayload::Image(image_url) => {
                content_vec.push(Content::Image(ImageContent {
                    type_field: ContentType::InputImage,
                    image_url,
                }));
            }
            ContentPayload::FileWithFileId(file_id) => {
                content_vec.push(Content::FileWithFileId(FileWithFileIdContent {
                    type_field: ContentType::InputFile,
                    file_id,
                }));
            }
            ContentPayload::FileWithBase64(filename, file_data) => {
                content_vec.push(Content::FileWithBase64(FileWithBase64Content {
                    type_field: ContentType::InputFile,
                    filename,
                    file_data,
                }));
            }
        });

        Self {
            role,
            content: content_vec,
        }
    }

    fn append_content(&mut self, content_to_add: Content<'a>) {
        self.content.push(content_to_add);
    }
}

//------------------------------------------------------------------------------
// Main Input Enum and Implementation
//------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Input<'a> {
    Text(&'a str),
    SingleContent(SingleContentInput<'a>),
    MultiContent(MultiContentInput<'a>),
}

impl<'a> Input<'a> {
    pub fn build_text_input(content: &'a str) -> Self {
        Self::Text(content)
    }

    pub fn build_single_content_input(role: Role, content: &'a str) -> Self {
        Self::SingleContent(SingleContentInput::build(role, content))
    }

    pub fn build_multi_content_input(role: Role, payload: Vec<ContentPayload<'a>>) -> Self {
        Self::MultiContent(MultiContentInput::build(role, payload))
    }

    // NOTE: this only works with the MultiContentInput type
    pub fn append_content(&mut self, content_to_add: Content<'a>) -> Result<(), ContentError> {
        match self {
            Input::MultiContent(multi_content_input) => {
                multi_content_input.append_content(content_to_add);

                Ok(())
            }
            _ => Err(ContentError::UnableToAppend),
        }
    }
}

//------------------------------------------------------------------------------
// Tests
//------------------------------------------------------------------------------

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
        let expected = SingleContentInput {
            role: Role::Assistant,
            content,
        };

        let result = SingleContentInput::build(Role::Assistant, content);
        assert_eq!(result, expected);
    }

    #[test]
    fn it_builds_multi_content_input() {
        let content_payloads = vec![
            ContentPayload::Text("test content"),
            ContentPayload::Image("http://image.url"),
            ContentPayload::FileWithFileId("file_id"),
            ContentPayload::FileWithBase64("filename", "file_data"),
        ];

        let expected = MultiContentInput {
            role: Role::User,
            content: vec![
                Content::Text(TextContent {
                    type_field: ContentType::InputText,
                    text: "test content",
                }),
                Content::Image(ImageContent {
                    type_field: ContentType::InputImage,
                    image_url: "http://image.url",
                }),
                Content::FileWithFileId(FileWithFileIdContent {
                    type_field: ContentType::InputFile,
                    file_id: "file_id",
                }),
                Content::FileWithBase64(FileWithBase64Content {
                    type_field: ContentType::InputFile,
                    filename: "filename",
                    file_data: "file_data",
                }),
            ],
        };
        let result = MultiContentInput::build(Role::User, content_payloads);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_appends_content_to_multi_content_input() {
        let mut multi_content_input = MultiContentInput {
            role: Role::User,
            content: vec![
                Content::Text(TextContent {
                    type_field: ContentType::InputText,
                    text: "test content",
                }),
                Content::Image(ImageContent {
                    type_field: ContentType::InputImage,
                    image_url: "http://image.url",
                }),
                Content::FileWithFileId(FileWithFileIdContent {
                    type_field: ContentType::InputFile,
                    file_id: "file_id",
                }),
            ],
        };

        let content_to_add = Content::FileWithBase64(FileWithBase64Content {
            type_field: ContentType::InputFile,
            filename: "filename",
            file_data: "file_data",
        });

        multi_content_input.append_content(content_to_add.clone());

        let last_content = multi_content_input.content.last().unwrap();

        assert_eq!(*last_content, content_to_add);
    }

    #[test]
    fn it_builds_text_input() {
        let content = "test content";
        let expected = Input::Text(content);

        let result = Input::build_text_input(content);
        assert_eq!(result, expected);
    }

    #[test]
    fn it_builds_single_content_input_wrapper() {
        let content = "test content";
        let role = Role::User;
        let expected = Input::SingleContent(SingleContentInput { role, content });

        let result = Input::build_single_content_input(role, content);
        assert_eq!(result, expected);
    }

    #[test]
    fn it_builds_multi_content_input_wrapper() {
        let content_payloads = vec![
            ContentPayload::Text("test content"),
            ContentPayload::Image("http://image.url"),
        ];
        let role = Role::Assistant;

        let expected_content = vec![
            Content::Text(TextContent {
                type_field: ContentType::InputText,
                text: "test content",
            }),
            Content::Image(ImageContent {
                type_field: ContentType::InputImage,
                image_url: "http://image.url",
            }),
        ];

        let expected = Input::MultiContent(MultiContentInput {
            role,
            content: expected_content,
        });

        let result = Input::build_multi_content_input(role, content_payloads);
        assert_eq!(result, expected);
    }

    #[test]
    fn it_appends_content_to_multi_content_input_wrapper() {
        let initial_content = vec![ContentPayload::Text("initial text")];
        let role = Role::System;

        let mut input = Input::build_multi_content_input(role, initial_content);

        let content_to_add = Content::Image(ImageContent {
            type_field: ContentType::InputImage,
            image_url: "http://new.image.url",
        });

        let append_result = input.append_content(content_to_add.clone());
        assert!(append_result.is_ok());

        if let Input::MultiContent(multi_content) = input {
            assert_eq!(multi_content.content.len(), 2);
            assert_eq!(multi_content.content[1], content_to_add);
        } else {
            panic!("Expected MultiContent variant");
        }
    }

    #[test]
    fn it_returns_error_when_appending_to_non_multi_content() {
        let mut text_input = Input::build_text_input("simple text");

        let content_to_add = Content::Text(TextContent {
            type_field: ContentType::InputText,
            text: "additional text",
        });

        let result = text_input.append_content(content_to_add);
        assert_eq!(result, Err(ContentError::UnableToAppend));
    }
}
