use crate::util::{
    include::Include, input::Input, reasoning::Reasoning, service_tier::ServiceTier, text::Text,
    tool::Tool, tool_choice::ToolChoice, truncation::Truncation,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Entry<'a> {
    input: Input<'a>,
    include: Option<Vec<Include>>,
    instructions: Option<&'a str>,
    max_output_tokens: usize,
    metadata: Option<std::collections::HashMap<String, String>>,
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
