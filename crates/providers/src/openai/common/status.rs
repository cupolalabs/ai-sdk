use crate::openai::errors::ConversionError;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    InProgress,
    Completed,
    Incomplete,
    Failed,
}

impl FromStr for Status {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "in_progress" => Ok(Status::InProgress),
            "completed" => Ok(Status::Completed),
            "incomplete" => Ok(Status::Incomplete),
            "failed" => Ok(Status::Failed),
            _ => Err(ConversionError::FromStr(s.to_string())),
        }
    }
}
