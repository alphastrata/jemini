use dotenv::dotenv;

use crate::GeminiError;

pub(crate) struct ApiKey {
    pub(crate) inner: String,
}

impl ApiKey {
    pub(crate) fn from_env() -> Result<Self, GeminiError> {
        dotenv().ok();

        let key = ApiKey {
            inner: std::env::var("GEIMIN_API_KEY")?.to_string(),
        };

        Ok(key)
    }
}
