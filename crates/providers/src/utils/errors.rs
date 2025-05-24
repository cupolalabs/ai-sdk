use std::fmt;

#[derive(Debug)]
pub enum ProviderError {
    NetworkError(String),
    ApiError { status: u16, message: String },
    DeserializationError(String),
    ValidationError(String),
    CapabilityError(String),
    NotSupported(String),
    InternalError(String),
    Other(String),
}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProviderError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ProviderError::ApiError { status, message } => {
                write!(f, "API error (status {}): {}", status, message)
            }
            ProviderError::DeserializationError(msg) => {
                write!(f, "Deserialization error: {}", msg)
            }
            ProviderError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ProviderError::CapabilityError(msg) => write!(f, "Capability error: {}", msg),
            ProviderError::NotSupported(msg) => write!(f, "Operation not supported: {}", msg),
            ProviderError::InternalError(msg) => write!(f, "Internal provider error: {}", msg),
            ProviderError::Other(msg) => write!(f, "An unexpected error occurred: {}", msg),
        }
    }
}

impl std::error::Error for ProviderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ProviderError::NetworkError(_) => None,
            ProviderError::ApiError { .. } => None,
            ProviderError::DeserializationError(_) => None,
            ProviderError::ValidationError(_) => None,
            ProviderError::CapabilityError(_) => None,
            ProviderError::NotSupported(_) => None,
            ProviderError::InternalError(_) => None,
            ProviderError::Other(_) => None,
        }
    }
}
