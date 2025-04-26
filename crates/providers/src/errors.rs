use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum ContentError {
    InvalidRole(String),
    InvalidContentType(String),
    UnableToAppend,
}

impl Display for ContentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentError::InvalidRole(msg) => write!(f, "Invalid role: {}", msg),
            ContentError::InvalidContentType(msg) => write!(f, "Invalid content type: {}", msg),
            ContentError::UnableToAppend => write!(f, "Unable to append content to the input"),
        }
    }
}
