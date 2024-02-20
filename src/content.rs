//! For content we SEND to Gemini

use reqwest::Body;

use crate::{
    images::ImageData,
    types::{ChatMsg, Part, Role, SimpleTextMsg},
    Chat, GeminiError,
};

impl SimpleTextMsg {
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

impl ChatMsg {
    pub fn new(prompt: &str) -> Result<(Chat, Body), GeminiError> {
        let chat = Chat::default();
        let b = serde_json::to_string(&ChatMsg {
            parts: vec![Part {
                text: prompt.to_string(),
                url: None,
            }],
            role: Role::User,
        })?;

        let b = format!(
            r#"{{
                "contents": [
                    {b}
                    ]
            }}"#,
        );

        Ok((chat, b.into()))
    }
}
