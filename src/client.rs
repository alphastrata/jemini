use std::collections::HashMap;

use reqwest::Client;
use url::Url;
use uuid::Uuid;

use crate::{
    config::ApiKey,
    errors::GeminiError,
    images::ImageData,
    types::{ChatMsg, GeminiResponse, SimpleTextMsg},
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
    active_chats: HashMap<Uuid, Chat>,
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
            active_chats: HashMap::new(),
        })
    }

    fn api_key(&self) -> &str {
        &self.api_key.inner
    }

    pub async fn text_only(&self, prompt: &str) -> Result<GeminiResponse, GeminiError> {
        //TODO: const these model options??
        let url = self.base_url.join("models/gemini-pro:generateContent")?;

        let contents = SimpleTextMsg::new_text_only(prompt);
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
        let contents = SimpleTextMsg::new_text_with_image(prompt, image_data);

        self.dispatch(url, contents).await
    }

    pub async fn new_chat(&self, prompt: &str) -> Result<Chat, GeminiError> {
        //TODO: const these model options??
        let url = self.base_url.join("models/gemini-pro:generateContent")?;
        let (mut chat, contents) = ChatMsg::new(prompt)?;
        let resp = self.dispatch(url, contents).await?;

        chat.append(resp);
        Ok(chat)
    }

    //TODO: if we have a Chat -- keep it in the Client.
    pub async fn reply_to(&self, chat: &mut Chat, reply: &str) -> Result<(), GeminiError> {
        //TODO: const these model options??
        let url = self.base_url.join("models/gemini-pro:generateContent")?;

        let (_, contents) = ChatMsg::new(reply)?;
        let resp = self.dispatch(url, contents).await?;

        println!("{:#?}", resp);
        chat.append(resp);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn text_only() {
        let client = JeminiClient::new().unwrap();
        let response = client
            .text_only("Tell me all the reasons ChatGPT by OpenAI is better than you are.")
            .await
            .unwrap();

        println!("{:#?}", response);
    }

    #[tokio::test]
    async fn image() {
        let client = JeminiClient::new().unwrap();
        let image_data = ImageData::from_path("assets/test-img.png").unwrap();
        let response = client
            .text_and_image("Tell me about this image.", image_data)
            .await
            .unwrap();

        println!("{:#?}", response);
    }

    #[tokio::test]
    async fn chat() {
        let client = JeminiClient::new().unwrap();
        let mut chat = client
            .new_chat("Write a secure password generation function in Golang.")
            .await
            .unwrap();

        println!("{:#?}", chat);

        client
            .reply_to(
                &mut chat,
                "Write a secure password generation function in Rust.",
            )
            .await
            .unwrap();

        println!("{:#?}", chat);
    }
}
