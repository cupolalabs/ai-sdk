use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "lowercase")]
pub enum ToolChoiceMode {
    None,
    Auto,
    Required,
}

impl FromStr for ToolChoiceMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(ToolChoiceMode::None),
            "auto" => Ok(ToolChoiceMode::Auto),
            "required" => Ok(ToolChoiceMode::Required),
            _ => Err(format!("Invalid ToolChoiceMode value: {}", s)),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "snake_case")]
enum HostedToolType {
    FileSearch,
    WebSearchPreview,
    ComputerUsePreview,
}

impl FromStr for HostedToolType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "file_search" => Ok(HostedToolType::FileSearch),
            "web_search_preview" => Ok(HostedToolType::WebSearchPreview),
            "computer_use_preview" => Ok(HostedToolType::ComputerUsePreview),
            _ => Err(format!("Invalid HostedToolType value: {}", s)),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HostedToolChoice {
    #[serde(rename = "type")]
    type_field: HostedToolType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionToolChoice<'a> {
    name: &'a str,
    #[serde(rename = "type")]
    type_field: &'a str,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
#[serde(untagged)]
pub enum ToolChoice<'a> {
    Mode(ToolChoiceMode),
    HostedTool(HostedToolChoice),
    FunctionTool(FunctionToolChoice<'a>),
}

impl<'a> ToolChoice<'a> {
    pub fn build_tool_choice_mode(mode: &'a str) -> Self {
        let tool_choice_mode = ToolChoiceMode::from_str(mode).unwrap();

        ToolChoice::Mode(tool_choice_mode)
    }

    pub fn build_hosted_tool(hosted_tool: &'a str) -> Self {
        ToolChoice::HostedTool(HostedToolChoice {
            type_field: HostedToolType::from_str(hosted_tool).unwrap(),
        })
    }

    pub fn build_function_tool(name: &'a str) -> Self {
        ToolChoice::FunctionTool(FunctionToolChoice {
            name,
            type_field: "function",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_choice_mode() {
        let result = ToolChoice::build_tool_choice_mode("auto");
        let expected = ToolChoice::Mode(ToolChoiceMode::from_str("auto").unwrap());

        assert_eq!(result, expected);
    }

    #[test]
    fn it_builds_hosted_tool() {
        let result = ToolChoice::build_hosted_tool("web_search_preview");
        let expected = ToolChoice::HostedTool(HostedToolChoice {
            type_field: HostedToolType::from_str("web_search_preview").unwrap(),
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn it_builds_function_tool() {
        let result = ToolChoice::build_function_tool("test name");
        let expected = ToolChoice::FunctionTool(FunctionToolChoice {
            name: "test name",
            type_field: "function",
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn it_builds_tool_choice_from_str() {
        let test_cases = [
            (
                "auto",
                ToolChoice::Mode(ToolChoiceMode::from_str("auto").unwrap()),
            ),
            (
                "none",
                ToolChoice::Mode(ToolChoiceMode::from_str("none").unwrap()),
            ),
            (
                "required",
                ToolChoice::Mode(ToolChoiceMode::from_str("required").unwrap()),
            ),
        ];

        for (input, expected) in test_cases {
            let result = ToolChoice::build_tool_choice_mode(input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn it_builds_hosted_tool_from_str() {
        let test_cases = [
            (
                "file_search",
                ToolChoice::HostedTool(HostedToolChoice {
                    type_field: HostedToolType::from_str("file_search").unwrap(),
                }),
            ),
            (
                "web_search_preview",
                ToolChoice::HostedTool(HostedToolChoice {
                    type_field: HostedToolType::from_str("web_search_preview").unwrap(),
                }),
            ),
            (
                "computer_use_preview",
                ToolChoice::HostedTool(HostedToolChoice {
                    type_field: HostedToolType::from_str("computer_use_preview").unwrap(),
                }),
            ),
        ];

        for (input, expected) in test_cases {
            let result = ToolChoice::build_hosted_tool(input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn it_returns_error_for_invalid_tool_choice_mode() {
        let result = ToolChoiceMode::from_str("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn it_returns_error_for_invalid_hosted_tool_type() {
        let result = HostedToolType::from_str("invalid");
        assert!(result.is_err());
    }
}
