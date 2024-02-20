use base64::{engine::general_purpose, Engine as _};
use bytes::Bytes;
use image::{DynamicImage, ImageFormat};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::path::{Path, PathBuf};

use crate::GeminiError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageData {
    pub mime_type: String,     // Image MIME type (e.g., "image/jpeg")
    pub data: String,          // Base64-encoded image data
    pub path: Option<PathBuf>, // Optional path to the image file

    #[serde(skip_serializing, skip_deserializing)]
    pub img: DynamicImage,
}

impl ImageData {
    pub fn from_path<P: AsRef<Path> + Into<PathBuf>>(
        path: P,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let format = ImageFormat::from_path(&path)?;
        let img: DynamicImage = image::open(&path)?;

        let mut buf: Vec<u8> = Vec::new();
        img.write_to(&mut Cursor::new(&mut buf), format)?;

        Ok(ImageData {
            mime_type: format.to_mime_type().to_string(),
            data: general_purpose::STANDARD.encode(&buf),
            path: Some(path.into()),
            img,
        })
    }

    pub fn from_bytes(b: Bytes, format: ImageFormat) -> Result<Self, GeminiError> {
        let cursor = Cursor::new(b.clone());
        let img = image::load(cursor, format)?;

        let mut buf: Vec<u8> = Vec::new();
        img.write_to(&mut Cursor::new(&mut buf), format)?;

        Ok(ImageData {
            mime_type: format.to_mime_type().to_string(),
            data: general_purpose::STANDARD.encode(&buf),
            path: None,
            img,
        })
    }

    pub fn resize_image(self, new_width: u32) -> DynamicImage {
        self.img.resize(
            new_width,
            self.img.height(),
            image::imageops::FilterType::Lanczos3,
        )
    }
}
