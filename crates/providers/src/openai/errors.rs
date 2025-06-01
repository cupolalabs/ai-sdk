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
    InvalidButtonForClickAction(String),
    ConversionError(ConversionError),
    InvalidModelId(String),
    // ImageGenerationTool
    InvalidPartialImage(usize),
    // FileSearchTool
    EmptyVectorStoreIds,
    // FunctionTool
    EmptyName,
    EmptyParameters,
    StrictFunctionTool,
    // ComputerUseTool
    InvalidDisplayHeight,
    InvalidDisplayWidth,
    EmptyEnvironment,
    // MCPTool
    EmptyServerLabel,
    EmptyServerUrl,
    // CodeInterpreterTool
    EmptyFileIds,
    EmptyTypeField,
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
            InputError::InvalidButtonForClickAction(msg) => {
                write!(f, "Invalid button value for click action: {}", msg)
            }
            InputError::ConversionError(err) => write!(f, "Conversion error: {}", err),
            InputError::InvalidModelId(err) => write!(f, "Invalid model id: {}", err),
            InputError::InvalidPartialImage(value) => {
                write!(
                    f,
                    "Invalid partial image value: {}. Value must be between 0 and 3",
                    value
                )
            }
            InputError::EmptyVectorStoreIds => write!(f, "Vector store ids cannot be empty"),
            InputError::EmptyName => write!(f, "Name cannot be empty"),
            InputError::EmptyParameters => write!(f, "Parameters cannot be empty"),
            InputError::StrictFunctionTool => write!(f, "Strict function tool cannot be used"),
            InputError::InvalidDisplayHeight => write!(f, "Display height must be greater than 0"),
            InputError::InvalidDisplayWidth => write!(f, "Display width must be greater than 0"),
            InputError::EmptyEnvironment => write!(f, "Environment cannot be empty"),
            InputError::EmptyServerLabel => write!(f, "Server label cannot be empty"),
            InputError::EmptyServerUrl => write!(f, "Server url cannot be empty"),
            InputError::EmptyFileIds => write!(f, "File ids cannot be empty"),
            InputError::EmptyTypeField => write!(f, "Type field cannot be empty"),
        }
    }
}

impl Error for InputError {}

impl From<ConversionError> for InputError {
    fn from(error: ConversionError) -> Self {
        InputError::ConversionError(error)
    }
}

#[derive(Debug)]
pub enum BuilderError {
    Conversion(ConversionError),
    Input(InputError),
}

impl From<ConversionError> for BuilderError {
    fn from(error: ConversionError) -> Self {
        BuilderError::Conversion(error)
    }
}

impl From<InputError> for BuilderError {
    fn from(error: InputError) -> Self {
        BuilderError::Input(error)
    }
}
