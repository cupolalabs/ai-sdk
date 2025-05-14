use serde::{Deserialize, Serialize};

use crate::errors::ConversionError;

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
pub struct JsonSchemaFormat<'a> {
    #[serde(rename = "type")]
    type_field: ResponseFormatType, // always jsonschema
    name: &'a str,
    schema: serde_json::Value,
    description: Option<&'a str>,
    strict: Option<bool>,
}

impl<'a> JsonSchemaFormat<'a> {
    pub fn new(name: &'a str, schema: serde_json::Value) -> Self {
        Self {
            type_field: ResponseFormatType::JsonSchema,
            name,
            schema,
            description: None,
            strict: Some(false),
        }
    }

    pub fn description(mut self, value: &'a str) -> Self {
        self.description = Some(value);
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
    type_field: ResponseFormatType, // always jsonobject
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
#[serde(bound(deserialize = "'de: 'a"))]
#[serde(untagged)]
pub enum ResponseFormat<'a> {
    Text(TextFormat),
    JsonSchema(JsonSchemaFormat<'a>),
    JsonObject(JsonObjectFormat),
}

impl From<TextFormat> for ResponseFormat<'_> {
    fn from(text_format: TextFormat) -> Self {
        Self::Text(text_format)
    }
}

impl<'a> TryFrom<ResponseFormat<'a>> for TextFormat {
    type Error = ConversionError;

    fn try_from(response_format: ResponseFormat<'a>) -> Result<Self, Self::Error> {
        match response_format {
            ResponseFormat::Text(text_format) => Ok(text_format),
            _ => Err(ConversionError::TryFrom("ResponseFormat".to_string())),
        }
    }
}

impl<'a> From<JsonSchemaFormat<'a>> for ResponseFormat<'a> {
    fn from(json_schema_format: JsonSchemaFormat<'a>) -> Self {
        Self::JsonSchema(json_schema_format)
    }
}

impl<'a> TryFrom<ResponseFormat<'a>> for JsonSchemaFormat<'a> {
    type Error = ConversionError;

    fn try_from(response_format: ResponseFormat<'a>) -> Result<Self, Self::Error> {
        match response_format {
            ResponseFormat::JsonSchema(json_schema_format) => Ok(json_schema_format),
            _ => Err(ConversionError::TryFrom("ResponseFormat".to_string())),
        }
    }
}

impl From<JsonObjectFormat> for ResponseFormat<'_> {
    fn from(json_object_format: JsonObjectFormat) -> Self {
        Self::JsonObject(json_object_format)
    }
}

impl TryFrom<ResponseFormat<'_>> for JsonObjectFormat {
    type Error = ConversionError;

    fn try_from(response_format: ResponseFormat<'_>) -> Result<Self, Self::Error> {
        match response_format {
            ResponseFormat::JsonObject(json_object_format) => Ok(json_object_format),
            _ => Err(ConversionError::TryFrom("ResponseFormat".to_string())),
        }
    }
}

impl Default for ResponseFormat<'_> {
    fn default() -> Self {
        TextFormat::new().into()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct Text<'a> {
    format: Option<ResponseFormat<'a>>,
}

impl Default for Text<'_> {
    fn default() -> Self {
        Self {
            format: Some(ResponseFormat::default()),
        }
    }
}

impl<'a> Text<'a> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn response_format(mut self, value: ResponseFormat<'a>) -> Self {
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
        let result = Text::new().response_format(TextFormat::new().into());

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

        let result = Text::new().response_format(response_format);

        let expected = Text {
            format: Some(ResponseFormat::JsonSchema(JsonSchemaFormat {
                type_field: ResponseFormatType::JsonSchema,
                name: "test",
                schema,
                description: Some("this is a description"),
                strict: Some(false),
            })),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn it_builds_json_object_response_format() {
        let response_format: ResponseFormat = JsonObjectFormat::new().into();
        let result = Text::new().response_format(response_format);

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
        let text = Text::new();
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
        let text_with_schema = Text::new().response_format(json_schema_format.into());
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
        let text_with_json_object = Text::new().response_format(JsonObjectFormat::new().into());
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
