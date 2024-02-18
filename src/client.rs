use reqwest::Client;
use serde_json::{json, Value};
use url::Url;

use crate::{config::ApiKey, content::Content, errors::GeminiError};

const VERSION: &str = "v1beta";

pub struct JeminiClient {
    client: Client,
    base_url: Url,
    api_key: ApiKey,
}

impl JeminiClient {
    pub async fn new(api_key: ApiKey) -> Result<Self, GeminiError> {
        Ok(Self {
            client: Client::new(),
            base_url: Url::parse(&format!(
                "https://generativelanguage.googleapis.com/{VERSION}/"
            ))?,
            api_key,
        })
    }

    pub(crate) fn api_key(&self) -> &str {
        &self.api_key.inner
    }
}

impl JeminiClient {
    pub async fn text_only(&self, prompt: &str) -> Result<Value, GeminiError> {
        let url = self.base_url.join("models/gemini-pro:generateContent")?;

        let contents = Content::new_text_only(prompt);

        self.client
            .post(url)
            .header("Content-Type", "application/json")
            .bearer_auth(self.api_key())
            .json(&contents)
            .send()
            .await?
            .json()
            .await
            .map_err(GeminiError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
}
