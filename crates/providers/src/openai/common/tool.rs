use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::openai::{
    constants::OpenAIModelId,
    errors::{ConversionError, InputError},
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ComparisonOperator {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
}

impl FromStr for ComparisonOperator {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "eq" => Ok(ComparisonOperator::Eq),
            "ne" => Ok(ComparisonOperator::Ne),
            "gt" => Ok(ComparisonOperator::Gt),
            "gte" => Ok(ComparisonOperator::Gte),
            "lt" => Ok(ComparisonOperator::Lt),
            "lte" => Ok(ComparisonOperator::Lte),
            _ => Err(ConversionError::FromStr(s.to_string())),
        }
    }
}

impl From<&str> for ComparisonOperator {
    fn from(s: &str) -> Self {
        Self::from_str(s).unwrap()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilterValue {
    String(String),
    Boolean(bool),
    Number(f64),
}

impl FilterValue {
    pub fn string(filter: impl Into<String>) -> Self {
        Self::String(filter.into())
    }

    pub fn boolean(filter: bool) -> Self {
        Self::Boolean(filter)
    }

    pub fn number(filter: f64) -> Self {
        Self::Number(filter)
    }
}

impl From<String> for FilterValue {
    fn from(value: String) -> Self {
        FilterValue::String(value)
    }
}

impl From<&str> for FilterValue {
    fn from(value: &str) -> Self {
        FilterValue::String(value.to_string())
    }
}

impl From<bool> for FilterValue {
    fn from(value: bool) -> Self {
        FilterValue::Boolean(value)
    }
}

impl From<f64> for FilterValue {
    fn from(value: f64) -> Self {
        FilterValue::Number(value)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompoundOperator {
    And,
    Or,
}

impl FromStr for CompoundOperator {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "and" => Ok(CompoundOperator::And),
            "or" => Ok(CompoundOperator::Or),
            _ => Err(ConversionError::FromStr(s.to_string())),
        }
    }
}

impl From<&str> for CompoundOperator {
    fn from(s: &str) -> Self {
        Self::from_str(s).unwrap()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FileSearchFilter {
    Comparison {
        key: String,
        #[serde(rename = "type")]
        type_field: ComparisonOperator,
        value: FilterValue,
    },
    Compound {
        filters: Vec<FileSearchFilter>,
        #[serde(rename = "type")]
        type_field: CompoundOperator,
    },
}

impl FileSearchFilter {
    pub fn comparison<V: Into<FilterValue>>(
        key: impl Into<String>,
        comparison_operator: impl AsRef<str>,
        value: V,
    ) -> Result<Self, ConversionError> {
        Ok(Self::Comparison {
            key: key.into(),
            type_field: ComparisonOperator::from_str(comparison_operator.as_ref())?,
            value: value.into(),
        })
    }

    pub fn compound(
        filters: Vec<FileSearchFilter>,
        compound_operator: impl AsRef<str>,
    ) -> Result<Self, ConversionError> {
        Ok(Self::Compound {
            filters,
            type_field: CompoundOperator::from_str(compound_operator.as_ref())?,
        })
    }
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RankingOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<f32>,
}

impl RankingOptions {
    pub fn new() -> Self {
        Self {
            ranker: None,
            score_threshold: None,
        }
    }

    pub fn ranker(mut self, value: impl Into<String>) -> Self {
        self.ranker = Some(value.into());
        self
    }

    pub fn score_threshold(mut self, value: f32) -> Self {
        self.score_threshold = Some(value.into());
        self
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FileSearchTool {
    pub vector_store_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FileSearchFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_num_results: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranking_options: Option<RankingOptions>,
}

impl FileSearchTool {
    pub fn new(vector_store_ids: Vec<impl Into<String>>) -> Self {
        Self {
            vector_store_ids: vector_store_ids.into_iter().map(|id| id.into()).collect(),
            filters: None,
            max_num_results: None,
            ranking_options: None,
        }
    }

    pub fn filters(mut self, filters: FileSearchFilter) -> Self {
        self.filters = Some(filters);
        self
    }

    pub fn max_num_results(mut self, value: u8) -> Self {
        self.max_num_results = Some(value);
        self
    }

    pub fn ranking_options(mut self, value: RankingOptions) -> Self {
        self.ranking_options = Some(value);
        self
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionTool {
    pub name: String,
    pub parameters: serde_json::Value,
    pub strict: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl FunctionTool {
    pub fn new(name: impl Into<String>, parameters: serde_json::Value, strict: bool) -> Self {
        Self {
            name: name.into(),
            parameters,
            strict,
            description: None,
        }
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputerUseTool {
    pub display_height: f32,
    pub display_width: f32,
    pub environment: String,
}

impl ComputerUseTool {
    pub fn new(display_height: f32, display_width: f32, environment: impl Into<String>) -> Self {
        Self {
            display_height,
            display_width,
            environment: environment.into(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchContextSize {
    Low,
    Medium,
    High,
}

impl FromStr for SearchContextSize {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "low" => Ok(SearchContextSize::Low),
            "medium" => Ok(SearchContextSize::Medium),
            "high" => Ok(SearchContextSize::High),
            _ => Err(ConversionError::FromStr(s.to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserLocation {
    #[serde(rename = "type")]
    pub type_field: String, // NOTE: this is always "approximate" value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>, // NOTE: this is ISO-3166 country code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>, // NOTE: this is IANA timezone
}

impl UserLocation {
    pub fn new() -> Self {
        Self {
            type_field: "approximate".to_string(),
            city: None,
            country: None,
            region: None,
            timezone: None,
        }
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn region(mut self, value: impl Into<String>) -> Self {
        self.region = Some(value.into());
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }
}

impl Default for UserLocation {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq)]
pub enum WebSearchVariant {
    Preview,
    Preview2025_03_11,
}

impl Default for WebSearchVariant {
    fn default() -> Self {
        Self::Preview
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSearchTool {
    #[serde(skip)]
    pub variant: WebSearchVariant,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_context_size: Option<SearchContextSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_location: Option<UserLocation>,
}

impl WebSearchTool {
    pub fn new() {}

    pub fn preview() -> Self {
        Self {
            variant: WebSearchVariant::Preview,
            search_context_size: None,
            user_location: None,
        }
    }

    pub fn preview_2025_03_11() -> Self {
        Self {
            variant: WebSearchVariant::Preview2025_03_11,
            search_context_size: None,
            user_location: None,
        }
    }

    pub fn search_context_size(mut self, value: SearchContextSize) -> Self {
        self.search_context_size = Some(value);
        self
    }

    pub fn user_location(mut self, value: UserLocation) -> Self {
        self.user_location = Some(value);
        self
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AllowedTools {
    MCPAllowedTools(Vec<String>),
    MCPAllowedToolsFilter(HashMap<String, Vec<String>>),
}

impl AllowedTools {
    pub fn from_allowed_tools(value: Vec<impl Into<String>>) -> Self {
        Self::MCPAllowedTools(value.into_iter().map(|v| v.into()).collect())
    }

    pub fn from_allowed_tools_filter(value: Vec<impl Into<String>>) -> Self {
        Self::MCPAllowedToolsFilter(HashMap::from([(
            "tool_names".to_string(),
            value.into_iter().map(|v| v.into()).collect(),
        )]))
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApprovalSetting {
    Always,
    Never,
}

impl FromStr for ApprovalSetting {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "always" => Ok(ApprovalSetting::Always),
            "never" => Ok(ApprovalSetting::Never),
            _ => Err(ConversionError::FromStr(s.to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequireApproval {
    MCPToolApprovalFilter {
        #[serde(rename = "always")]
        #[serde(skip_serializing_if = "Option::is_none")]
        always: Option<HashMap<String, Vec<String>>>,
        #[serde(rename = "never")]
        #[serde(skip_serializing_if = "Option::is_none")]
        never: Option<HashMap<String, Vec<String>>>,
    },
    MCPToolApprovalSetting(ApprovalSetting),
}

impl RequireApproval {
    // NOTE: always is a list of tools that always require approval
    // NOTE: never is a list of tools that never require approval
    pub fn from_approval_filter(
        always: Vec<impl Into<String>>,
        never: Vec<impl Into<String>>,
    ) -> Self {
        Self::MCPToolApprovalFilter {
            always: Some(HashMap::from([(
                "tool_names".to_string(),
                always.into_iter().map(|v| v.into()).collect(),
            )])),
            never: Some(HashMap::from([(
                "tool_names".to_string(),
                never.into_iter().map(|v| v.into()).collect(),
            )])),
        }
    }

    pub fn from_approval_setting(setting: impl AsRef<str>) -> Result<Self, ConversionError> {
        ApprovalSetting::from_str(setting.as_ref()).map(Self::MCPToolApprovalSetting)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MCPTool {
    pub server_label: String,
    pub server_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_tools: Option<AllowedTools>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_approval: Option<RequireApproval>,
}

impl MCPTool {
    pub fn new(server_label: impl Into<String>, server_url: impl Into<String>) -> Self {
        Self {
            server_label: server_label.into(),
            server_url: server_url.into(),
            allowed_tools: None,
            headers: None,
            require_approval: None,
        }
    }

    pub fn allowed_tools(mut self, value: AllowedTools) -> Self {
        self.allowed_tools = Some(value);
        self
    }

    pub fn headers(mut self, value: HashMap<String, String>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn require_approval(mut self, value: RequireApproval) -> Self {
        self.require_approval = Some(value);
        self
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeInterpreterContainer {
    ID(String),
    IDS {
        #[serde(rename = "type")]
        type_field: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        file_ids: Option<Vec<String>>,
    },
}

impl CodeInterpreterContainer {
    pub fn from_id(value: impl Into<String>) -> Self {
        Self::ID(value.into())
    }

    pub fn from_ids(value: Vec<impl Into<String>>) -> Self {
        Self::IDS {
            type_field: "auto".to_string(),
            file_ids: Some(value.into_iter().map(|v| v.into()).collect()),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeInterpreterTool {
    pub container: CodeInterpreterContainer,
}

impl CodeInterpreterTool {
    pub fn new(container: CodeInterpreterContainer) -> Self {
        Self { container }
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageGenerationBackground {
    Transparent,
    Opaque,
    #[default]
    Auto,
}

impl FromStr for ImageGenerationBackground {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "transparent" => Ok(ImageGenerationBackground::Transparent),
            "opaque" => Ok(ImageGenerationBackground::Opaque),
            "auto" => Ok(ImageGenerationBackground::Auto),
            _ => Err(ConversionError::TryFrom("ImageGenerationTool".to_string())),
        }
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct InputImageMask {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

impl InputImageMask {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageGenerationOutputFormat {
    #[default]
    PNG,
    WEBP,
    JPEG,
}

impl FromStr for ImageGenerationOutputFormat {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "png" => Ok(ImageGenerationOutputFormat::PNG),
            "webp" => Ok(ImageGenerationOutputFormat::WEBP),
            "jpeg" => Ok(ImageGenerationOutputFormat::JPEG),
            _ => Err(ConversionError::TryFrom(
                "ImageGenerationOutputFormat".to_string(),
            )),
        }
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageGenerationQuality {
    Low,
    Medium,
    High,
    #[default]
    Auto,
}

impl FromStr for ImageGenerationQuality {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "low" => Ok(ImageGenerationQuality::Low),
            "medium" => Ok(ImageGenerationQuality::Medium),
            "high" => Ok(ImageGenerationQuality::High),
            "auto" => Ok(ImageGenerationQuality::Auto),
            _ => Err(ConversionError::TryFrom(
                "ImageGenerationQuality".to_string(),
            )),
        }
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageGenerationSize {
    #[serde(rename = "1024x1024")]
    Size1024x1024,
    #[serde(rename = "1024x1536")]
    Size1024x1536,
    #[serde(rename = "1536x1024")]
    Size1536x1024,
    #[default]
    Auto,
}

impl FromStr for ImageGenerationSize {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1024x1024" => Ok(ImageGenerationSize::Size1024x1024),
            "1024x1536" => Ok(ImageGenerationSize::Size1024x1536),
            "1536x1024" => Ok(ImageGenerationSize::Size1536x1024),
            "auto" => Ok(ImageGenerationSize::Auto),
            _ => Err(ConversionError::TryFrom("ImageGenerationSize".to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageGenerationTool {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<ImageGenerationBackground>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_image_mask: Option<InputImageMask>,
    // NOTE: Image generation model. Currently, we accept OpenAIModelId and
    // there is no control over the image generation model names -- defaults to gpt-image-1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<OpenAIModelId>,
    // NOTE: there is no explanation for the values in docs -- defaults to auto
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation: Option<String>,
    // NOTE: defaults to 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_compression: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<ImageGenerationOutputFormat>,
    // NOTE: this value can be from 0 to 3 -- defaults to 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_image: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<ImageGenerationQuality>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageGenerationSize>,
}

impl Default for ImageGenerationTool {
    fn default() -> Self {
        Self {
            background: None,
            input_image_mask: None,
            model: Some(OpenAIModelId::GptImage1),
            moderation: Some("auto".into()),
            output_compression: Some(100),
            output_format: None,
            partial_image: Some(0),
            quality: None,
            size: None,
        }
    }
}

impl ImageGenerationTool {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn background(mut self, value: impl Into<String>) -> Result<Self, ConversionError> {
        self.background = Some(ImageGenerationBackground::from_str(&value.into())?);
        Ok(self)
    }

    pub fn input_image_mask(mut self, value: InputImageMask) -> Self {
        self.input_image_mask = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(OpenAIModelId::from_str(&value.into()).unwrap());
        self
    }

    pub fn moderation(mut self, value: impl Into<String>) -> Self {
        self.moderation = Some(value.into());
        self
    }

    pub fn output_compression(mut self, value: usize) -> Self {
        self.output_compression = Some(value);
        self
    }

    pub fn output_format(mut self, value: impl Into<String>) -> Result<Self, ConversionError> {
        self.output_format = Some(ImageGenerationOutputFormat::from_str(&value.into())?);
        Ok(self)
    }

    pub fn partial_image(mut self, value: usize) -> Result<Self, InputError> {
        if value > 3 {
            return Err(InputError::InvalidPartialImage(value));
        }
        self.partial_image = Some(value);
        Ok(self)
    }

    pub fn quality(mut self, value: impl Into<String>) -> Result<Self, ConversionError> {
        self.quality = Some(ImageGenerationQuality::from_str(&value.into())?);
        Ok(self)
    }

    pub fn size(mut self, value: impl Into<String>) -> Result<Self, ConversionError> {
        self.size = Some(ImageGenerationSize::from_str(&value.into())?);
        Ok(self)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Tool {
    #[serde(rename = "web_search_preview")]
    WebSearch(WebSearchTool),
    #[serde(rename = "web_search_preview_2025_03_11")]
    WebSearchPreview2025_03_11(WebSearchTool),
    #[serde(rename = "file_search")]
    FileSearch(FileSearchTool),
    #[serde(rename = "function")]
    Function(FunctionTool),
    #[serde(rename = "computer_use_preview")]
    ComputerUse(ComputerUseTool),
    #[serde(rename = "mcp")]
    MCP(MCPTool),
    #[serde(rename = "code_interpreter")]
    CodeInterpreter(CodeInterpreterTool),
    #[serde(rename = "image_generation")]
    ImageGeneration(ImageGenerationTool),
    #[serde(rename = "local_shell")]
    LocalShell,
}

impl From<FileSearchTool> for Tool {
    fn from(tool: FileSearchTool) -> Self {
        Tool::FileSearch(tool)
    }
}

impl TryFrom<Tool> for FileSearchTool {
    type Error = ConversionError;

    fn try_from(tool: Tool) -> Result<Self, Self::Error> {
        match tool {
            Tool::FileSearch(inner) => Ok(inner),
            _ => Err(ConversionError::TryFrom("Tool".to_string())),
        }
    }
}

impl From<FunctionTool> for Tool {
    fn from(tool: FunctionTool) -> Self {
        Tool::Function(tool)
    }
}

impl TryFrom<Tool> for FunctionTool {
    type Error = ConversionError;

    fn try_from(tool: Tool) -> Result<Self, Self::Error> {
        match tool {
            Tool::Function(inner) => Ok(inner),
            _ => Err(ConversionError::TryFrom("Tool".to_string())),
        }
    }
}

impl From<ComputerUseTool> for Tool {
    fn from(tool: ComputerUseTool) -> Self {
        Tool::ComputerUse(tool)
    }
}

impl TryFrom<Tool> for ComputerUseTool {
    type Error = ConversionError;

    fn try_from(tool: Tool) -> Result<Self, Self::Error> {
        match tool {
            Tool::ComputerUse(inner) => Ok(inner),
            _ => Err(ConversionError::TryFrom("Tool".to_string())),
        }
    }
}

impl From<WebSearchTool> for Tool {
    fn from(tool: WebSearchTool) -> Self {
        match tool.variant {
            WebSearchVariant::Preview => Tool::WebSearch(tool),
            WebSearchVariant::Preview2025_03_11 => Tool::WebSearchPreview2025_03_11(tool),
        }
    }
}

impl TryFrom<Tool> for WebSearchTool {
    type Error = ConversionError;

    fn try_from(tool: Tool) -> Result<Self, Self::Error> {
        match tool {
            Tool::WebSearch(inner) | Tool::WebSearchPreview2025_03_11(inner) => Ok(inner),
            _ => Err(ConversionError::TryFrom("Tool".to_string())),
        }
    }
}

impl From<MCPTool> for Tool {
    fn from(tool: MCPTool) -> Self {
        Self::MCP(tool)
    }
}

impl TryFrom<Tool> for MCPTool {
    type Error = ConversionError;

    fn try_from(tool: Tool) -> Result<Self, Self::Error> {
        match tool {
            Tool::MCP(inner) => Ok(inner),
            _ => Err(ConversionError::TryFrom("Tool".to_string())),
        }
    }
}

impl From<CodeInterpreterTool> for Tool {
    fn from(tool: CodeInterpreterTool) -> Self {
        Self::CodeInterpreter(tool)
    }
}

impl TryFrom<Tool> for CodeInterpreterTool {
    type Error = ConversionError;

    fn try_from(tool: Tool) -> Result<Self, Self::Error> {
        match tool {
            Tool::CodeInterpreter(inner) => Ok(inner),
            _ => Err(ConversionError::TryFrom("Tool".to_string())),
        }
    }
}

impl From<ImageGenerationTool> for Tool {
    fn from(tool: ImageGenerationTool) -> Self {
        Self::ImageGeneration(tool)
    }
}

impl TryFrom<Tool> for ImageGenerationTool {
    type Error = ConversionError;

    fn try_from(tool: Tool) -> Result<Self, Self::Error> {
        match tool {
            Tool::ImageGeneration(inner) => Ok(inner),
            _ => Err(ConversionError::TryFrom("Tool".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_builds_web_search_tool() {
        let preview: Tool = WebSearchTool::preview()
            .search_context_size(SearchContextSize::High)
            .user_location(
                UserLocation::new()
                    .city("Istanbul")
                    .country("Turkey")
                    .region("Europe")
                    .timezone("UTC+3"),
            )
            .into();
        let preview_2025_03_11: Tool = WebSearchTool::preview_2025_03_11()
            .search_context_size(SearchContextSize::High)
            .user_location(
                UserLocation::new()
                    .city("San Fransisco")
                    .country("USA")
                    .region("Bay Area")
                    .timezone("UTC-7"),
            )
            .into();

        let preview_expected = json!({
            "type": "web_search_preview",
            "search_context_size": "high",
            "user_location": {
                "city": "Istanbul",
                "country": "Turkey",
                "region": "Europe",
                "timezone": "UTC+3",
                "type": "approximate"
            }
        });
        let preview_2025_03_11_expected = json!({
            "type": "web_search_preview_2025_03_11",
            "search_context_size": "high",
            "user_location": {
                "city": "San Fransisco",
                "country": "USA",
                "region": "Bay Area",
                "timezone": "UTC-7",
                "type": "approximate"
            }
        });

        assert_eq!(serde_json::to_value(preview).unwrap(), preview_expected);
        assert_eq!(
            serde_json::to_value(preview_2025_03_11).unwrap(),
            preview_2025_03_11_expected
        );
    }

    #[test]
    fn it_builds_file_search_tool() {
        let tool: Tool = FileSearchTool::new(vec!["vector-id-1", "vector-id-2"])
            .max_num_results(8)
            .ranking_options(
                RankingOptions::new()
                    .ranker("test-ranker")
                    .score_threshold(32.0),
            )
            .filters(
                FileSearchFilter::compound(
                    vec![
                        FileSearchFilter::comparison("comparison-1", "eq", "comparison-1").unwrap(),
                        FileSearchFilter::comparison("comparison-2", "eq", "comparison-2").unwrap(),
                    ],
                    "and",
                )
                .unwrap(),
            )
            .into();

        let expected = json!({
            "type": "file_search",
            "vector_store_ids": ["vector-id-1", "vector-id-2"],
            "max_num_results": 8,
            "ranking_options": {
                "ranker": "test-ranker",
                "score_threshold": 32.0
            },
            "filters": {
                "filters": [
                    {
                        "key": "comparison-1",
                        "type": "eq",
                        "value": "comparison-1"
                    },
                    {
                        "key": "comparison-2",
                        "type": "eq",
                        "value": "comparison-2"
                    }
                ],
                "type": "and"
            }
        });

        assert_eq!(serde_json::to_value(tool).unwrap(), expected);
    }

    #[test]
    fn it_builds_function_tool() {
        let tool: Tool = FunctionTool::new(
            "function_name",
            json!({
                "name": "function_name",
                "params": [{ "name": "value", "type": "u32" }]
            }),
            false,
        )
        .description("test-description")
        .into();

        let expected = json!({
            "name": "function_name",
            "parameters": {
                "name": "function_name",
                "params": [{
                    "name": "value",
                    "type": "u32"
                }]
            },
            "strict": false,
            "description": "test-description",
            "type": "function"
        });

        assert_eq!(serde_json::to_value(tool).unwrap(), expected);
    }

    #[test]
    fn it_builds_computer_use() {
        let tool: Tool = ComputerUseTool::new(1080.0, 1920.0, "test-environment").into();

        let expected = json!({
            "type": "computer_use_preview",
            "display_height": 1080.0,
            "display_width": 1920.0,
            "environment": "test-environment"
        });

        assert_eq!(serde_json::to_value(tool).unwrap(), expected);
    }

    #[test]
    fn it_builds_mcp() {
        let tool: Tool = MCPTool::new("server-label", "server-url")
            .allowed_tools(AllowedTools::from_allowed_tools_filter(vec![
                "allowed-tool-1",
                "allowed-tool-2",
                "allowed-tool-3",
            ]))
            .headers(HashMap::from([
                ("Authorization".to_string(), "Bearer token".to_string()),
                ("Accept-Language".to_string(), "en-us, en;q=0.5".to_string()),
            ]))
            .require_approval(RequireApproval::from_approval_filter(
                vec!["allowed-tool-1", "allowed-tool-2"],
                vec!["allowed-tool-3"],
            ))
            .into();

        let expected = json!({
            "server_label": "server-label",
            "server_url": "server-url",
            "type": "mcp",
            "allowed_tools": {
                "tool_names": ["allowed-tool-1", "allowed-tool-2", "allowed-tool-3"]
            },
            "headers": {
                "Authorization": "Bearer token",
                "Accept-Language": "en-us, en;q=0.5"
            },
            "require_approval": {
                "always": {
                    "tool_names": ["allowed-tool-1", "allowed-tool-2"]
                },
                "never": {
                    "tool_names": ["allowed-tool-3"]
                }
            }
        });

        assert_eq!(serde_json::to_value(tool).unwrap(), expected);
    }

    #[test]
    fn it_builds_code_interpreter() {
        let tool: Tool =
            CodeInterpreterTool::new(CodeInterpreterContainer::from_ids(vec!["id-1", "id-2"]))
                .into();

        let expected = json!({
            "container": {"type": "auto", "file_ids": ["id-1", "id-2"]},
            "type": "code_interpreter",
        });

        assert_eq!(serde_json::to_value(tool).unwrap(), expected);
    }

    #[test]
    fn it_builds_image_generation() {
        let tool: Tool = ImageGenerationTool::new()
            .background("transparent")
            .unwrap()
            .input_image_mask(
                InputImageMask::new()
                    .file_id("image-mask-file-id")
                    .image_url("image-mask-image-url"),
            )
            .model(OpenAIModelId::GptImage1)
            .moderation("other-than-auto")
            .output_compression(69)
            .output_format("webp")
            .unwrap()
            .partial_image(2)
            .unwrap()
            .quality("high")
            .unwrap()
            .size("1024x1024")
            .unwrap()
            .into();

        let expected = json!({
            "background": "transparent",
            "input_image_mask": {
                "file_id": "image-mask-file-id",
                "image_url": "image-mask-image-url",
            },
            "model": "gpt-image-1",
            "moderation": "other-than-auto",
            "output_compression": 69,
            "output_format": "webp",
            "partial_image": 2,
            "quality": "high",
            "size": "1024x1024",
            "type": "image_generation"
        });

        assert_eq!(serde_json::to_value(tool).unwrap(), expected);
    }

    #[test]
    fn it_builds_local_shell() {
        let tool = Tool::LocalShell;

        let expected = json!({
            "type": "local_shell"
        });

        assert_eq!(serde_json::to_value(tool).unwrap(), expected);
    }
}
