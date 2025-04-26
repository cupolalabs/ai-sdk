use crate::input_builder::Input;

// -- tool choice starts here --
enum ToolChoiceMode {
    None,
    Auto,
    Required,
}

enum HostedToolType {
    FileSearch,
    WebSearchPreview,
    ComputerUsePreview,
}

struct HostedTool {
    type_field: HostedToolType,
}

struct FunctionToolChoice {
    name: String,
    type_field: String,
}

enum ToolChoice {
    Mode(ToolChoiceMode),
    HostedTool(HostedTool),
    FunctionTool(FunctionToolChoice),
}
// -- tool choice ends here --

// -- tool starts here --
enum ComparisonOperator {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
}

enum FilterValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

struct ComparisonFilter {
    key: String,
    type_field: ComparisonOperator,
    value: FilterValue,
}

enum CompoundOperationType {
    And,
    Or,
}

enum Filter {
    Comparison(ComparisonFilter),
    Compound(CompoundFilter),
}

struct CompoundFilter {
    filters: Vec<Filter>,
    type_field: CompoundOperationType,
}

struct RankingOptions {
    ranker: Option<String>,
    score_threshold: Option<f32>,
}

struct FileSearchTool {
    type_field: String,
    vector_store_ids: Vec<String>,
    filters: Option<Filter>,
    max_num_results: Option<u8>,
    ranking_options: Option<RankingOptions>,
}

struct FunctionTool {
    name: String,
    parameters: serde_json::Value,
    strict: bool,
    type_field: String,
    description: Option<String>,
}

struct ComputerUseTool {
    display_height: f32,
    display_width: f32,
    environment: String,
    type_field: String,
}

struct UserLocation {
    type_field: String,
    city: Option<String>,
    country: Option<String>,
    region: Option<String>,
    timezone: Option<String>,
}

struct WebSearchTool {
    type_field: String,
    search_context_size: Option<String>,
    user_location: Option<UserLocation>,
}

enum Tool {
    FileSearch(FileSearchTool),
    Function(FunctionTool),
    ComputerUse(ComputerUseTool),
    WebSearch(WebSearchTool),
}
// -- tool ends here --

// -- reasoning starts here --
enum Effort {
    Low,
    Medium,
    High,
}

enum Summary {
    Auto,
    Concise,
    Detailed,
}

struct Reasoning {
    effort: Option<Effort>,
    summary: Option<Summary>,
}
// -- reasoning ends here --

enum Truncation {
    Auto,
    Disabled,
}

enum ServiceTier {
    Auto,
    Default,
    Flex,
}

enum Include {
    FileSearchCallResults,
    MessageInputImageUrl,
    ComputerCallOutputImageUrl,
}

// -- text starts here --
enum ResponseFormatType {
    Text,
    JsonSchema,
    JsonObject,
}

struct TextFormat {
    type_field: ResponseFormatType, // always text
}

struct JsonSchemaFormat {
    type_field: ResponseFormatType, // always jsonschema
    name: String,
    schema: serde_json::Value,
    description: Option<String>,
    strict: Option<bool>,
}

struct JsonObjectFormat {
    type_field: ResponseFormatType, // always jsonobject
}

enum ResponseFormat {
    Text(TextFormat),
    JsonSchema(JsonSchemaFormat),
    JsonObject(JsonObjectFormat),
}

struct Text {
    format: Option<ResponseFormat>,
}
// -- text ends here --

struct Request<'a> {
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
