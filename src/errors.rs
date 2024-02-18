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
}
