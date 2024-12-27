use futures::Stream;
use std::pin::Pin;
mod openai;
mod anthropic;
mod mock;

pub use openai::OpenAIProvider;
pub use anthropic::AnthropicProvider;
pub use mock::MockProvider;

#[async_trait::async_trait]
pub trait Provider {
    async fn complete(&self, prompt: &str) -> Result<String, crate::types::Error>;
    async fn chat(&self, messages: Vec<crate::types::Message>) -> Result<String, crate::types::Error>;
    async fn stream_chat(&self, messages: Vec<crate::types::Message>) -> Result<Pin<Box<dyn Stream<Item = Result<String, crate::types::Error>> + Send>>, crate::types::Error>;
}
