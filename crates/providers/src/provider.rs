use crate::util::{include::Include, input::Input, reasoning::Reasoning};
use serde::{Deserialize, Serialize};

// -- reasoning starts here --
// -- reasoning ends here --

// -- service_tier field starts here --
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ServiceTier {
    Auto,
    Default,
    Flex,
}
// -- service_tier field ends here --

// -- text field starts here --
#[derive(Serialize, Deserialize)]
#[serde(rename = "lowercase")]
enum ResponseFormatType {
    Text,
    JsonSchema,
    JsonObject,
}

#[derive(Serialize, Deserialize)]
struct TextFormat {
    #[serde(rename = "type")]
    type_field: ResponseFormatType, // always text
}

#[derive(Serialize, Deserialize)]
struct JsonSchemaFormat<'a> {
    #[serde(rename = "type")]
    type_field: ResponseFormatType, // always jsonschema
    name: &'a str,
    schema: serde_json::Value,
    description: Option<&'a str>,
    strict: Option<bool>,
}

#[derive(Serialize, Deserialize)]
struct JsonObjectFormat {
    #[serde(rename = "type")]
    type_field: ResponseFormatType, // always jsonobject
}

#[derive(Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
#[serde(untagged)]
enum ResponseFormat<'a> {
    Text(TextFormat),
    JsonSchema(JsonSchemaFormat<'a>),
    JsonObject(JsonObjectFormat),
}

#[derive(Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
struct Text<'a> {
    format: Option<ResponseFormat<'a>>,
}
// -- text field ends here --

// -- tool_choice field starts here --
#[derive(Serialize, Deserialize)]
#[serde(rename = "lowercase")]
enum ToolChoiceMode {
    None,
    Auto,
    Required,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "snake_case")]
enum HostedToolType {
    FileSearch,
    WebSearchPreview,
    ComputerUsePreview,
}

#[derive(Serialize, Deserialize)]
struct HostedTool {
    #[serde(rename = "type")]
    type_field: HostedToolType,
}

#[derive(Serialize, Deserialize)]
struct FunctionToolChoice<'a> {
    name: &'a str,
    #[serde(rename = "type")]
    type_field: &'a str,
}

#[derive(Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
#[serde(untagged)]
enum ToolChoice<'a> {
    Mode(ToolChoiceMode),
    HostedTool(HostedTool),
    FunctionTool(FunctionToolChoice<'a>),
}
// -- tool_choice field ends here --

// -- tool starts here --
#[derive(Serialize, Deserialize)]
#[serde(rename = "lowercase")]
enum ComparisonOperator {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum FilterValue {
    String(String),
    Boolean(bool),
    Number(f64),
}

#[derive(Serialize, Deserialize)]
struct ComparisonFilter<'a> {
    key: &'a str,
    #[serde(rename = "type")]
    type_field: ComparisonOperator,
    value: FilterValue,
}

#[derive(Serialize, Deserialize)]
enum CompoundOperationType {
    And,
    Or,
}

#[derive(Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
struct CompoundFilter<'a> {
    filters: Vec<FileSearchFilter<'a>>,
    #[serde(rename = "type")]
    type_field: CompoundOperationType,
}

#[derive(Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
enum FileSearchFilter<'a> {
    Comparison(ComparisonFilter<'a>),
    Compound(CompoundFilter<'a>),
}

#[derive(Serialize, Deserialize)]
struct RankingOptions<'a> {
    ranker: Option<&'a str>,
    score_threshold: Option<f32>,
}

#[derive(Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
struct FileSearchTool<'a> {
    #[serde(rename = "type")]
    type_field: &'a str, // NOTE: this is always "file_search" value
    vector_store_ids: Vec<&'a str>,
    filters: Option<FileSearchFilter<'a>>,
    max_num_results: Option<u8>,
    ranking_options: Option<RankingOptions<'a>>,
}

#[derive(Serialize, Deserialize)]
struct FunctionTool<'a> {
    name: &'a str,
    parameters: serde_json::Value,
    strict: bool,
    #[serde(rename = "type")]
    type_field: &'a str, // NOTE: this is always "function" value
    description: Option<&'a str>,
}

#[derive(Serialize, Deserialize)]
struct ComputerUseTool<'a> {
    display_height: f32,
    display_width: f32,
    environment: &'a str,
    #[serde(rename = "type")]
    type_field: &'a str, // NOTE: this is always "computer_use_preview" value
}

#[derive(Serialize, Deserialize)]
struct UserLocation<'a> {
    #[serde(rename = "type")]
    type_field: &'a str, // NOTE: this is always "approximate" value
    city: Option<&'a str>,
    country: Option<&'a str>, // NOTE: this is ISO-3166 country code
    region: Option<&'a str>,
    timezone: Option<&'a str>, // NOTE: this is IANA timezone
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "lowercase")]
enum SearchContextSize {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize)]
struct WebSearchTool<'a> {
    #[serde(rename = "type")]
    type_field: &'a str, // NOTE: this is either web_search_preview or web_search_preview_2025_03_11C
    search_context_size: Option<SearchContextSize>,
    user_location: Option<UserLocation<'a>>,
}

#[derive(Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
#[serde(untagged)]
enum Tool<'a> {
    FileSearch(FileSearchTool<'a>),
    Function(FunctionTool<'a>),
    ComputerUse(ComputerUseTool<'a>),
    WebSearch(WebSearchTool<'a>),
}
// -- tool ends here --

// -- trancation field starts here --
#[derive(Serialize, Deserialize)]
#[serde(rename = "losercase")]
enum Truncation {
    Auto,
    Disabled,
}
// -- trancation field ends here --

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
