use crate::error::{Error, Result};
use mockall::automock;

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::NetworkError {
            message: e.to_string(),
        }
    }
}

#[automock]
#[async_trait::async_trait]
pub trait HttpClient: Send + Sync + 'static {
    async fn get(&self, url: &str) -> Result<String>;
}

#[async_trait::async_trait]
impl HttpClient for reqwest::Client {
    async fn get(&self, url: &str) -> Result<String> {
        Ok(self
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?)
    }
}
