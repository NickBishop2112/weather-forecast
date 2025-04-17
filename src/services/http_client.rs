use mockall::automock;

#[derive(Debug)]
pub struct NetworkError {
    pub message: String,
}

impl From<reqwest::Error> for NetworkError {
    fn from(e: reqwest::Error) -> NetworkError {
        NetworkError {
            message: e.to_string(),
        }
    }
}

#[automock]
#[async_trait::async_trait]
pub trait HttpClient: Send + Sync + 'static {
    async fn get(&self, url: &str) -> Result<String, NetworkError>;
}

#[async_trait::async_trait]
impl HttpClient for reqwest::Client {
    async fn get(&self, url: &str) -> Result<String, NetworkError> {
        Ok(self.get(url)
        .send().await?
        .error_for_status()?
        .text().await?)
    }
}
