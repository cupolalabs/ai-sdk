use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ResponseError<'a> {
    code: &'a str,
    message: &'a str,
}

impl<'a> ResponseError<'a> {
    pub fn new(code: &'a str, message: &'a str) -> Self {
        Self { code, message }
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
