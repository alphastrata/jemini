use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Content {
    // Optional user role for chat-like interactions
    pub role: Option<String>,

    // Required text prompt for all content types
    pub text: String,

    // Optional image data for image-related content
    pub image_data: Option<ImageData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageData {
    pub mime_type: String, // Image MIME type (e.g., "image/jpeg")
    pub data: String,      // Base64-encoded image data
}

impl Content {
    pub fn new_text_only(prompt: &str) -> Self {
        Self {
            text: prompt.into(),
            ..Default::default()
        }
    }

    pub fn new_text_with_image(prompt: &str, image_data: ImageData) -> Self {
        todo!()
    }

    // Consider feasibility/ethical implications of image_only
    pub fn new_image_only(image_data: ImageData) {
        // ... implement based on potential API support and ethical considerations
        // Be mindful of ethical considerations and potential limitations.
        todo!()
    }

    pub fn new_chat(role: &str, message: &str, img_data: Option<ImageData>) {
        todo!()
    }
}
