use std::{collections::HashMap, str::FromStr};

use futures::stream::All;
use serde::{Deserialize, Serialize};

use crate::openai::errors::ConversionError;

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
    ranker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    score_threshold: Option<f32>,
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
    vector_store_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<FileSearchFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_num_results: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ranking_options: Option<RankingOptions>,
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
    name: String,
    parameters: serde_json::Value,
    strict: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
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
    display_height: f32,
    display_width: f32,
    environment: String,
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
    type_field: String, // NOTE: this is always "approximate" value
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<String>, // NOTE: this is ISO-3166 country code
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timezone: Option<String>, // NOTE: this is IANA timezone
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
enum WebSearchVariant {
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
    variant: WebSearchVariant,
    #[serde(skip_serializing_if = "Option::is_none")]
    search_context_size: Option<SearchContextSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_location: Option<UserLocation>,
}

impl WebSearchTool {
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
        always: Option<HashMap<String, Vec<String>>>,
        #[serde(rename = "never")]
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
    pub allowed_tools: Option<AllowedTools>,
    pub headers: Option<HashMap<String, String>>,
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
}
