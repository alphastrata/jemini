//! For content we SEND to Gemini

use reqwest::Body;
use serde::{Deserialize, Serialize};

use crate::images::ImageData;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Content {
    // Optional user role for chat-like interactions
    pub role: Option<String>,

    // Required text prompt for all content types
    pub text: String,

    // Optional image data for image-related content
    pub image_data: Option<ImageData>,
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

    // Consider feasibility/ethical implications of image_only
    pub fn new_image_only(image_data: ImageData) {
        // ... implement based on potential API support and ethical considerations
        // Be mindful of ethical considerations and potential limitations.
        todo!()
    }

    pub fn new_chat(role: &str, message: &str, img_data: Option<ImageData>) -> Body {
        todo!()
    }
}
