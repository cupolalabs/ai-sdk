use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceTier {
    Auto,
    Default,
    Flex,
}

impl FromStr for ServiceTier {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(ServiceTier::Auto),
            "default" => Ok(ServiceTier::Default),
            "flex" => Ok(ServiceTier::Flex),
            _ => Err(format!("Invalid service_tier value {}", s)),
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
            Err(format!("Invalid service_tier value {}", value))
        );
    }
}
