use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ResponseError {
    code: String,
    message: String,
}

impl ResponseError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_serializes_to_json() {
        let error = ResponseError::new("test_code", "test_message");
        let json = serde_json::to_value(&error).unwrap();
        assert_eq!(
            json,
            json!({
                "code": "test_code",
                "message": "test_message"
            })
        );
    }
}
