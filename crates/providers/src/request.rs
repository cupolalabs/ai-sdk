use crate::util::{
    include::Include, input::Input, reasoning::Reasoning, service_tier::ServiceTier, text::Text,
    tool::Tool, tool_choice::ToolChoice, truncation::Truncation,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Serialize, Deserialize)]
pub struct Request<'a> {
    input: Input<'a>,
    include: Option<Vec<Include>>,
    instructions: Option<&'a str>,
    max_output_tokens: Option<usize>,
    metadata: Option<HashMap<String, String>>,
    parallel_tool_calls: Option<bool>,
    previous_response_id: Option<&'a str>,
    reasoning: Option<Reasoning>,
    service_tier: Option<ServiceTier>,
    store: Option<bool>,
    stream: Option<bool>,
    temperature: Option<f32>,
    text: Option<Text<'a>>,
    tool_choice: Option<ToolChoice<'a>>,
    tools: Option<Vec<Tool<'a>>>,
    top_p: Option<f32>,
    truncation: Option<Truncation>,
    user: Option<&'a str>,
}

impl<'a> Request<'a> {
    pub fn new(input: Input<'a>) -> Self {
        Self {
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

    // NOTE: Inserts value using key to the metadata HashMap
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

    pub fn stream(mut self, value: bool) -> Self {
        self.stream = Some(value);
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

    // NOTE: Adds single Tool item to the tools vector
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

    #[test]
    fn it_builds_request() {}
}
