use futures::Stream;
use std::pin::Pin;
use crate::provider::Provider;
use crate::types::{Message, Error};

pub struct Client {
    provider: Box<dyn Provider + Send + Sync>,
}

impl Client {
    pub fn new(provider: Box<dyn Provider + Send + Sync>) -> Self {
        Self { provider }
    }

    pub async fn complete(&self, prompt: &str) -> Result<String, Error> {
        self.provider.complete(prompt).await
    }

    pub async fn chat(&self, messages: Vec<Message>) -> Result<String, Error> {
        self.provider.chat(messages).await
    }

    pub async fn stream_chat(&self, messages: Vec<Message>) -> Result<Pin<Box<dyn Stream<Item = Result<String, Error>> + Send>>, Error> {
        self.provider.stream_chat(messages).await
    }
}
