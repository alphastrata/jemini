use reqwest::Body;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::{
    types::{Part, Role},
    Content, GeminiError,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Chat {
    pub uuid: Uuid,
    pub start_time: Duration, // Time since epoch in millis
    pub role_part_pairings: Vec<(Role, Part)>,
}

impl Chat {
    pub fn new(roles: Vec<String>, contents: Vec<Content>) -> Result<Self, GeminiError> {
        Ok(Chat {
            uuid: Uuid::new_v4(),
            start_time: SystemTime::now().duration_since(UNIX_EPOCH)?,
            role_part_pairings: vec![],
        })
    }

    fn new_chat(prompt: &str) -> Body {
        let mut contents = Content::default();
        contents.role = Role::User;
        contents.parts.push(prompt.to_string());

        let chat = Chat::new(vec!["user".to_string()], vec![contents]).unwrap();
        let payload = json!({
            "uuid": chat.uuid,
            "start_time": chat.start_time,
            "contents": chat.contents.into_iter().map(|content| {
                json!({
                    "role": content.role.unwrap_or_default(),
                    "parts": content.parts.into_iter().map(|part| {
                        json!({
                            "text": prompt,
                        })
                    }).collect::<Vec<_>>()
                })
            }).collect::<Vec<_>>()
        });

        payload.to_string().into()
    }
}
