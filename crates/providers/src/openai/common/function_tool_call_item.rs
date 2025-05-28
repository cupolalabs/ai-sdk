use serde::{Deserialize, Serialize};

use crate::openai::common::status::Status;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionToolCallItem {
    pub arguments: String,
    pub call_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl FunctionToolCallItem {
    pub fn new(
        arguments: impl Into<String>,
        call_id: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            arguments: arguments.into(),
            call_id: call_id.into(),
            name: name.into(),
            id: None,
            status: None,
        }
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: Status) -> Self {
        self.status = Some(value);
        self
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn it_builds_function_tool_call_item() {
        let item = FunctionToolCallItem::new("test-arguments", "test-id", "test-name")
            .id("test-id")
            .status(Status::Completed);

        let expected = json!({
            "arguments": "test-arguments",
            "call_id": "test-id",
            "name": "test-name",
            "id": "test-id",
            "status": "completed"
        });

        assert_eq!(serde_json::to_value(item).unwrap(), expected);
    }
}
