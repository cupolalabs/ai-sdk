use crate::input_builder::Input;
use serde::{Deserialize, Serialize};

// -- include field starts here --
#[derive(Serialize, Deserialize)]
enum Include {
    #[serde(rename = "file_search_call.results")]
    FileSearchCallResults,
    #[serde(rename = "message.input_image.image_url")]
    MessageInputImageUrl,
    #[serde(rename = "computer_call_output.output.image_url")]
    ComputerCallOutputImageUrl,
}
// -- include field ends here --

// -- reasoning starts here --
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Effort {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Summary {
    Auto,
    Concise,
    Detailed,
}

#[derive(Serialize, Deserialize)]
struct Reasoning {
    effort: Option<Effort>,
    summary: Option<Summary>,
}
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
struct JsonSchemaFormat {
    #[serde(rename = "type")]
    type_field: ResponseFormatType, // always jsonschema
    name: String,
    schema: serde_json::Value,
    description: Option<String>,
    strict: Option<bool>,
}

#[derive(Serialize, Deserialize)]
struct JsonObjectFormat {
    #[serde(rename = "type")]
    type_field: ResponseFormatType, // always jsonobject
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum ResponseFormat {
    Text(TextFormat),
    JsonSchema(JsonSchemaFormat),
    JsonObject(JsonObjectFormat),
}

#[derive(Serialize, Deserialize)]
struct Text {
    format: Option<ResponseFormat>,
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
struct FunctionToolChoice {
    name: String,
    #[serde(rename = "type")]
    type_field: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum ToolChoice {
    Mode(ToolChoiceMode),
    HostedTool(HostedTool),
    FunctionTool(FunctionToolChoice),
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
    Number(f64),
    Boolean(bool),
}

#[derive(Serialize, Deserialize)]
struct ComparisonFilter {
    key: String,
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
struct CompoundFilter {
    filters: Vec<FileSearchFilter>,
    #[serde(rename = "type")]
    type_field: CompoundOperationType,
}

#[derive(Serialize, Deserialize)]
enum FileSearchFilter {
    Comparison(ComparisonFilter),
    Compound(CompoundFilter),
}

#[derive(Serialize, Deserialize)]
struct RankingOptions {
    ranker: Option<String>,
    score_threshold: Option<f32>,
}

#[derive(Serialize, Deserialize)]
struct FileSearchTool {
    #[serde(rename = "type")]
    type_field: String, // NOTE: this is always "file_search" value
    vector_store_ids: Vec<String>,
    filters: Option<FileSearchFilter>,
    max_num_results: Option<u8>,
    ranking_options: Option<RankingOptions>,
}

#[derive(Serialize, Deserialize)]
struct FunctionTool {
    name: String,
    parameters: serde_json::Value,
    strict: bool,
    #[serde(rename = "type")]
    type_field: String, // NOTE: this is always "function" value
    description: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ComputerUseTool {
    display_height: f32,
    display_width: f32,
    environment: String,
    #[serde(rename = "type")]
    type_field: String, // NOTE: this is always "computer_use_preview" value
}

#[derive(Serialize, Deserialize)]
struct UserLocation {
    #[serde(rename = "type")]
    type_field: String, // NOTE: this is always "approximate" value
    city: Option<String>,
    country: Option<String>, // NOTE: this is ISO-3166 country code
    region: Option<String>,
    timezone: Option<String>, // NOTE: this is IANA timezone
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "lowercase")]
enum SearchContextSize {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize)]
struct WebSearchTool {
    #[serde(rename = "type")]
    type_field: String, // NOTE: this is either web_search_preview or web_search_preview_2025_03_11C
    search_context_size: Option<SearchContextSize>,
    user_location: Option<UserLocation>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Tool {
    FileSearch(FileSearchTool),
    Function(FunctionTool),
    ComputerUse(ComputerUseTool),
    WebSearch(WebSearchTool),
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
    text: Option<Text>,
    tool_choice: Option<ToolChoice>,
    tools: Option<Vec<Tool>>,
    top_p: Option<f32>,
    truncation: Option<Truncation>,
    user: Option<&'a str>,
}
