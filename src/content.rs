//! For content we SEND to Gemini

use reqwest::Body;
use serde::{Deserialize, Serialize};

use crate::{images::ImageData, types::Role};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Content {
    // Optional user role for chat-like interactions
    pub role: Role,

    // Required text prompt for all content types
    pub text: String,

    // Optional image data for image-related content
    pub image_data: Option<ImageData>,

    pub parts: Vec<String>,
}

impl Content {
    pub fn new_text_only(prompt: &str) -> Body {
        (format!(
            r#"{{"contents": [{{"parts": [{{"text": "{}"}}]}}]}}"#,
            prompt
        ))
        .into()
    }

    pub fn new_text_with_image(prompt: &str, image_data: ImageData) -> Body {
        format!(
            r#"{{
                "contents": [
                    {{
                        "parts": [
                            {{
                                "text": "{}"
                            }},
                            {{
                                "inline_data": {{
                                    "mime_type": "{}",
                                    "data": "{}"
                                }}
                            }}
                        ]
                    }}
                ]
            }}"#,
            prompt, image_data.mime_type, image_data.data
        )
        .into()
    }
}
