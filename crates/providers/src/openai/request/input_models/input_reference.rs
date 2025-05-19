use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputReference {
    pub id: String,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<String>,
}

impl InputReference {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            type_field: None,
        }
    }

    pub fn insert_type(mut self) -> Self {
        self.type_field = Some("item_reference".to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_values() {
        let input_reference = InputReference::new("123").insert_type();
        let json_value = serde_json::to_value(&input_reference).unwrap();
        assert_eq!(
            json_value,
            serde_json::json!({
                "id": "123",
                "type": "item_reference"
            })
        );
    }
}
