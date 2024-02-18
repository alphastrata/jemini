//! For stuff we RECEIVE from Gemini
use crate::types::*;

impl GeminiResponse {
    pub fn iter_responses(&self) -> impl Iterator<Item = &str> {
        self.candidates
            .iter()
            .flat_map(|can| can.content.parts.iter().map(|pt| pt.text.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use crate::JeminiClient;

    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn into_gemini_response() {
        // Create a new JeminiClient instance
        let client = JeminiClient::new().expect("Failed to create JeminiClient");

        // Perform a text_only call
        let response = client
            .text_only("What is the meaning of life?")
            .await
            .expect("Failed to get response from text_only call");

        let gemini_response = match serde_json::from_value::<GeminiResponse>(response.clone()) {
            Ok(gemini_response) => gemini_response,
            Err(err) => panic!(
                "Failed to parse response into GeminiResponse: {:?}\n{:#?}",
                err, response
            ),
        };

        gemini_response
            .iter_responses()
            .for_each(|resp| println!("{}", resp));
    }
}
