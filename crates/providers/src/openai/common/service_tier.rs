use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::openai::errors::ConversionError;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceTier {
    Auto,
    Default,
    Flex,
}

impl FromStr for ServiceTier {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(ServiceTier::Auto),
            "default" => Ok(ServiceTier::Default),
            "flex" => Ok(ServiceTier::Flex),
            _ => Err(ConversionError::FromStr(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_str_to_service_tier() {
        let values = ["auto", "default", "flex"];
        let expected_values = [ServiceTier::Auto, ServiceTier::Default, ServiceTier::Flex];

        for (index, value) in values.iter().enumerate() {
            assert_eq!(
                ServiceTier::from_str(value).unwrap(),
                expected_values[index]
            );
        }
    }

    #[test]
    fn it_returns_error_when_wrong_service_tier_is_given() {
        let value = "wrong";

        assert_eq!(
            ServiceTier::from_str(value),
            Err(ConversionError::FromStr(value.to_string()))
        );
    }

    #[test]
    fn test_json_values() {
        let service_tier = ServiceTier::Auto;
        let json_value = serde_json::to_value(&service_tier).unwrap();
        assert_eq!(json_value, serde_json::json!("auto"));
    }
}
