use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Include {
    #[serde(rename = "file_search_call.results")]
    FileSearchCallResults,
    #[serde(rename = "message.input_image.image_url")]
    MessageInputImageUrl,
    #[serde(rename = "computer_call_output.output.image_url")]
    ComputerCallOutputImageUrl,
}

impl FromStr for Include {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "file_search_call.results" => Ok(Include::FileSearchCallResults),
            "message.input_image.image_url" => Ok(Include::MessageInputImageUrl),
            "computer_call_output.output.image_url" => Ok(Include::ComputerCallOutputImageUrl),
            _ => Err(format!("Invalid include value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_str_to_include_value() {
        let values = [
            "file_search_call.results",
            "message.input_image.image_url",
            "computer_call_output.output.image_url",
        ];

        let expected = [
            Include::FileSearchCallResults,
            Include::MessageInputImageUrl,
            Include::ComputerCallOutputImageUrl,
        ];

        for (index, value) in values.iter().enumerate() {
            assert_eq!(Include::from_str(value).unwrap(), expected[index]);
        }
    }

    #[test]
    fn it_converts_include_into_json() {
        for value in [
            "file_search_call.results",
            "message.input_image.image_url",
            "computer_call_output.output.image_url",
        ]
        .iter()
        {
            let result = serde_json::to_value(Include::from_str(value).unwrap()).unwrap();

            let expected = value.to_string();

            assert_eq!(result, expected);
        }
    }
}
