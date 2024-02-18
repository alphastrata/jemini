use base64::{engine::general_purpose, Engine as _};
use bytes::Bytes;
use image::{DynamicImage, ImageFormat};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageData {
    pub mime_type: String,     // Image MIME type (e.g., "image/jpeg")
    pub data: String,          // Base64-encoded image data
    pub path: Option<PathBuf>, // Optional path to the image file

    #[serde(skip_serializing, skip_deserializing)]
    img: DynamicImage,
}

impl ImageData {
    pub fn from_path<P: AsRef<Path> + Into<PathBuf>>(
        path: P,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let format = ImageFormat::from_path(&path)?;
        let img: DynamicImage = image::open(&path)?;

        let mut buf: Vec<u8> = Vec::new();
        img.write_to(&mut Cursor::new(&mut buf), format)?;
        let encoded_data = general_purpose::STANDARD.encode(&buf);

        Ok(ImageData {
            mime_type: format.to_mime_type().to_string(),
            data: encoded_data,
            path: Some(path.into()),
            img,
        })
    }

    pub fn from_bytes(b: Bytes) -> Result<Self, Box<dyn std::error::Error>> {
        // Decode the base64-encoded bytes into a Vec<u8> using the general_purpose::STANDARD engine
        let decoded_data = general_purpose::STANDARD.decode(b)?;

        let cursor = Cursor::new(decoded_data.clone());

        //TODO: how do we work out the format?
        let img = image::load(cursor, ImageFormat::Jpeg)?;

        // Create the ImageData instance
        Ok(ImageData {
            mime_type: "image/jpeg".to_string(),
            data: general_purpose::STANDARD.encode(decoded_data),
            path: None,
            img,
        })
    }

    fn resize_image(self, new_width: u32) -> DynamicImage {
        self.img.resize(
            new_width,
            self.img.height(),
            image::imageops::FilterType::Lanczos3,
        )
    }
}
