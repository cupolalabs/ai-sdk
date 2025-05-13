use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::errors::ConversionError;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "losercase")]
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
