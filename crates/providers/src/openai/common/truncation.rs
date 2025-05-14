use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::openai::errors::ConversionError;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Truncation {
    Auto,
    Disabled,
}

impl FromStr for Truncation {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Truncation::Auto),
            "disabled" => Ok(Truncation::Disabled),
            _ => Err(ConversionError::FromStr(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_from_str() {
        assert_eq!(Truncation::from_str("auto").unwrap(), Truncation::Auto);
        assert_eq!(
            Truncation::from_str("disabled").unwrap(),
            Truncation::Disabled
        );
    }

    #[test]
    fn test_from_str_error() {
        assert!(Truncation::from_str("invalid").is_err());
    }

    // test json representation
    #[test]
    fn test_json_representation() {
        let truncation = Truncation::Auto;
        let json = serde_json::to_value(&truncation).unwrap();
        assert_eq!(json, json!("auto"));
    }

    #[test]
    fn test_json_representation_disabled() {
        let truncation = Truncation::Disabled;
        let json = serde_json::to_value(&truncation).unwrap();
        assert_eq!(json, json!("disabled"));
    }
}
