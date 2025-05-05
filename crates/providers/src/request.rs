use crate::util::{
    include::Include, input::Input, reasoning::Reasoning, service_tier::ServiceTier, text::Text,
    tool::Tool, tool_choice::ToolChoice, truncation::Truncation,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Request<'a> {
    input: Input<'a>,
    model: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<Vec<Include>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instructions: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    previous_response_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning: Option<Reasoning>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_tier: Option<ServiceTier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    store: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Text<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<ToolChoice<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tool<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    truncation: Option<Truncation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<&'a str>,
}

impl<'a> Request<'a> {
    pub fn new(model: &'a str, input: Input<'a>) -> Self {
        Self {
            model,
            input,
            ..Default::default()
        }
    }

    // NOTE: Adds single Include item to the include vector
    pub fn include(mut self, value: Include) -> Self {
        match self.include {
            Some(ref mut include) => include.push(value),
            None => self.include = Some(vec![value]),
        }

        self
    }

    pub fn instructions(mut self, value: &'a str) -> Self {
        self.instructions = Some(value);
        self
    }

    pub fn max_output_tokens(mut self, value: usize) -> Self {
        self.max_output_tokens = Some(value);
        self
    }

    // NOTE: this function inserts value using key to the metadata HashMap
    pub fn insert_metadata(mut self, key: String, value: String) -> Self {
        match self.metadata {
            Some(ref mut metadata) => {
                metadata.insert(key, value);
            }
            None => {
                self.metadata = Some({
                    let mut new_map: HashMap<String, String> = HashMap::new();
                    new_map.insert(key, value);
                    new_map
                });
            }
        }

        self
    }

    pub fn parallel_tool_calls(mut self, value: bool) -> Self {
        self.parallel_tool_calls = Some(value);
        self
    }

    pub fn previous_response_id(mut self, value: &'a str) -> Self {
        self.previous_response_id = Some(value);
        self
    }

    pub fn reasoning(mut self, value: Reasoning) -> Self {
        self.reasoning = Some(value);
        self
    }

    pub fn service_tier(mut self, value: ServiceTier) -> Self {
        self.service_tier = Some(value);
        self
    }

    pub fn store(mut self, value: bool) -> Self {
        self.store = Some(value);
        self
    }

    pub fn stream(mut self) -> Self {
        self.stream = Some(true);
        self
    }

    pub fn temperature(mut self, value: f32) -> Self {
        self.temperature = Some(value);
        self
    }

    pub fn text(mut self, value: Text<'a>) -> Self {
        self.text = Some(value);
        self
    }

    pub fn tool_choice(mut self, value: ToolChoice<'a>) -> Self {
        self.tool_choice = Some(value);
        self
    }

    // NOTE: this function adds  single Tool item to the tools vector
    pub fn add_tool(mut self, value: Tool<'a>) -> Self {
        match self.tools {
            Some(ref mut tools) => tools.push(value),
            None => self.tools = Some(vec![value]),
        }
        self
    }

    pub fn top_p(mut self, value: f32) -> Self {
        self.top_p = Some(value);
        self
    }

    pub fn truncation(mut self, value: Truncation) -> Self {
        self.truncation = Some(value);
        self
    }

    pub fn user(mut self, value: Truncation) -> Self {
        self.truncation = Some(value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{
        input::{Content, ImageContent, MultiContentInput},
        tool::{FileSearchTool, FunctionTool, WebSearchTool},
        tool_choice::ToolChoiceMode,
    };
    use serde_json::json;
    use std::str::FromStr;

    static MODEL: &str = "test-model";
    static PLACEHOLDER_CONTENT: &str = "test-input";
    static INSTRUCTIONS: &str = "You are a helpful assistant.";

    #[test]
    fn it_builds_request_with_text_input() {
        let request = Request::new(MODEL, PLACEHOLDER_CONTENT.into());

        let result = serde_json::to_value(&request).unwrap();
        let expected = json!({
            "model": MODEL,
            "input": PLACEHOLDER_CONTENT
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn it_builds_request_with_image_input() {
        let content: Content = ImageContent::build(PLACEHOLDER_CONTENT).into();
        let request = Request::new(
            MODEL,
            vec![MultiContentInput::new().append_content(vec![content])].into(),
        );

        let result = serde_json::to_value(&request).unwrap();
        let expected = json!({
            "model": MODEL,
            "input": [{
                "role": "user",
                "content": [{
                    "type": "input_image",
                    "image_url": PLACEHOLDER_CONTENT
                }]
            }]
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn it_builds_request_with_web_search() {
        let tool: Tool = WebSearchTool::new("web_search_preview").unwrap().into();
        let request = Request::new(MODEL, PLACEHOLDER_CONTENT.into()).add_tool(tool);

        let result = serde_json::to_value(&request).unwrap();
        let expected = json!({
            "model": MODEL,
            "tools": [{ "type": "web_search_preview" }],
            "input": PLACEHOLDER_CONTENT
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn it_builds_request_with_file_search() {
        let vector_store_ids = vec!["vs_test123"];
        let tool: Tool = FileSearchTool::new(vector_store_ids)
            .max_num_results(20)
            .into();
        let request = Request::new(MODEL, PLACEHOLDER_CONTENT.into()).add_tool(tool);

        let result = serde_json::to_value(&request).unwrap();
        let expected = json!({
            "model": MODEL,
            "tools": [{
                "type": "file_search",
                "vector_store_ids": ["vs_test123"],
                "max_num_results": 20
            }],
            "input": PLACEHOLDER_CONTENT
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn it_builds_request_with_streaming() {
        let request = Request::new(MODEL, PLACEHOLDER_CONTENT.into())
            .instructions(INSTRUCTIONS)
            .stream();

        let result = serde_json::to_value(&request).unwrap();
        let expected = json!({
            "model": MODEL,
            "instructions": INSTRUCTIONS,
            "input": PLACEHOLDER_CONTENT,
            "stream": true
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn it_builds_request_with_functions() {
        let function_name = "get_current_weather";
        let parameters = json!({
              "type": "object",
              "properties": {
                "location": {
                  "type": "string",
                  "description": "The city and state, e.g. San Francisco, CA"
                },
                "unit": {
                  "type": "string",
                  "enum": ["celsius", "fahrenheit"]
                }
              },
              "required": ["location", "unit"]
        });
        let description = "Get the current weather in a given location";
        let request = Request::new(MODEL, PLACEHOLDER_CONTENT.into())
            .add_tool(
                FunctionTool::new(function_name, parameters.clone())
                    .description(description)
                    .into(),
            )
            .tool_choice(ToolChoiceMode::from_str("auto").unwrap().into());

        let result = serde_json::to_value(&request).unwrap();
        let expected = json!({
                "model": MODEL,
                "input": PLACEHOLDER_CONTENT,
        "tools": [
          {
            "type": "function",
            "name": function_name,
            "description": description,
            "parameters": parameters,
            "strict": true
          }
        ],
                "tool_choice": "auto"
            });

        assert_eq!(result, expected);
    }

    #[test]
    fn it_builds_request_with_reasoning() {
        let request = Request::new(MODEL, PLACEHOLDER_CONTENT.into())
            .reasoning(Reasoning::new().effort("high"));
        let result = serde_json::to_value(request).unwrap();

        let expected = json!({
            "model": MODEL,
            "input": PLACEHOLDER_CONTENT,
            "reasoning": {
                "effort": "high"
            }
        });

        assert_eq!(result, expected);
    }
}
