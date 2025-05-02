use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Effort {
    Low,
    Medium,
    High,
}

impl FromStr for Effort {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "low" => Ok(Effort::Low),
            "medium" => Ok(Effort::Medium),
            "high" => Ok(Effort::High),
            _ => Err(format!("Invalid effort value: {}", s)),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Summary {
    Auto,
    Concise,
    Detailed,
}

impl FromStr for Summary {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Summary::Auto),
            "concise" => Ok(Summary::Concise),
            "detailed" => Ok(Summary::Detailed),
            _ => Err(format!("Invalid summary value: {}", s)),
        }
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Reasoning {
    pub effort: Option<Effort>,
    pub summary: Option<Summary>,
}

impl Reasoning {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn effort(mut self, effort: &str) -> Self {
        self.effort = Some(Effort::from_str(effort).unwrap());
        self
    }

    pub fn summary(mut self, summary: &str) -> Self {
        self.summary = Some(Summary::from_str(summary).unwrap());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_reasoning_instance() {
        let result = Reasoning::new().effort("low").summary("detailed");

        assert_eq!(result.effort, Some(Effort::Low));
        assert_eq!(result.summary, Some(Summary::Detailed));
    }

    #[test]
    fn it_converts_string_into_effort() {
        let values = ["low", "medium", "high"];
        let expected = [Effort::Low, Effort::Medium, Effort::High];

        for (index, value) in values.iter().enumerate() {
            assert_eq!(Effort::from_str(value).unwrap(), expected[index]);
        }
    }

    #[test]
    fn it_reverts_when_invalid_effort_value_is_given() {
        let invalid_value = "invalid_value";

        assert_eq!(
            Effort::from_str(invalid_value),
            Err(format!("Invalid effort value: {}", invalid_value))
        );
    }

    #[test]
    fn it_converts_string_into_summary() {
        let values = ["auto", "concise", "detailed"];
        let expected = [Summary::Auto, Summary::Concise, Summary::Detailed];

        for (index, value) in values.iter().enumerate() {
            assert_eq!(Summary::from_str(value).unwrap(), expected[index]);
        }
    }

    #[test]
    fn it_reverts_when_invalid_summary_value_is_given() {
        let invalid_value = "invalid_value";

        assert_eq!(
            Summary::from_str(invalid_value),
            Err(format!("Invalid summary value: {}", invalid_value))
        );
    }
}
