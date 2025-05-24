use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum ConversionError {
    FromStr(String),
    TryFrom(String),
}

impl Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversionError::FromStr(msg) => write!(f, "Failed to convert from string: {}", msg),
            ConversionError::TryFrom(msg) => {
                write!(f, "Failed to convert from {} to target type", msg)
            }
        }
    }
}

impl Error for ConversionError {}

#[derive(Debug, PartialEq)]
pub enum InputError {
    InvalidToolType(String),
    InvalidRole(String),
    InvalidButton(String),
    ConversionError(ConversionError),
    InvalidModelId(String),
}

impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputError::InvalidToolType(msg) => write!(f, "Invalid tool type: {}", msg),
            InputError::InvalidRole(msg) => {
                write!(
                    f,
                    "The role {} is not compatible with InputMessageItem",
                    msg
                )
            }
            InputError::InvalidButton(msg) => {
                write!(f, "Invalid button value: {}", msg)
            }
            InputError::ConversionError(err) => write!(f, "Conversion error: {}", err),
            InputError::InvalidModelId(err) => write!(f, "Invalid model id: {}", err),
        }
    }
}

impl Error for InputError {}

impl From<ConversionError> for InputError {
    fn from(error: ConversionError) -> Self {
        InputError::ConversionError(error)
    }
}
