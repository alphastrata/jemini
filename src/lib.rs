mod chat;
pub mod client;
pub(crate) mod config;
mod content;
mod errors;
mod images;
mod response_utils;
mod types;

pub use chat::*;
pub use client::JeminiClient;
pub use errors::GeminiError;
pub use images::ImageData;
pub use types::GeminiResponse;

impl GeminiResponse {
    pub fn most_recent(&self) -> Option<&str> {
        self.candidates.last().map(|candidate| {
            candidate
                .content
                .parts
                .last()
                .map(|p| p.text.as_str())
                .unwrap_or_else(|| "ERROR: No Message.")
        })
    }
}
