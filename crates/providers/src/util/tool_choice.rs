use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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

impl HostedToolChoice {
    pub fn new(hosted_tool_type: &str) -> Self {
        Self {
            type_field: HostedToolType::from_str(hosted_tool_type).unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionToolChoice<'a> {
    name: &'a str,
    #[serde(rename = "type")]
    type_field: &'a str,
}

impl<'a> FunctionToolChoice<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            type_field: "function",
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
#[serde(untagged)]
pub enum ToolChoice<'a> {
    Mode(ToolChoiceMode),
    HostedTool(HostedToolChoice),
    FunctionTool(FunctionToolChoice<'a>),
}

impl<'a> From<ToolChoiceMode> for ToolChoice<'a> {
    fn from(tool: ToolChoiceMode) -> Self {
        ToolChoice::Mode(tool)
    }
}

impl<'a> From<HostedToolChoice> for ToolChoice<'a> {
    fn from(tool: HostedToolChoice) -> Self {
        ToolChoice::HostedTool(tool)
    }
}

impl<'a> From<FunctionToolChoice<'a>> for ToolChoice<'a> {
    fn from(tool: FunctionToolChoice<'a>) -> Self {
        ToolChoice::FunctionTool(tool)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_choice_mode() {
        let result: ToolChoice = ToolChoiceMode::from_str("auto").unwrap().into();
        let expected = ToolChoice::Mode(ToolChoiceMode::Auto);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_builds_hosted_tool() {
        let result: ToolChoice = HostedToolChoice::new("web_search_preview").into();
        let expected = ToolChoice::HostedTool(HostedToolChoice {
            type_field: HostedToolType::from_str("web_search_preview").unwrap(),
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn it_builds_function_tool() {
        let result: ToolChoice = FunctionToolChoice::new("test name").into();
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
            let result: ToolChoice = ToolChoiceMode::from_str(input).unwrap().into();
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
            let result: ToolChoice = HostedToolChoice::new(input).into();
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
