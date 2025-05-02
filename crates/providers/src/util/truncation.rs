use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "losercase")]
pub enum Truncation {
    Auto,
    Disabled,
}

impl FromStr for Truncation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Truncation::Auto),
            "disabled" => Ok(Truncation::Disabled),
            _ => Err(format!("Invalid truncation value: {}", s)),
        }
    }
}
