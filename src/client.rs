use reqwest::Client;
use serde_json::{json, Value};
use url::Url;

use crate::{
    config::ApiKey,
    content::SimpleContent,
    errors::GeminiError,
    images::ImageData,
    types::{Content, GeminiResponse},
    Chat,
};

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
    pub async fn text_only(&self, prompt: &str) -> Result<GeminiResponse, GeminiError> {
        //TODO: const these model options??
        let url = self.base_url.join("models/gemini-pro:generateContent")?;

        let contents = SimpleContent::new_text_only(prompt);
        self.dispatch(url, contents).await
    }

    async fn dispatch<B: Into<reqwest::Body>>(
        &self,
        mut url: Url,
        contents: B,
    ) -> Result<GeminiResponse, GeminiError> {
        url.query_pairs_mut().append_pair("key", self.api_key());

        self.client
            .post(url)
            .header("Content-Type", "application/json")
            .body(contents)
            .send()
            .await?
            .json::<GeminiResponse>()
            .await
            .map_err(GeminiError::from)
    }
    /// Implements https://ai.google.dev/tutorials/rest_quickstart?hl=en#text-and-image_input
    pub async fn text_and_image(
        &self,
        prompt: &str,
        image_data: ImageData,
    ) -> Result<GeminiResponse, GeminiError> {
        let url = self
            .base_url
            //TODO: const
            .join("models/gemini-pro-vision:generateContent")?;
        let contents = SimpleContent::new_text_with_image(prompt, image_data);

        self.dispatch(url, contents).await
    }

    pub async fn new_chat(&self, prompt: &str) -> Result<Chat, GeminiError> {
        //TODO: const these model options??
        let url = self.base_url.join("models/gemini-pro:generateContent")?;

        let (mut chat, contents) = Content::new_chat(prompt)?;
        let resp = self.dispatch(url, contents).await?;

        chat.append(resp);
        Ok(chat)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn text_only() {
        let client = JeminiClient::new().unwrap();
        let response = client
            .text_only("What is the meaning of life?")
            .await
            .unwrap();

        println!("{:#?}", response);
    }

    #[tokio::test]
    async fn image() {
        let client = JeminiClient::new().unwrap();
        let image_data = ImageData::from_path("test-img.png").unwrap();
        let response = client
            .text_and_image("Tell me about this image?", image_data)
            .await
            .unwrap();

        println!("{:#?}", response);
    }

    #[tokio::test]
    async fn chat() {
        let client = JeminiClient::new().unwrap();
        let response = client
            .new_chat("Write a password generation function in golang.")
            .await
            .unwrap();

        println!("{:#?}", response);
    }
}
