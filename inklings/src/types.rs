#[derive(Debug, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Clone)]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("API request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Provider error: {0}")]
    ProviderError(String),
}
