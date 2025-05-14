use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IncompleteDetails<'a> {
    reason: &'a str,
}

impl<'a> IncompleteDetails<'a> {
    pub fn new(reason: &'a str) -> Self {
        Self { reason }
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
