use serde::{Deserialize, Serialize};

use crate::openai::errors::ConversionError;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonSchemaFormat {
    name: String,
    schema: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict: Option<bool>,
}

impl JsonSchemaFormat {
    pub fn new(name: impl Into<String>, schema: serde_json::Value) -> Self {
        Self {
            name: name.into(),
            schema,
            description: None,
            strict: Some(false),
        }
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn strict(mut self) -> Self {
        self.strict = Some(true);
        self
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResponseFormat {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "json_schema")]
    JsonSchema(JsonSchemaFormat),
    #[serde(rename = "json_object")]
    JsonObject,
}

impl From<JsonSchemaFormat> for ResponseFormat {
    fn from(format: JsonSchemaFormat) -> Self {
        Self::JsonSchema(format)
    }
}

impl TryFrom<ResponseFormat> for JsonSchemaFormat {
    type Error = ConversionError;

    fn try_from(format: ResponseFormat) -> Result<Self, Self::Error> {
        match format {
            ResponseFormat::JsonSchema(inner) => Ok(inner),
            _ => Err(ConversionError::TryFrom("ResponseFormat".to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Text {
    // NOTE: this one is optiona in the docs however outer field which is Text
    // is also optional in the input field, so that's why we don't need to
    // make format Option
    format: ResponseFormat,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            format: ResponseFormat::Text,
        }
    }
}

impl Text {
    pub fn new(value: ResponseFormat) -> Self {
        Self { format: value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_builds_text() {
        let text = Text::new(ResponseFormat::Text);
        let json_schema = Text::new(ResponseFormat::JsonSchema(
            JsonSchemaFormat::new(
                "json-schema-format-name",
                json!({
                    "page": "homepage",
                    "theme": "dark"
                }),
            )
            .description("this-is-a-description")
            .strict(),
        ));
        let json_object = Text::new(ResponseFormat::JsonObject);

        let text_expected = json!({
            "format": { "type": "text" }
        });
        let json_schema_expected = json!({
            "format": {
                "type": "json_schema",
                "name": "json-schema-format-name",
                "schema": {
                    "page": "homepage",
                    "theme": "dark",
                },
                "description": "this-is-a-description",
                "strict": true
            }
        });
        let json_object_expected = json!({
            "format": {
                "type": "json_object"
            }
        });

        assert_eq!(serde_json::to_value(text).unwrap(), text_expected);
        assert_eq!(
            serde_json::to_value(json_schema).unwrap(),
            json_schema_expected
        );
        assert_eq!(
            serde_json::to_value(json_object).unwrap(),
            json_object_expected
        );
    }
}
