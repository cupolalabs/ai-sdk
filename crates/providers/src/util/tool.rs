use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "lowercase")]
pub enum ComparisonOperator {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
}

impl FromStr for ComparisonOperator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "eq" => Ok(ComparisonOperator::Eq),
            "ne" => Ok(ComparisonOperator::Ne),
            "gt" => Ok(ComparisonOperator::Gt),
            "gte" => Ok(ComparisonOperator::Gte),
            "lt" => Ok(ComparisonOperator::Lt),
            "lte" => Ok(ComparisonOperator::Lte),
            _ => Err(format!("Invalid comparison operator: {}", s)),
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
    pub fn string(filter: String) -> Self {
        Self::String(filter)
    }

    pub fn boolean(filter: bool) -> Self {
        Self::Boolean(filter)
    }

    pub fn number(filter: f64) -> Self {
        Self::Number(filter)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ComparisonFilter<'a> {
    key: &'a str,
    #[serde(rename = "type")]
    type_field: ComparisonOperator,
    value: FilterValue,
}

impl From<String> for FilterValue {
    fn from(value: String) -> Self {
        FilterValue::String(value)
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

impl<'a> ComparisonFilter<'a> {
    pub fn build<V: Into<FilterValue>>(
        key: &'a str,
        comparison_operator: &'a str,
        value: V,
    ) -> Self {
        Self {
            key,
            type_field: ComparisonOperator::from_str(comparison_operator).unwrap(),
            value: value.into(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CompoundOperator {
    And,
    Or,
}

impl FromStr for CompoundOperator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "and" => Ok(CompoundOperator::And),
            "or" => Ok(CompoundOperator::Or),
            _ => Err(format!("Invalid compound operator: {}", s)),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct CompoundFilter<'a> {
    filters: Vec<FileSearchFilter<'a>>,
    #[serde(rename = "type")]
    type_field: CompoundOperator,
}

impl<'a> CompoundFilter<'a> {
    pub fn build(filters: Vec<FileSearchFilter<'a>>, compound_operator: &'a str) -> Self {
        Self {
            filters,
            type_field: CompoundOperator::from_str(compound_operator).unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub enum FileSearchFilter<'a> {
    Comparison(ComparisonFilter<'a>),
    Compound(CompoundFilter<'a>),
}

impl<'a> FileSearchFilter<'a> {
    pub fn build_comparison_filter<V: Into<FilterValue>>(
        key: &'a str,
        comparison_operator: &'a str,
        value: V,
    ) -> Self {
        Self::Comparison(ComparisonFilter::build(key, comparison_operator, value))
    }

    pub fn build_compound_filter(
        filters: Vec<FileSearchFilter<'a>>,
        compound_operator: &'a str,
    ) -> Self {
        Self::Compound(CompoundFilter::build(filters, compound_operator))
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RankingOptions<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ranker: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    score_threshold: Option<f32>,
}

impl<'a> RankingOptions<'a> {
    pub fn new() -> Self {
        Self {
            ranker: None,
            score_threshold: None,
        }
    }

    pub fn ranker(mut self, value: &'a str) -> Self {
        self.ranker = Some(value);
        self
    }

    pub fn score_threshold(mut self, value: f32) -> Self {
        self.score_threshold = Some(value);
        self
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct FileSearchTool<'a> {
    #[serde(rename = "type")]
    type_field: &'a str, // NOTE: this is always "file_search" value
    vector_store_ids: Vec<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<FileSearchFilter<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_num_results: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ranking_options: Option<RankingOptions<'a>>,
}

impl<'a> FileSearchTool<'a> {
    pub fn new(vector_store_ids: Vec<&'a str>) -> Self {
        Self {
            type_field: "file_search",
            vector_store_ids,
            filters: None,
            max_num_results: None,
            ranking_options: None,
        }
    }

    pub fn filters(mut self, filters: FileSearchFilter<'a>) -> Self {
        self.filters = Some(filters);
        self
    }

    pub fn max_num_results(mut self, value: u8) -> Self {
        self.max_num_results = Some(value);
        self
    }

    pub fn ranking_options(mut self, value: RankingOptions<'a>) -> Self {
        self.ranking_options = Some(value);
        self
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionTool<'a> {
    name: &'a str,
    parameters: serde_json::Value,
    strict: bool,
    #[serde(rename = "type")]
    type_field: &'a str, // NOTE: this is always "function" value
    description: Option<&'a str>,
}

impl<'a> FunctionTool<'a> {
    pub fn new(name: &'a str, parameters: serde_json::Value) -> Self {
        Self {
            name,
            parameters,
            strict: true,
            type_field: "function",
            description: None,
        }
    }

    pub fn strict(mut self, value: bool) -> Self {
        self.strict = value;
        self
    }

    pub fn description(mut self, value: &'a str) -> Self {
        self.description = Some(value);
        self
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputerUseTool<'a> {
    display_height: f32,
    display_width: f32,
    environment: &'a str,
    #[serde(rename = "type")]
    type_field: &'a str, // NOTE: this is always "computer_use_preview" value
}

impl<'a> ComputerUseTool<'a> {
    pub fn new(display_height: f32, display_width: f32, environment: &'a str) -> Self {
        Self {
            display_height,
            display_width,
            environment,
            type_field: "computer_use_preview",
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "lowercase")]
pub enum SearchContextSize {
    Low,
    Medium,
    High,
}

impl FromStr for SearchContextSize {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "low" => Ok(SearchContextSize::Low),
            "medium" => Ok(SearchContextSize::Medium),
            "high" => Ok(SearchContextSize::High),
            _ => Err(format!("Invalid search_context_size value: {}", s)),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserLocation<'a> {
    #[serde(rename = "type")]
    type_field: &'a str, // NOTE: this is always "approximate" value
    city: Option<&'a str>,
    country: Option<&'a str>, // NOTE: this is ISO-3166 country code
    region: Option<&'a str>,
    timezone: Option<&'a str>, // NOTE: this is IANA timezone
}

impl<'a> UserLocation<'a> {
    pub fn new() -> Self {
        Self {
            type_field: "approximate",
            city: None,
            country: None,
            region: None,
            timezone: None,
        }
    }

    pub fn city(mut self, value: &'a str) -> Self {
        self.city = Some(value);
        self
    }

    pub fn country(mut self, value: &'a str) -> Self {
        self.country = Some(value);
        self
    }

    pub fn region(mut self, value: &'a str) -> Self {
        self.region = Some(value);
        self
    }

    pub fn timezone(mut self, value: &'a str) -> Self {
        self.timezone = Some(value);
        self
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSearchTool<'a> {
    #[serde(rename = "type")]
    type_field: &'a str, // NOTE: this is either web_search_preview or web_search_preview_2025_03_11C
    #[serde(skip_serializing_if = "Option::is_none")]
    search_context_size: Option<SearchContextSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_location: Option<UserLocation<'a>>,
}

impl<'a> WebSearchTool<'a> {
    fn valid_types() -> &'static [&'static str] {
        &["web_search_preview", "web_search_preview_2025_03_11C"]
    }

    pub fn new(tool_type: &'a str) -> Result<Self, String> {
        if Self::valid_types().contains(&tool_type) {
            Ok(Self {
                type_field: tool_type,
                search_context_size: None,
                user_location: None,
            })
        } else {
            Err(format!("Invalid web search tool type value: {}", tool_type))
        }
    }

    pub fn search_context_size(mut self, value: &'a str) -> Self {
        self.search_context_size = Some(SearchContextSize::from_str(value).unwrap());

        self
    }

    pub fn user_location(mut self, value: UserLocation<'a>) -> Self {
        self.user_location = Some(value);

        self
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
#[serde(untagged)]
pub enum Tool<'a> {
    FileSearch(FileSearchTool<'a>),
    Function(FunctionTool<'a>),
    ComputerUse(ComputerUseTool<'a>),
    WebSearch(WebSearchTool<'a>),
}

impl<'a> From<FileSearchTool<'a>> for Tool<'a> {
    fn from(tool: FileSearchTool<'a>) -> Self {
        Tool::FileSearch(tool)
    }
}

impl<'a> TryFrom<Tool<'a>> for FileSearchTool<'a> {
    type Error = String;

    fn try_from(tool: Tool<'a>) -> Result<Self, Self::Error> {
        match tool {
            Tool::FileSearch(inner) => Ok(inner),
            _ => Err("Unable to convert Tool into FileSearch".to_string()),
        }
    }
}

impl<'a> From<FunctionTool<'a>> for Tool<'a> {
    fn from(tool: FunctionTool<'a>) -> Self {
        Tool::Function(tool)
    }
}

impl<'a> TryFrom<Tool<'a>> for FunctionTool<'a> {
    type Error = String;

    fn try_from(tool: Tool<'a>) -> Result<Self, Self::Error> {
        match tool {
            Tool::Function(inner) => Ok(inner),
            _ => Err("Unable to convert Tool into Function".to_string()),
        }
    }
}

impl<'a> From<ComputerUseTool<'a>> for Tool<'a> {
    fn from(tool: ComputerUseTool<'a>) -> Self {
        Tool::ComputerUse(tool)
    }
}

impl<'a> TryFrom<Tool<'a>> for ComputerUseTool<'a> {
    type Error = String;

    fn try_from(tool: Tool<'a>) -> Result<Self, Self::Error> {
        match tool {
            Tool::ComputerUse(inner) => Ok(inner),
            _ => Err("Unable to convert Tool into ComputerUse".to_string()),
        }
    }
}

impl<'a> From<WebSearchTool<'a>> for Tool<'a> {
    fn from(tool: WebSearchTool<'a>) -> Self {
        Tool::WebSearch(tool)
    }
}

impl<'a> TryFrom<Tool<'a>> for WebSearchTool<'a> {
    type Error = String;

    fn try_from(tool: Tool<'a>) -> Result<Self, Self::Error> {
        match tool {
            Tool::WebSearch(inner) => Ok(inner),
            _ => Err("Unable to convert Tool into WebSearchTool".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_creates_file_search_tool_with_comparison_operator() {
        let vector_store_ids = vec!["id_1", "id_2", "id_3", "id_4"];
        // NOTE: try_from here is to test the TryFrom trait that's implemented into Tool
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
                "test_value".to_string(),
            ))
            .max_num_results(1)
            .into();

        let expected = Tool::FileSearch(FileSearchTool {
            type_field: "file_search",
            vector_store_ids,
            ranking_options: Some(RankingOptions {
                ranker: Some("test_ranker"),
                score_threshold: Some(1.0),
            }),
            filters: Some(FileSearchFilter::Comparison(ComparisonFilter {
                key: "test_key",
                type_field: ComparisonOperator::Eq,
                value: FilterValue::String("test_value".to_string()),
            })),
            max_num_results: Some(1),
        });

        assert_eq!(tool, expected);
    }

    #[test]
    fn it_creates_file_search_tool_with_compound_operator() {
        let vector_store_ids = vec!["id_1", "id_2", "id_3", "id_4"];
        let tool: Tool = FileSearchTool::new(vector_store_ids.clone())
            .filters(FileSearchFilter::build_compound_filter(
                vec![FileSearchFilter::build_comparison_filter(
                    "test_key",
                    "eq",
                    "test_value".to_string(),
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
            type_field: "file_search",
            vector_store_ids,
            ranking_options: Some(RankingOptions {
                ranker: Some("test_ranker"),
                score_threshold: Some(1.0),
            }),
            filters: Some(FileSearchFilter::Compound(CompoundFilter {
                type_field: CompoundOperator::And,
                filters: vec![FileSearchFilter::Comparison(ComparisonFilter {
                    key: "test_key",
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
            description: Some("this is description"),
            type_field: "function",
            strict: true,
            parameters: json!({"name": "test"}),
            name: "function_tool_test",
        });

        assert_eq!(tool, expected);
    }

    #[test]
    fn it_creates_computer_use_tool() {
        let tool: Tool = ComputerUseTool::new(64.0, 64.0, "test_environment").into();

        let expected = Tool::ComputerUse(ComputerUseTool {
            type_field: "computer_use_preview",
            environment: "test_environment",
            display_width: 64.0,
            display_height: 64.0,
        });

        assert_eq!(tool, expected);
    }

    #[test]
    fn it_creates_web_search_tool() {
        let tool: Tool = WebSearchTool::new("web_search_preview")
            .unwrap()
            .search_context_size("low")
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
                type_field: "approximate",
                city: Some("Istanbul"),
                country: Some("TR"),
                region: Some("Marmara"),
                timezone: Some("Europe/Istanbul"),
            }),
            search_context_size: Some(SearchContextSize::Low),
            type_field: "web_search_preview",
        });

        assert_eq!(tool, expected);
    }
}
