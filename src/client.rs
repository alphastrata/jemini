use reqwest::Client;
use serde_json::{json, Value};
use url::Url;

use crate::{config::ApiKey, content::Content, errors::GeminiError};

const VERSION: &str = "v1beta";

pub struct JeminiClient {
    client: Client,
    base_url: Url,
    api_key: ApiKey,
    /*TODO:

    available models
    chat_histories: HashMap{uuid,Vec<String>}

     */
}

impl JeminiClient {
    pub fn new() -> Result<Self, GeminiError> {
        Ok(Self {
            client: Client::new(),
            base_url: Url::parse(&format!(
                //TODO: Const
                "https://generativelanguage.googleapis.com/{VERSION}/"
            ))?,
            api_key: ApiKey::from_env()?,
        })
    }

    pub(crate) fn api_key(&self) -> &str {
        &self.api_key.inner
    }
}

impl JeminiClient {
    pub async fn text_only(&self, prompt: &str) -> Result<Value, GeminiError> {
        //TODO: const these model options??
        let url = self.base_url.join("models/gemini-pro:generateContent")?;

        let contents = format!(
            r#"{{"contents": [{{"parts": [{{"text": "{}"}}]}}]}}"#,
            prompt
        );

        self.dispatch(url, contents).await
    }

    async fn dispatch(&self, mut url: Url, contents: String) -> Result<Value, GeminiError> {
        url.query_pairs_mut().append_pair("key", self.api_key());

        self.client
            .post(url)
            .header("Content-Type", "application/json")
            .body(contents)
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

    #[tokio::test]
    async fn test_text_only() {
        let client = JeminiClient::new().unwrap();
        let response = client
            .text_only("What is the meaning of life?")
            .await
            .unwrap();

        println!("{:#?}", response);
    }
}
