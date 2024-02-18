use reqwest::Client;
use serde_json::{json, Value};
use url::Url;

use crate::{config::ApiKey, content::Content, errors::GeminiError};

pub struct JeminiClient {
    client: Client,
    base_url: Url,
    api_key: ApiKey,
}

impl JeminiClient {
    pub async fn new(api_key: ApiKey) -> Result<Self, GeminiError> {
        let base_url = Url::parse("https://generativelanguage.googleapis.com/v1beta/")?;
        let client = Client::new();

        Ok(Self {
            client,
            base_url,
            api_key,
        })
    }
    fn api_key(&self) -> &str {
        &self.api_key.inner
    }
}

impl JeminiClient {
    pub async fn text_only(&self, prompt: &str) -> Result<Value, GeminiError> {
        let url = self.base_url.join("models/gemini-pro:generateContent")?;

        let contents = Content::new_text_only(prompt);

        let response = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .bearer_auth(self.api_key())
            .body(contents.into())
            .send()
            .await?;

        response.json().await?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
}
