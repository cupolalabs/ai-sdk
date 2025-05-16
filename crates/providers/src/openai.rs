pub mod common {
    pub mod computer_tool_call_item;
    pub mod file_search_tool_item;
    pub mod function_tool_call_item;
    pub mod output_message_item;
    pub mod reasoning;
    pub mod reasoning_item;
    pub mod service_tier;
    pub mod status;
    pub mod text;
    pub mod tool;
    pub mod tool_choice;
    pub mod truncation;
    pub mod web_search_tool_call_item;
}

pub mod errors;

pub mod response {
    pub mod incomplete_details;
    pub mod response_error;
    pub mod response_output;
    pub mod usage;
    pub mod events {
        pub mod streaming;
    }
}

pub mod request {
    pub mod include;
    pub mod input;
}

pub const OPENAI_API_URL: &str = "https://api.openai.com/v1";
