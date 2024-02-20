use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeminiError {
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),

    #[error(transparent)]
    ResponseError(#[from] serde_json::Error),

    #[error("Invalid API key")]
    InvalidApiKey,

    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    ImageError(#[from] image::ImageError),

    #[error(transparent)]
    DecodeError(#[from] base64::DecodeError),

    #[error(transparent)]
    VarError(#[from] std::env::VarError),

    #[error("StatusCode:{0}")]
    StatusCode(reqwest::StatusCode),

    #[error(transparent)]
    SystemTime(#[from] std::time::SystemTimeError),
}
