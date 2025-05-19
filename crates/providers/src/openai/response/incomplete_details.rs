use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IncompleteDetails {
    reason: String,
}

impl IncompleteDetails {
    pub fn new(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_serializes_to_json() {
        let incomplete_details = IncompleteDetails::new("test_reason");
        let json = serde_json::to_value(&incomplete_details).unwrap();
        assert_eq!(json, json!({ "reason": "test_reason" }));
    }
}
