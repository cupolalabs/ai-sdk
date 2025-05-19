use std::str::FromStr;

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
pub struct ComparisonFilter {
    key: String,
    #[serde(rename = "type")]
    type_field: ComparisonOperator,
    value: FilterValue,
}

impl ComparisonFilter {
    pub fn build<V: Into<FilterValue>>(
        key: impl Into<String>,
        comparison_operator: impl AsRef<str>,
        value: V,
    ) -> Self {
        Self {
            key: key.into(),
            type_field: ComparisonOperator::from_str(comparison_operator.as_ref()).unwrap(),
            value: value.into(),
        }
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CompoundFilter {
    filters: Vec<FileSearchFilter>,
    #[serde(rename = "type")]
    type_field: CompoundOperator,
}

impl CompoundFilter {
    pub fn build(filters: Vec<FileSearchFilter>, compound_operator: impl AsRef<str>) -> Self {
        Self {
            filters,
            type_field: CompoundOperator::from_str(compound_operator.as_ref()).unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FileSearchFilter {
    Comparison(ComparisonFilter),
    Compound(CompoundFilter),
}

impl FileSearchFilter {
    pub fn build_comparison_filter<V: Into<FilterValue>>(
        key: impl Into<String>,
        comparison_operator: impl AsRef<str>,
        value: V,
    ) -> Self {
        Self::Comparison(ComparisonFilter::build(key, comparison_operator, value))
    }

    pub fn build_compound_filter(
        filters: Vec<FileSearchFilter>,
        compound_operator: impl AsRef<str>,
    ) -> Self {
        Self::Compound(CompoundFilter::build(filters, compound_operator))
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
        self.score_threshold = Some(value);
        self
    }
}

impl Default for RankingOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FileSearchTool {
    #[serde(rename = "type")]
    type_field: String,
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
            type_field: "file_search".to_string(),
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
    #[serde(rename = "type")]
    type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

impl FunctionTool {
    pub fn new(name: impl Into<String>, parameters: serde_json::Value) -> Self {
        Self {
            name: name.into(),
            parameters,
            strict: true,
            type_field: "function".to_string(),
            description: None,
        }
    }

    pub fn strict(mut self, value: bool) -> Self {
        self.strict = value;
        self
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
    #[serde(rename = "type")]
    type_field: String,
}

impl ComputerUseTool {
    pub fn new(display_height: f32, display_width: f32, environment: impl Into<String>) -> Self {
        Self {
            display_height,
            display_width,
            environment: environment.into(),
            type_field: "computer_use_preview".to_string(),
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSearchTool {
    #[serde(rename = "type")]
    type_field: String, // NOTE: this is either web_search_preview or web_search_preview_2025_03_11C
    #[serde(skip_serializing_if = "Option::is_none")]
    search_context_size: Option<SearchContextSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_location: Option<UserLocation>,
}

impl WebSearchTool {
    pub fn new(type_field: impl Into<String>) -> Self {
        Self {
            type_field: type_field.into(),
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
pub enum Tool {
    FileSearch(FileSearchTool),
    Function(FunctionTool),
    ComputerUse(ComputerUseTool),
    WebSearch(WebSearchTool),
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
        Tool::WebSearch(tool)
    }
}

impl TryFrom<Tool> for WebSearchTool {
    type Error = ConversionError;

    fn try_from(tool: Tool) -> Result<Self, Self::Error> {
        match tool {
            Tool::WebSearch(inner) => Ok(inner),
            _ => Err(ConversionError::TryFrom("Tool".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_creates_file_search_tool_with_comparison_operator() {
        let vector_store_ids = vec![
            "id_1".to_string(),
            "id_2".to_string(),
            "id_3".to_string(),
            "id_4".to_string(),
        ];
        let tool: Tool = FileSearchTool::new(vector_store_ids.clone()).into();
        let tool: Tool = FileSearchTool::try_from(tool)
            .unwrap()
            .ranking_options(
                RankingOptions::new()
                    .ranker("test_ranker")
                    .score_threshold(1.0),
            )
            .filters(FileSearchFilter::build_comparison_filter(
                "test_key",
                "eq",
                "test_value",
            ))
            .max_num_results(1)
            .into();

        let expected = Tool::FileSearch(FileSearchTool {
            type_field: "file_search".to_string(),
            vector_store_ids,
            ranking_options: Some(RankingOptions {
                ranker: Some("test_ranker".to_string()),
                score_threshold: Some(1.0),
            }),
            filters: Some(FileSearchFilter::Comparison(ComparisonFilter {
                key: "test_key".to_string(),
                type_field: ComparisonOperator::Eq,
                value: FilterValue::String("test_value".to_string()),
            })),
            max_num_results: Some(1),
        });

        assert_eq!(tool, expected);
    }

    #[test]
    fn it_creates_file_search_tool_with_compound_operator() {
        let vector_store_ids = vec![
            "id_1".to_string(),
            "id_2".to_string(),
            "id_3".to_string(),
            "id_4".to_string(),
        ];
        let tool: Tool = FileSearchTool::new(vector_store_ids.clone())
            .filters(FileSearchFilter::build_compound_filter(
                vec![FileSearchFilter::build_comparison_filter(
                    "test_key",
                    "eq",
                    "test_value",
                )],
                "and",
            ))
            .ranking_options(
                RankingOptions::new()
                    .ranker("test_ranker")
                    .score_threshold(1.0),
            )
            .into();

        let expected = Tool::FileSearch(FileSearchTool {
            type_field: "file_search".to_string(),
            vector_store_ids,
            ranking_options: Some(RankingOptions {
                ranker: Some("test_ranker".to_string()),
                score_threshold: Some(1.0),
            }),
            filters: Some(FileSearchFilter::Compound(CompoundFilter {
                type_field: CompoundOperator::And,
                filters: vec![FileSearchFilter::Comparison(ComparisonFilter {
                    key: "test_key".to_string(),
                    type_field: ComparisonOperator::Eq,
                    value: FilterValue::String("test_value".to_string()),
                })],
            })),
            max_num_results: None,
        });

        assert_eq!(tool, expected);
    }

    #[test]
    fn it_creates_function_tool() {
        let tool: Tool = FunctionTool::new(
            "function_tool_test",
            json!({
                "name": "test"
            }),
        )
        .description("this is description")
        .into();

        let expected = Tool::Function(FunctionTool {
            description: Some("this is description".to_string()),
            type_field: "function".to_string(),
            strict: true,
            parameters: json!({"name": "test"}),
            name: "function_tool_test".to_string(),
        });

        assert_eq!(tool, expected);
    }

    #[test]
    fn it_creates_computer_use_tool() {
        let tool: Tool = ComputerUseTool::new(64.0, 64.0, "test_environment").into();

        let expected = Tool::ComputerUse(ComputerUseTool {
            type_field: "computer_use_preview".to_string(),
            environment: "test_environment".to_string(),
            display_width: 64.0,
            display_height: 64.0,
        });

        assert_eq!(tool, expected);
    }

    #[test]
    fn it_creates_web_search_tool() {
        let tool: Tool = WebSearchTool::new("web_search_preview".to_string())
            .search_context_size(SearchContextSize::Low)
            .user_location(
                UserLocation::new()
                    .city("Istanbul")
                    .country("TR")
                    .region("Marmara")
                    .timezone("Europe/Istanbul"),
            )
            .into();

        let expected = Tool::WebSearch(WebSearchTool {
            user_location: Some(UserLocation {
                type_field: "approximate".to_string(),
                city: Some("Istanbul".to_string()),
                country: Some("TR".to_string()),
                region: Some("Marmara".to_string()),
                timezone: Some("Europe/Istanbul".to_string()),
            }),
            search_context_size: Some(SearchContextSize::Low),
            type_field: "web_search_preview".to_string(),
        });

        assert_eq!(tool, expected);
    }

    // test the json values of the tool
    #[test]
    fn test_json_values() {
        // FileSearchTool test
        let tool: Tool = FileSearchTool::new(vec!["id_1", "id_2"])
            .filters(FileSearchFilter::build_comparison_filter(
                "test_key",
                "eq",
                "test_value".to_string(),
            ))
            .max_num_results(1)
            .ranking_options(
                RankingOptions::new()
                    .ranker("test_ranker")
                    .score_threshold(1.0),
            )
            .into();
        let json_value = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json_value,
            serde_json::json!({
                "type": "file_search",
                "vector_store_ids": ["id_1", "id_2"],
                "filters": {
                    "type": "comparison",
                    "key": "test_key",
                    "type": "eq",
                    "value": "test_value"
                },
                "max_num_results": 1,
                "ranking_options": {
                    "ranker": "test_ranker",
                    "score_threshold": 1.0
                }
            })
        );

        // FunctionTool test
        let tool: Tool = FunctionTool::new("test", json!({}))
            .description("this is description")
            .into();
        let json_value = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json_value,
            serde_json::json!({
                "type": "function",
                "name": "test",
                "parameters": {},
                "strict": true,
                "description": "this is description"
            })
        );

        // ComputerUseTool test
        let tool: Tool = ComputerUseTool::new(64.0, 64.0, "test_environment").into();
        let json_value = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json_value,
            serde_json::json!({
                "type": "computer_use_preview",
                "environment": "test_environment",
                "display_width": 64.0,
                "display_height": 64.0
            })
        );

        // WebSearchTool test with web_search_preview
        let tool: Tool = WebSearchTool::new("web_search_preview".to_string())
            .search_context_size(SearchContextSize::Low)
            .user_location(
                UserLocation::new()
                    .city("Istanbul")
                    .country("TR")
                    .region("Marmara")
                    .timezone("Europe/Istanbul"),
            )
            .into();
        let json_value = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json_value,
            serde_json::json!({
                "type": "web_search_preview",
                "search_context_size": "low",
                "user_location": {
                    "type": "approximate",
                    "city": "Istanbul",
                    "country": "TR",
                    "region": "Marmara",
                    "timezone": "Europe/Istanbul"
                }
            })
        );

        // WebSearchTool test with web_search_preview_2025_03_11C
        let tool: Tool = WebSearchTool::new("web_search_preview_2025_03_11C".to_string())
            .search_context_size(SearchContextSize::Low)
            .user_location(
                UserLocation::new()
                    .city("Istanbul")
                    .country("TR")
                    .region("Marmara")
                    .timezone("Europe/Istanbul"),
            )
            .into();
        let json_value = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json_value,
            serde_json::json!({
                "type": "web_search_preview_2025_03_11C",
                "search_context_size": "low",
                "user_location": {
                    "type": "approximate",
                    "city": "Istanbul",
                    "country": "TR",
                    "region": "Marmara",
                    "timezone": "Europe/Istanbul"
                }
            })
        );
    }
}
