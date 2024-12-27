use async_trait::async_trait;
use serde_json::json;
use crate::types::{Error, Message, Role};
use super::Provider;
use futures::{Stream, StreamExt};
use eventsource_stream::Eventsource;
use std::pin::Pin;

pub struct AnthropicProvider {
    api_key: String,
    client: reqwest::Client,
    model: String,
}

impl AnthropicProvider {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
            model: model.unwrap_or_else(|| "claude-3-5-haiku-20241022".to_string()),
        }
    }

}

#[async_trait]
impl Provider for AnthropicProvider {
    async fn complete(&self, prompt: &str) -> Result<String, Error> {
        let messages = vec![Message {
            role: Role::User,
            content: prompt.to_string(),
        }];
        self.chat(messages).await
    }

    async fn chat(&self, messages: Vec<Message>) -> Result<String, Error> {
        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&json!({
                "model": self.model,
                "messages": messages.iter().map(|m| json!({
                    "role": match m.role {
                        Role::System => "system",
                        Role::User => "user",
                        Role::Assistant => "assistant"
                    },
                    "content": m.content
                })).collect::<Vec<_>>(),
                "max_tokens": 1000,
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::ProviderError(format!(
                "Anthropic API error: {}",
                response.text().await?
            )));
        }

        let response: serde_json::Value = response.json().await?;
        Ok(response["content"][0]["text"]
            .as_str()
            .ok_or_else(|| Error::ProviderError("Invalid response format".to_string()))?
            .to_string())
    }

    async fn stream_chat(&self, messages: Vec<Message>) -> Result<Pin<Box<dyn Stream<Item = Result<String, Error>> + Send>>, Error> {
        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&json!({
                "model": self.model,
                "messages": messages.iter().map(|m| json!({
                    "role": match m.role {
                        Role::System => "system",
                        Role::User => "user",
                        Role::Assistant => "assistant"
                    },
                    "content": m.content
                })).collect::<Vec<_>>(),
                "max_tokens": 1000,
                "stream": true,
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::ProviderError(format!(
                "Anthropic API error: {}",
                response.text().await?
            )));
        }

        Ok(Box::pin(response
            .bytes_stream()
            .eventsource()
            .map(|event| {
                event
                    .map_err(|e| Error::ProviderError(e.to_string()))
                    .and_then(|event| {
                        let data = event.data;
                        if data == "[DONE]" {
                            return Ok("".to_string());
                        }
                        let json: serde_json::Value = serde_json::from_str(&data)
                            .map_err(|e| Error::ProviderError(e.to_string()))?;
                        
                        // Handle the delta format from Anthropic's streaming response
                        Ok(json["delta"]["text"]
                            .as_str()
                            .unwrap_or("")
                            .to_string())
                    })
            })
            .filter(|result| futures::future::ready(!matches!(result, Ok(s) if s.is_empty())))))
    }
}
