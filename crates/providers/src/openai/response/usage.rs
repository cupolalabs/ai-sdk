use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InputTokensDetails {
    pub cached_tokens: usize,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputTokensDetails {
    pub reasoning_tokens: usize,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    pub input_tokens: usize,
    pub input_tokens_details: InputTokensDetails,
    pub output_tokens: usize,
    pub output_tokens_details: OutputTokensDetails,
    pub total_tokens: usize,
}

impl Usage {
    pub fn new(
        input_tokens: usize,
        input_tokens_details: InputTokensDetails,
        output_tokens: usize,
        output_tokens_details: OutputTokensDetails,
        total_tokens: usize,
    ) -> Self {
        Self {
            input_tokens,
            input_tokens_details,
            output_tokens,
            output_tokens_details,
            total_tokens,
        }
    }
}
