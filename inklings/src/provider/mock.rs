use async_trait::async_trait;
use futures::Stream;
use tokio::time::Duration;
use tokio_stream::StreamExt;
use std::pin::Pin;
use crate::types::{Error, Message};
use super::Provider;

pub struct MockProvider {
    response: String,
    error: Option<String>,
    stream_responses: Option<Vec<String>>,
}

impl MockProvider {
    pub fn new(response: String) -> Self {
        Self { 
            response,
            error: None,
            stream_responses: None,
        }
    }

    pub fn with_stream_response(responses: Vec<String>) -> Self {
        Self {
            response: responses.join(""),
            stream_responses: Some(responses),
            error: None,
        }
    }

    pub fn with_error(error: String) -> Self {
        Self {
            response: String::new(),
            error: Some(error),
            stream_responses: None,
        }
    }
}

#[async_trait]
impl Provider for MockProvider {
    async fn complete(&self, _prompt: &str) -> Result<String, Error> {
        if let Some(error) = &self.error {
            return Err(Error::ProviderError(error.clone()));
        }
        Ok(self.response.clone())
    }

    async fn chat(&self, _messages: Vec<Message>) -> Result<String, Error> {
        if let Some(error) = &self.error {
            return Err(Error::ProviderError(error.clone()));
        }
        Ok(self.response.clone())
    }

    async fn stream_chat(&self, _messages: Vec<Message>) -> Result<Pin<Box<dyn Stream<Item = Result<String, Error>> + Send>>, Error> {
        if let Some(error) = &self.error {
            return Err(Error::ProviderError(error.clone()));
        }

        let stream_responses = self.stream_responses.clone()
            .unwrap_or_else(|| vec![self.response.clone()]);

        Ok(Box::pin(tokio_stream::iter(stream_responses)
            .map(|s| Ok(s))
            .throttle(Duration::from_millis(100))))
    }
}
