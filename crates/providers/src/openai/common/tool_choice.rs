use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::openai::errors::ConversionError;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoiceMode {
    None,
    Auto,
    Required,
}

impl FromStr for ToolChoiceMode {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(ToolChoiceMode::None),
            "auto" => Ok(ToolChoiceMode::Auto),
            "required" => Ok(ToolChoiceMode::Required),
            _ => Err(ConversionError::FromStr(s.to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostedToolType {
    FileSearch,
    WebSearchPreview,
    ComputerUsePreview,
}

impl FromStr for HostedToolType {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "file_search" => Ok(HostedToolType::FileSearch),
            "web_search_preview" => Ok(HostedToolType::WebSearchPreview),
            "computer_use_preview" => Ok(HostedToolType::ComputerUsePreview),
            _ => Err(ConversionError::FromStr(s.to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HostedToolChoice {
    #[serde(rename = "type")]
    type_field: HostedToolType,
}

impl HostedToolChoice {
    pub fn new<T: Into<HostedToolType>>(hosted_tool_type: T) -> Self {
        Self {
            type_field: hosted_tool_type.into(),
        }
    }
}

impl From<&str> for HostedToolType {
    fn from(s: &str) -> Self {
        Self::from_str(s).unwrap()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionToolChoice {
    name: String,
    #[serde(rename = "type")]
    type_field: String,
}

impl FunctionToolChoice {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            type_field: "function".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolChoice {
    Mode(ToolChoiceMode),
    HostedTool(HostedToolChoice),
    FunctionTool(FunctionToolChoice),
}

impl From<ToolChoiceMode> for ToolChoice {
    fn from(tool: ToolChoiceMode) -> Self {
        ToolChoice::Mode(tool)
    }
}

impl From<HostedToolChoice> for ToolChoice {
    fn from(tool: HostedToolChoice) -> Self {
        ToolChoice::HostedTool(tool)
    }
}

impl From<FunctionToolChoice> for ToolChoice {
    fn from(tool: FunctionToolChoice) -> Self {
        ToolChoice::FunctionTool(tool)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn it_builds_tool_choice() {
        let tool_choice_from_mode: ToolChoice = ToolChoiceMode::Auto.into();
        let tool_choice_from_hosted_tool: ToolChoice = HostedToolChoice::new("file_search").into();
        let tool_choice_from_function_tool: ToolChoice =
            FunctionToolChoice::new("function-tool-choice-name").into();

        let tool_choice_from_mode_expected = json!("auto");
        let tool_choice_from_hosted_tool_expected = json!({
            "type": "file_search"
        });
        let tool_choice_from_function_tool_expected = json!({
            "name": "function-tool-choice-name",
            "type": "function"
        });

        assert_eq!(
            serde_json::to_value(tool_choice_from_mode).unwrap(),
            tool_choice_from_mode_expected
        );
        assert_eq!(
            serde_json::to_value(tool_choice_from_hosted_tool).unwrap(),
            tool_choice_from_hosted_tool_expected
        );
        assert_eq!(
            serde_json::to_value(tool_choice_from_function_tool).unwrap(),
            tool_choice_from_function_tool_expected
        );
    }
}
