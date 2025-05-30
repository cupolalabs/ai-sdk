use crate::openai::errors::InputError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub const OPENAI_API_URL: &str = "https://api.openai.com/v1";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(into = "String", try_from = "&str")]
pub enum OpenAIModelId {
    Gpt4,
    Gpt4Turbo,
    Gpt4TurboPreview,
    Gpt4_0125Preview,
    Gpt4_1106Preview,
    Gpt4_0613,
    Gpt4O,
    Gpt4O2024_05_13,
    Gpt4O2024_08_06,
    Gpt4O2024_11_20,
    Gpt4ORealtimePreview,
    Gpt4ORealtimePreview2024_10_01,
    Gpt4ORealtimePreview2024_12_17,
    Gpt4OAudioPreview,
    Gpt4OAudioPreview2024_10_01,
    Gpt4OAudioPreview2024_12_17,
    Gpt4OMini,
    Gpt4OMini2024_07_18,
    Gpt4OMiniRealtimePreview,
    Gpt4OMiniRealtimePreview2024_12_17,
    Gpt4OMiniAudioPreview,
    Gpt4OMiniAudioPreview2024_12_17,
    Gpt4OMiniSearchPreview,
    Gpt4OMiniSearchPreview2025_03_11,
    Gpt4OSearchPreview,
    Gpt4OSearchPreview2025_03_11,
    Gpt4OMiniTts,
    Gpt4OTranscribe,
    Gpt4OMiniTranscribe,
    Gpt4_5Preview,
    Gpt4_5Preview2025_02_27,
    Gpt4_1,
    Gpt4_1_2025_04_14,
    Gpt4_1Mini,
    Gpt4_1Mini2025_04_14,
    Gpt4_1Nano,
    Gpt4_1Nano2025_04_14,
    Gpt3_5Turbo,
    Gpt3_5Turbo0125,
    Gpt3_5Turbo1106,
    Gpt3_5Turbo16k,
    Gpt3_5TurboInstruct,
    Gpt3_5TurboInstruct0914,
    GptImage1,
    Tts1Hd,
    Tts1Hd1106,
    TextEmbeddingAda002,
    TextEmbedding3Small,
    TextEmbedding3Large,
    ChatGpt4oLatest,
    O1Preview,
    O1Preview2024_09_12,
    O1Mini,
    O1Mini2024_09_12,
    O1Pro,
    O1Pro2025_03_19,
    O3Mini,
    O3Mini2025_01_31,
    O4Mini,
    O4Mini2025_04_16,
    OmniModerationLatest,
    OmniModeration2024_09_26,
    CodexMiniLatest,
}

impl Default for OpenAIModelId {
    fn default() -> Self {
        Self::Gpt3_5Turbo
    }
}

impl OpenAIModelId {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Gpt4 => "gpt-4",
            Self::Gpt4Turbo => "gpt-4-turbo",
            Self::Gpt4TurboPreview => "gpt-4-turbo-preview",
            Self::Gpt4_0125Preview => "gpt-4-0125-preview",
            Self::Gpt4_1106Preview => "gpt-4-1106-preview",
            Self::Gpt4_0613 => "gpt-4-0613",
            Self::Gpt4O => "gpt-4o",
            Self::Gpt4O2024_05_13 => "gpt-4o-2024-05-13",
            Self::Gpt4O2024_08_06 => "gpt-4o-2024-08-06",
            Self::Gpt4O2024_11_20 => "gpt-4o-2024-11-20",
            Self::Gpt4ORealtimePreview => "gpt-4o-realtime-preview",
            Self::Gpt4ORealtimePreview2024_10_01 => "gpt-4o-realtime-preview-2024-10-01",
            Self::Gpt4ORealtimePreview2024_12_17 => "gpt-4o-realtime-preview-2024-12-17",
            Self::Gpt4OAudioPreview => "gpt-4o-audio-preview",
            Self::Gpt4OAudioPreview2024_10_01 => "gpt-4o-audio-preview-2024-10-01",
            Self::Gpt4OAudioPreview2024_12_17 => "gpt-4o-audio-preview-2024-12-17",
            Self::Gpt4OMini => "gpt-4o-mini",
            Self::Gpt4OMini2024_07_18 => "gpt-4o-mini-2024-07-18",
            Self::Gpt4OMiniRealtimePreview => "gpt-4o-mini-realtime-preview",
            Self::Gpt4OMiniRealtimePreview2024_12_17 => "gpt-4o-mini-realtime-preview-2024-12-17",
            Self::Gpt4OMiniAudioPreview => "gpt-4o-mini-audio-preview",
            Self::Gpt4OMiniAudioPreview2024_12_17 => "gpt-4o-mini-audio-preview-2024-12-17",
            Self::Gpt4OMiniSearchPreview => "gpt-4o-mini-search-preview",
            Self::Gpt4OMiniSearchPreview2025_03_11 => "gpt-4o-mini-search-preview-2025-03-11",
            Self::Gpt4OSearchPreview => "gpt-4o-search-preview",
            Self::Gpt4OSearchPreview2025_03_11 => "gpt-4o-search-preview-2025-03-11",
            Self::Gpt4OMiniTts => "gpt-4o-mini-tts",
            Self::Gpt4OTranscribe => "gpt-4o-transcribe",
            Self::Gpt4OMiniTranscribe => "gpt-4o-mini-transcribe",
            Self::Gpt4_5Preview => "gpt-4.5-preview",
            Self::Gpt4_5Preview2025_02_27 => "gpt-4.5-preview-2025-02-27",
            Self::Gpt4_1 => "gpt-4.1",
            Self::Gpt4_1_2025_04_14 => "gpt-4.1-2025-04-14",
            Self::Gpt4_1Mini => "gpt-4.1-mini",
            Self::Gpt4_1Mini2025_04_14 => "gpt-4.1-mini-2025-04-14",
            Self::Gpt4_1Nano => "gpt-4.1-nano",
            Self::Gpt4_1Nano2025_04_14 => "gpt-4.1-nano-2025-04-14",
            Self::Gpt3_5Turbo => "gpt-3.5-turbo",
            Self::Gpt3_5Turbo0125 => "gpt-3.5-turbo-0125",
            Self::Gpt3_5Turbo1106 => "gpt-3.5-turbo-1106",
            Self::Gpt3_5Turbo16k => "gpt-3.5-turbo-16k",
            Self::Gpt3_5TurboInstruct => "gpt-3.5-turbo-instruct",
            Self::Gpt3_5TurboInstruct0914 => "gpt-3.5-turbo-instruct-0914",
            Self::GptImage1 => "gpt-image-1",
            Self::Tts1Hd => "tts-1-hd",
            Self::Tts1Hd1106 => "tts-1-hd-1106",
            Self::TextEmbeddingAda002 => "text-embedding-ada-002",
            Self::TextEmbedding3Small => "text-embedding-3-small",
            Self::TextEmbedding3Large => "text-embedding-3-large",
            Self::ChatGpt4oLatest => "chatgpt4o-latest",
            Self::O1Preview => "o1-preview",
            Self::O1Preview2024_09_12 => "o1-preview-2024-09-12",
            Self::O1Mini => "o1-mini",
            Self::O1Mini2024_09_12 => "o1-mini-2024-09-12",
            Self::O1Pro => "o1-pro",
            Self::O1Pro2025_03_19 => "o1-pro-2025-03-19",
            Self::O3Mini => "o3-mini",
            Self::O3Mini2025_01_31 => "o3-mini-2025-01-31",
            Self::O4Mini => "o4-mini",
            Self::O4Mini2025_04_16 => "o4-mini-2025-04-16",
            Self::OmniModerationLatest => "omni-moderation-latest",
            Self::OmniModeration2024_09_26 => "omni-moderation-2024-09-26",
            Self::CodexMiniLatest => "codex-mini-latest",
        }
    }
}

impl TryFrom<&str> for OpenAIModelId {
    type Error = InputError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        OpenAIModelId::from_str(value)
    }
}

impl From<OpenAIModelId> for String {
    fn from(value: OpenAIModelId) -> Self {
        value.as_str().to_string()
    }
}

impl FromStr for OpenAIModelId {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gpt-4" => Ok(Self::Gpt4),
            "gpt-4-turbo" => Ok(Self::Gpt4Turbo),
            "gpt-4-turbo-preview" => Ok(Self::Gpt4TurboPreview),
            "gpt-4-0125-preview" => Ok(Self::Gpt4_0125Preview),
            "gpt-4-1106-preview" => Ok(Self::Gpt4_1106Preview),
            "gpt-4-0613" => Ok(Self::Gpt4_0613),
            "gpt-4o" => Ok(Self::Gpt4O),
            "gpt-4o-2024-05-13" => Ok(Self::Gpt4O2024_05_13),
            "gpt-4o-2024-08-06" => Ok(Self::Gpt4O2024_08_06),
            "gpt-4o-2024-11-20" => Ok(Self::Gpt4O2024_11_20),
            "gpt-4o-realtime-preview" => Ok(Self::Gpt4ORealtimePreview),
            "gpt-4o-realtime-preview-2024-10-01" => Ok(Self::Gpt4ORealtimePreview2024_10_01),
            "gpt-4o-realtime-preview-2024-12-17" => Ok(Self::Gpt4ORealtimePreview2024_12_17),
            "gpt-4o-audio-preview" => Ok(Self::Gpt4OAudioPreview),
            "gpt-4o-audio-preview-2024-10-01" => Ok(Self::Gpt4OAudioPreview2024_10_01),
            "gpt-4o-audio-preview-2024-12-17" => Ok(Self::Gpt4OAudioPreview2024_12_17),
            "gpt-4o-mini" => Ok(Self::Gpt4OMini),
            "gpt-4o-mini-2024-07-18" => Ok(Self::Gpt4OMini2024_07_18),
            "gpt-4o-mini-realtime-preview" => Ok(Self::Gpt4OMiniRealtimePreview),
            "gpt-4o-mini-realtime-preview-2024-12-17" => {
                Ok(Self::Gpt4OMiniRealtimePreview2024_12_17)
            }
            "gpt-4o-mini-audio-preview" => Ok(Self::Gpt4OMiniAudioPreview),
            "gpt-4o-mini-audio-preview-2024-12-17" => Ok(Self::Gpt4OMiniAudioPreview2024_12_17),
            "gpt-4o-mini-search-preview" => Ok(Self::Gpt4OMiniSearchPreview),
            "gpt-4o-mini-search-preview-2025-03-11" => Ok(Self::Gpt4OMiniSearchPreview2025_03_11),
            "gpt-4o-search-preview" => Ok(Self::Gpt4OSearchPreview),
            "gpt-4o-search-preview-2025-03-11" => Ok(Self::Gpt4OSearchPreview2025_03_11),
            "gpt-4o-mini-tts" => Ok(Self::Gpt4OMiniTts),
            "gpt-4o-transcribe" => Ok(Self::Gpt4OTranscribe),
            "gpt-4o-mini-transcribe" => Ok(Self::Gpt4OMiniTranscribe),
            "gpt-4.5-preview" => Ok(Self::Gpt4_5Preview),
            "gpt-4.5-preview-2025-02-27" => Ok(Self::Gpt4_5Preview2025_02_27),
            "gpt-4.1" => Ok(Self::Gpt4_1),
            "gpt-4.1-2025-04-14" => Ok(Self::Gpt4_1_2025_04_14),
            "gpt-4.1-mini" => Ok(Self::Gpt4_1Mini),
            "gpt-4.1-mini-2025-04-14" => Ok(Self::Gpt4_1Mini2025_04_14),
            "gpt-4.1-nano" => Ok(Self::Gpt4_1Nano),
            "gpt-4.1-nano-2025-04-14" => Ok(Self::Gpt4_1Nano2025_04_14),
            "gpt-3.5-turbo" => Ok(Self::Gpt3_5Turbo),
            "gpt-3.5-turbo-0125" => Ok(Self::Gpt3_5Turbo0125),
            "gpt-3.5-turbo-1106" => Ok(Self::Gpt3_5Turbo1106),
            "gpt-3.5-turbo-16k" => Ok(Self::Gpt3_5Turbo16k),
            "gpt-3.5-turbo-instruct" => Ok(Self::Gpt3_5TurboInstruct),
            "gpt-3.5-turbo-instruct-0914" => Ok(Self::Gpt3_5TurboInstruct0914),
            "gpt-image-1" => Ok(Self::GptImage1),
            "tts-1-hd" => Ok(Self::Tts1Hd),
            "tts-1-hd-1106" => Ok(Self::Tts1Hd1106),
            "text-embedding-ada-002" => Ok(Self::TextEmbeddingAda002),
            "text-embedding-3-small" => Ok(Self::TextEmbedding3Small),
            "text-embedding-3-large" => Ok(Self::TextEmbedding3Large),
            "chatgpt4o-latest" => Ok(Self::ChatGpt4oLatest),
            "o1-preview" => Ok(Self::O1Preview),
            "o1-preview-2024-09-12" => Ok(Self::O1Preview2024_09_12),
            "o1-mini" => Ok(Self::O1Mini),
            "o1-mini-2024-09-12" => Ok(Self::O1Mini2024_09_12),
            "o1-pro" => Ok(Self::O1Pro),
            "o1-pro-2025-03-19" => Ok(Self::O1Pro2025_03_19),
            "o3-mini" => Ok(Self::O3Mini),
            "o3-mini-2025-01-31" => Ok(Self::O3Mini2025_01_31),
            "o4-mini" => Ok(Self::O4Mini),
            "o4-mini-2025-04-16" => Ok(Self::O4Mini2025_04_16),
            "omni-moderation-latest" => Ok(Self::OmniModerationLatest),
            "omni-moderation-2024-09-26" => Ok(Self::OmniModeration2024_09_26),
            "codex-mini-latest" => Ok(Self::CodexMiniLatest),
            _ => Err(InputError::InvalidModelId(s.to_string())),
        }
    }
}
