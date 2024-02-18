use reqwest::Body;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::{
    types::{Content, Part, Role},
    GeminiError,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Chat {
    pub uuid: Uuid,
    pub start_time: Duration, // Time since epoch in millis
    pub role_part_pairings: Vec<(Role, Part)>,
}

impl Default for Chat {
    fn default() -> Self {
        Chat {
            uuid: Uuid::new_v4(),
            start_time: SystemTime::now().duration_since(UNIX_EPOCH).expect(
            "The only way this can fail is if your system for some reason cannot compute a duration since UNIX_EPOCH 
            via SystemTime::now(). If this happens, file a bug report on `std::time::SystemTime::new()` 
            and be sure to mention your OS."
        ),
            role_part_pairings: vec![],
        }
    }
}

impl Chat {
    pub fn new() -> Result<Self, GeminiError> {
        Ok(Chat {
            uuid: Uuid::new_v4(),
            start_time: SystemTime::now().duration_since(UNIX_EPOCH)?,
            role_part_pairings: vec![],
        })
    }

    pub(crate) fn append(&mut self, resp: crate::types::GeminiResponse) {
        todo!()
    }
}
impl Content {
    pub fn new_chat(prompt: &str) -> Result<(Chat, Body), GeminiError> {
        let chat = Chat::default();
        let b = serde_json::to_string(&Content {
            parts: vec![Part {
                text: prompt.to_string(),
                url: None,
            }],
            role: Role::User,
        })?
        .into();

        Ok((chat, b))
    }
}
