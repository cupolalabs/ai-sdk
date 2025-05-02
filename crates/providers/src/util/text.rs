use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "lowercase")]
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
#[serde(untagged)]
pub enum ResponseFormat<'a> {
    Text(TextFormat),
    JsonSchema(JsonSchemaFormat<'a>),
    JsonObject(JsonObjectFormat),
}

impl<'a> From<TextFormat> for ResponseFormat<'a> {
    fn from(text_format: TextFormat) -> Self {
        Self::Text(text_format)
    }
}

impl<'a> TryFrom<ResponseFormat<'a>> for TextFormat {
    type Error = String;

    fn try_from(response_format: ResponseFormat) -> Result<Self, Self::Error> {
        match response_format {
            ResponseFormat::Text(text_format) => Ok(text_format),
            _ => Err("Invalid ResponseFormat value for TextFormat type".to_string()),
        }
    }
}

impl<'a> From<JsonSchemaFormat<'a>> for ResponseFormat<'a> {
    fn from(json_schema_format: JsonSchemaFormat<'a>) -> Self {
        Self::JsonSchema(json_schema_format)
    }
}

impl<'a> TryFrom<ResponseFormat<'a>> for JsonSchemaFormat<'a> {
    type Error = String;

    fn try_from(response_format: ResponseFormat<'a>) -> Result<Self, Self::Error> {
        match response_format {
            ResponseFormat::JsonSchema(json_schema_format) => Ok(json_schema_format),
            _ => Err("Invalid ResponseFormat value for JsonSchemaFormat type".to_string()),
        }
    }
}

impl<'a> From<JsonObjectFormat> for ResponseFormat<'a> {
    fn from(json_object_format: JsonObjectFormat) -> Self {
        Self::JsonObject(json_object_format)
    }
}

impl<'a> TryFrom<ResponseFormat<'a>> for JsonObjectFormat {
    type Error = String;

    fn try_from(response_format: ResponseFormat<'a>) -> Result<Self, Self::Error> {
        match response_format {
            ResponseFormat::JsonObject(json_object_format) => Ok(json_object_format),
            _ => Err("Invalid ResponseFormat value for JsonObjectFormat type".to_string()),
        }
    }
}

impl<'a> Default for ResponseFormat<'a> {
    fn default() -> Self {
        TextFormat::new().into()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct Text<'a> {
    format: Option<ResponseFormat<'a>>,
}

impl<'a> Default for Text<'a> {
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
}
