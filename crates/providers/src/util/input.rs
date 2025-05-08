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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "high" => Ok(ImageDetail::High),
            "low" => Ok(ImageDetail::Low),
            "auto" => Ok(ImageDetail::Auto),
            _ => Err(format!("Invalid string slice: {}", s)),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImageContent<'a> {
    #[serde(rename = "type")]
    pub type_field: ContentType, // always InputImage
    pub image_url: Option<&'a str>,
    pub file_id: Option<&'a str>,
    pub detail: ImageDetail,
}

impl<'a> Default for ImageContent<'a> {
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

    pub fn detail(mut self, value: &'a str) -> Self {
        self.detail = ImageDetail::from_str(value).unwrap();
        self
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FileContent<'a> {
    #[serde(rename = "type")]
    pub type_field: ContentType, // always InputFile,
    pub file_id: Option<&'a str>,
    pub file_data: Option<&'a str>,
    pub filename: Option<&'a str>,
}

impl<'a> Default for FileContent<'a> {
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct SingleContent<'a> {
    pub role: Role,
    pub content: &'a str,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<&'a str>,
}

impl<'a> SingleContent<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            role: Role::default(),
            content,
            type_field: None,
        }
    }

    pub fn role(mut self, role: &'a str) -> Self {
        self.role = Role::from_str(role).unwrap();
        self
    }

    pub fn insert_type(mut self) -> Self {
        self.type_field = Some("message");
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct MultiContent<'a> {
    pub role: Role,
    pub content: Vec<Content<'a>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<&'a str>,
}

impl<'a> Default for MultiContent<'a> {
    fn default() -> Self {
        Self {
            role: Role::default(),
            content: vec![],
            type_field: None,
        }
    }
}

impl<'a> MultiContent<'a> {
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

    pub fn insert_type(mut self) -> Self {
        self.type_field = Some("message");
        self
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Input<'a> {
    Text(&'a str),
    Object(Vec<ObjectInput<'a>>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
#[serde(untagged)]
pub enum ObjectInput<'a> {
    Single(SingleContent<'a>),
    Multi(MultiContent<'a>),
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

impl<'a> From<Vec<ObjectInput<'a>>> for Input<'a> {
    fn from(value: Vec<ObjectInput<'a>>) -> Self {
        Self::Object(value)
    }
}

impl<'a> TryFrom<Input<'a>> for Vec<ObjectInput<'a>> {
    type Error = String;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        match input {
            Input::Object(inner) => Ok(inner),
            _ => Err("Unable to convert Input into Vec<ObjectInput<'a>>".to_string()),
        }
    }
}

impl<'a> From<SingleContent<'a>> for ObjectInput<'a> {
    fn from(value: SingleContent<'a>) -> Self {
        ObjectInput::Single(value)
    }
}

impl<'a> TryFrom<ObjectInput<'a>> for SingleContent<'a> {
    type Error = String;

    fn try_from(object_input: ObjectInput<'a>) -> Result<Self, Self::Error> {
        match object_input {
            ObjectInput::Single(inner) => Ok(inner),
            _ => Err("Unable to convert ObjectInput into SingleContent".to_string()),
        }
    }
}

impl<'a> From<MultiContent<'a>> for ObjectInput<'a> {
    fn from(value: MultiContent<'a>) -> Self {
        ObjectInput::Multi(value)
    }
}

impl<'a> TryFrom<ObjectInput<'a>> for MultiContent<'a> {
    type Error = String;

    fn try_from(object_input: ObjectInput<'a>) -> Result<Self, Self::Error> {
        match object_input {
            ObjectInput::Multi(inner) => Ok(inner),
            _ => Err("Unable to convert ObjectInput into MultiContent".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

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
        let result = SingleContent::new(content).role("assistant");

        let expected = SingleContent {
            role: Role::Assistant,
            content,
            type_field: None,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn it_builds_multi_content_input() {
        let payload = vec![
            TextContent::new("test content").into(),
            ImageContent::new().image_url("http://image.url").into(),
            FileContent::new().file_id("file_id").into(),
            FileContent::new()
                .filename("filename")
                .file_data("file_data")
                .into(),
        ];

        let expected = MultiContent {
            role: Role::User,
            content: payload.clone(),
            type_field: None,
        };
        let result = MultiContent::new().append_content(payload);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_appends_content_to_multi_content_input() {
        let payload = vec![
            TextContent::new("test content").into(),
            ImageContent::new().image_url("http://image.url").into(),
            FileContent::new().file_id("file_id").into(),
        ];

        let multi_content_input = MultiContent::new().append_content(payload);

        let content_to_add: Content = FileContent::new()
            .filename("filename")
            .file_data("file_data")
            .into();

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
        let role = Role::System;
        let content = "test content";
        let result = Input::Object(vec![SingleContent {
            role,
            content,
            type_field: None,
        }
        .into()]);
        let expected: Input = Input::Object(vec![ObjectInput::Single(SingleContent {
            role,
            content,
            type_field: None,
        })]);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_builds_multi_content_input_wrapper() {
        let payload = vec![
            TextContent::new("test content").into(),
            ImageContent::new().image_url("http://image.url").into(),
        ];

        let role = Role::Assistant;

        let result: Input = vec![MultiContent::new()
            .role("assistant")
            .append_content(payload.clone())
            .into()]
        .into();

        let expected = Input::Object(vec![ObjectInput::Multi(MultiContent {
            content: payload,
            role,
            type_field: None,
        })]);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_appends_content_to_multi_content_input_wrapper() {
        let payload = vec![TextContent::new("test_content").into()];
        let role = Role::System;

        let input = MultiContent::new()
            .append_content(payload.clone())
            .role("system");

        let content_to_add = vec![ImageContent::new().image_url("http://image.url").into()];
        let input = input.append_content(content_to_add.clone());

        let multi_content_input: MultiContent = input.clone().try_into().unwrap();

        assert_eq!(multi_content_input.content.len(), 2);
        assert_eq!(multi_content_input.content[1], content_to_add[0]);

        let mut combined = payload.clone();
        combined.extend(content_to_add.clone());
        let expected_input = MultiContent {
            role,
            content: combined,
            type_field: None,
        };

        assert_eq!(input, expected_input);
    }

    #[test]
    fn it_converts_input_to_json() {
        let image_url = "https://image.url";
        let filename = "test_filename";
        let base64_data = "bGlnaHQgd29yaw==";
        let input: Input = vec![
            SingleContent::new("test content").into(),
            MultiContent::new()
                .append_content(vec![
                    ImageContent::new().image_url(image_url).into(),
                    FileContent::new()
                        .filename(filename)
                        .file_data(base64_data)
                        .into(),
                ])
                .into(),
        ]
        .into();

        let result = serde_json::to_value(input).unwrap();

        let expected = json!([{
            "role": "user",
            "content": "test content"
        }, {
            "role": "user",
            "content": [
                {
                    "type": "input_image",
                    "image_url": image_url
                },
                {
                    "type": "input_file",
                    "filename": filename,
                }
            ]
        }]);

        assert_eq!(result, expected);
    }
}
