use serde::{Deserialize, Serialize};

use crate::openai::errors::ConversionError;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ResponseFormatType {
    Text,
    JsonSchema,
    JsonObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TextFormat {
    #[serde(rename = "type")]
    type_field: ResponseFormatType, // always text
}

impl TextFormat {
    pub fn new() -> Self {
        Self {
            type_field: ResponseFormatType::Text,
        }
    }
}

impl Default for TextFormat {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonSchemaFormat {
    #[serde(rename = "type")]
    type_field: ResponseFormatType, // always json_schema
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
            type_field: ResponseFormatType::JsonSchema,
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
pub struct JsonObjectFormat {
    #[serde(rename = "type")]
    type_field: ResponseFormatType, // always json_object
}

impl JsonObjectFormat {
    pub fn new() -> Self {
        Self {
            type_field: ResponseFormatType::JsonObject,
        }
    }
}

impl Default for JsonObjectFormat {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseFormat {
    Text(TextFormat),
    JsonSchema(JsonSchemaFormat),
    JsonObject(JsonObjectFormat),
}

impl std::fmt::Display for ResponseFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseFormat::Text(_) => write!(f, "text"),
            ResponseFormat::JsonSchema(_) => write!(f, "json_schema"),
            ResponseFormat::JsonObject(_) => write!(f, "json_object"),
        }
    }
}

impl From<TextFormat> for ResponseFormat {
    fn from(text_format: TextFormat) -> Self {
        Self::Text(text_format)
    }
}

impl From<JsonSchemaFormat> for ResponseFormat {
    fn from(format: JsonSchemaFormat) -> Self {
        Self::JsonSchema(format)
    }
}

impl From<JsonObjectFormat> for ResponseFormat {
    fn from(format: JsonObjectFormat) -> Self {
        Self::JsonObject(format)
    }
}

impl TryFrom<ResponseFormat> for TextFormat {
    type Error = ConversionError;

    fn try_from(format: ResponseFormat) -> Result<Self, Self::Error> {
        match format {
            ResponseFormat::Text(inner) => Ok(inner),
            _ => Err(ConversionError::TryFrom("ResponseFormat".to_string())),
        }
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

impl TryFrom<ResponseFormat> for JsonObjectFormat {
    type Error = ConversionError;

    fn try_from(format: ResponseFormat) -> Result<Self, Self::Error> {
        match format {
            ResponseFormat::JsonObject(inner) => Ok(inner),
            _ => Err(ConversionError::TryFrom("ResponseFormat".to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Text {
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<ResponseFormat>,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            format: Some(ResponseFormat::Text(TextFormat::default())),
        }
    }
}

impl Text {
    pub fn response_format(mut self, value: ResponseFormat) -> Self {
        self.format = Some(value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_builds_text_response_format() {
        let result = Text::default().response_format(TextFormat::new().into());

        assert_eq!(
            result,
            Text {
                format: Some(ResponseFormat::Text(TextFormat {
                    type_field: ResponseFormatType::Text
                }))
            }
        );
    }

    #[test]
    fn it_builds_json_schema_response_format() {
        let schema = json!({
            "name": "Alice",
            "age": 30,
            "active": true,
            "friends": ["Bob", "Charlie"],
            "address": {
                "street": "123 Main St",
                "city": "Somewhere"
            }
        });

        let response_format: ResponseFormat = JsonSchemaFormat::new("test", schema.clone())
            .description("this is a description")
            .into();

        let result = Text::default().response_format(response_format);

        let expected = Text {
            format: Some(ResponseFormat::JsonSchema(JsonSchemaFormat {
                type_field: ResponseFormatType::JsonSchema,
                name: "test".to_string(),
                schema: schema,
                description: Some("this is a description".to_string()),
                strict: Some(false),
            })),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn it_builds_json_object_response_format() {
        let response_format: ResponseFormat = JsonObjectFormat::new().into();
        let result = Text::default().response_format(response_format);

        let expected = Text {
            format: Some(ResponseFormat::JsonObject(JsonObjectFormat {
                type_field: ResponseFormatType::JsonObject,
            })),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_json_values() {
        // Test default text format
        let text = Text::default();
        let json_value = serde_json::to_value(&text).unwrap();
        assert_eq!(
            json_value,
            serde_json::json!({
                "format": {
                    "type": "text"
                }
            })
        );

        // Test with JSON schema format
        let schema = json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" },
                "age": { "type": "number" },
                "active": { "type": "boolean" }
            },
            "required": ["name", "age"]
        });

        let json_schema_format = JsonSchemaFormat::new("user_data", schema.clone())
            .description("User information schema")
            .strict();
        let text_with_schema = Text::default().response_format(json_schema_format.into());
        let json_value = serde_json::to_value(&text_with_schema).unwrap();
        assert_eq!(
            json_value,
            serde_json::json!({
                "format": {
                    "type": "json_schema",
                    "name": "user_data",
                    "schema": {
                        "type": "object",
                        "properties": {
                            "name": { "type": "string" },
                            "age": { "type": "number" },
                            "active": { "type": "boolean" }
                        },
                        "required": ["name", "age"]
                    },
                    "description": "User information schema",
                    "strict": true
                }
            })
        );

        // Test with JSON object format
        let text_with_json_object = Text::default().response_format(JsonObjectFormat::new().into());
        let json_value = serde_json::to_value(&text_with_json_object).unwrap();
        assert_eq!(
            json_value,
            serde_json::json!({
                "format": {
                    "type": "json_object"
                }
            })
        );
    }
}
