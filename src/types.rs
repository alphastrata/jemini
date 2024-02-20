use serde::{Deserialize, Serialize};

use crate::images::ImageData;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Model,
    #[default]
    User,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimpleTextMsg {
    // Optional user role for chat-like interactions
    pub role: Role,

    // Required text prompt for all content types
    pub text: String,

    // Optional image data for image-related content
    pub image_data: Option<ImageData>,

    pub parts: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GeminiResponse {
    #[serde(default)]
    pub candidates: Vec<Candidate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_feedback: Option<PromptFeedback>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Candidate {
    pub content: ChatMsg,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_ratings: Option<Vec<SafetyRating>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatMsg {
    pub parts: Vec<Part>,
    #[serde(default)]
    pub role: Role,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct Part {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SafetyRating {
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PromptFeedback {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_ratings: Option<Vec<SafetyRating>>,
}
