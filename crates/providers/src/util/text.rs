use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "lowercase")]
enum ResponseFormatType {
    Text,
    JsonSchema,
    JsonObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TextFormat {
    #[serde(rename = "type")]
    type_field: ResponseFormatType, // always text
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct JsonSchemaFormat<'a> {
    #[serde(rename = "type")]
    type_field: ResponseFormatType, // always jsonschema
    name: &'a str,
    schema: serde_json::Value,
    description: Option<&'a str>,
    strict: Option<bool>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct JsonObjectFormat {
    #[serde(rename = "type")]
    type_field: ResponseFormatType, // always jsonobject
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
#[serde(untagged)]
enum ResponseFormat<'a> {
    Text(TextFormat),
    JsonSchema(JsonSchemaFormat<'a>),
    JsonObject(JsonObjectFormat),
}

impl<'a> Default for ResponseFormat<'a> {
    fn default() -> Self {
        Self::Text(TextFormat {
            type_field: ResponseFormatType::Text,
        })
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct Text<'a> {
    format: Option<ResponseFormat<'a>>,
}

impl<'a> Text<'a> {
    pub fn build_text_response_format() -> Self {
        Self {
            format: Some(ResponseFormat::default()),
        }
    }

    pub fn build_json_schema_response_format(
        name: &'a str,
        schema: serde_json::Value,
        description: Option<&'a str>,
        strict: Option<bool>,
    ) -> Self {
        Self {
            format: Some(ResponseFormat::JsonSchema(JsonSchemaFormat {
                type_field: ResponseFormatType::JsonSchema,
                name,
                schema,
                description,
                strict,
            })),
        }
    }

    pub fn build_json_object_response_format() -> Self {
        Self {
            format: Some(ResponseFormat::JsonObject(JsonObjectFormat {
                type_field: ResponseFormatType::JsonObject,
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_builds_text_response_format() {
        let result = Text::build_text_response_format();

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
        let value = json!({
            "name": "Alice",
            "age": 30,
            "active": true,
            "friends": ["Bob", "Charlie"],
            "address": {
                "street": "123 Main St",
                "city": "Somewhere"
            }
        });

        let result = Text::build_json_schema_response_format(
            "object",
            value.clone(),
            Some("this is a description"),
            Some(false),
        );

        let expected = Text {
            format: Some(ResponseFormat::JsonSchema(JsonSchemaFormat {
                type_field: ResponseFormatType::JsonSchema,
                name: "object",
                schema: value,
                description: Some("this is a description"),
                strict: Some(false),
            })),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn it_builds_json_object_response_format() {
        let result = Text::build_json_object_response_format();

        let expected = Text {
            format: Some(ResponseFormat::JsonObject(JsonObjectFormat {
                type_field: ResponseFormatType::JsonObject,
            })),
        };

        assert_eq!(result, expected);
    }
}
