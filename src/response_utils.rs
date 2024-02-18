//! For stuff we RECEIVE from Gemini
use crate::types::*;

impl GeminiResponse {
    pub fn iter_responses(&self) -> impl Iterator<Item = &str> {
        self.candidates
            .iter()
            .flat_map(|can| can.content.parts.iter().map(|pt| pt.text.as_str()))
    }

    pub fn role_part_pairings(self) -> impl Iterator<Item = (Role, Part)> {
        self.candidates.into_iter().map(|can| {
            let role = can.content.role;
            let part = can.content.parts.last();
            (role, part.unwrap().to_owned()) //FIXME:
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::JeminiClient;

    

    #[tokio::test]
    async fn into_gemini_response() {
        // Create a new JeminiClient instance
        let client = JeminiClient::new().expect("Failed to create JeminiClient");

        // Perform a text_only call
        let gemini_response = client
            .text_only("What is the meaning of life?")
            .await
            .expect("Failed to get response from text_only call");

        gemini_response
            .iter_responses()
            .for_each(|resp| println!("{}", resp));
    }
}
