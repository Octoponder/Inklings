use async_trait::async_trait;
use serde_json::json;
use crate::types::{Error, Message, Role};
use super::Provider;
use futures::{Stream, StreamExt};
use eventsource_stream::Eventsource;
use std::pin::Pin;

pub struct OpenAIProvider {
    api_key: String,
    client: reqwest::Client,
    model: String,
}

impl OpenAIProvider {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
            model: model.unwrap_or_else(|| "gpt-4o-mini".to_string()),
        }
    }

    fn convert_role(role: &Role) -> &'static str {
        match role {
            Role::System => "system",
            Role::User => "user",
            Role::Assistant => "assistant",
        }
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    async fn complete(&self, prompt: &str) -> Result<String, Error> {
        let messages = vec![Message {
            role: Role::User,
            content: prompt.to_string(),
        }];
        self.chat(messages).await
    }

    async fn chat(&self, messages: Vec<Message>) -> Result<String, Error> {
        let messages: Vec<serde_json::Value> = messages
            .iter()
            .map(|m| {
                json!({
                    "role": Self::convert_role(&m.role),
                    "content": m.content
                })
            })
            .collect();

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "model": self.model,
                "messages": messages,
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::ProviderError(format!(
                "OpenAI API error: {}",
                response.text().await?
            )));
        }

        let response: serde_json::Value = response.json().await?;
        Ok(response["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| Error::ProviderError("Invalid response format".to_string()))?
            .to_string())
    }

    async fn stream_chat(&self, messages: Vec<Message>) -> Result<Pin<Box<dyn Stream<Item = Result<String, Error>> + Send>>, Error> {
        let messages: Vec<serde_json::Value> = messages
            .iter()
            .map(|m| {
                json!({
                    "role": Self::convert_role(&m.role),
                    "content": m.content
                })
            })
            .collect();

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "model": self.model,
                "messages": messages,
                "stream": true,
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::ProviderError(format!(
                "OpenAI API error: {}",
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
                        Ok(json["choices"][0]["delta"]["content"]
                            .as_str()
                            .unwrap_or("")
                            .to_string())
                    })
            })
            .filter(|result| futures::future::ready(!matches!(result, Ok(s) if s.is_empty())))))
    }
}
