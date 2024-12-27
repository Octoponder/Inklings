mod common;

use inklings::{
    Client,
    provider::OpenAIProvider,
    types::{Message, Role},
};

#[tokio::test]
#[ignore] // Requires API key
async fn test_openai_complete() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let provider = OpenAIProvider::new(api_key, None);
    let client = Client::new(Box::new(provider));

    let result = client.complete("Say 'test'").await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore] // Requires API key
async fn test_openai_specific_response() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let provider = OpenAIProvider::new(api_key, None);
    common::test_specific_response(provider).await;
}

#[tokio::test]
#[ignore] // Requires API key
async fn test_openai_stream_specific_response() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let provider = OpenAIProvider::new(api_key, None);
    common::test_stream_specific_response(provider).await;
}

#[tokio::test]
#[ignore] // Requires API key
async fn test_openai_chat() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let provider = OpenAIProvider::new(api_key, None);
    let client = Client::new(Box::new(provider));

    let messages = vec![
        Message {
            role: Role::User,
            content: "Say 'test'".to_string(),
        },
    ];

    let result = client.chat(messages).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_openai_invalid_key() {
    let provider = OpenAIProvider::new("invalid_key".to_string(), None);
    let client = Client::new(Box::new(provider));

    let result = client.complete("Test prompt").await;
    assert!(result.is_err());
}

#[tokio::test]
#[ignore] // Requires API key
async fn test_openai_stream_chat() {
    use futures::StreamExt;
    
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let provider = OpenAIProvider::new(api_key, None);
    let client = Client::new(Box::new(provider));

    let messages = vec![Message {
        role: Role::User,
        content: "Count from 1 to 5 slowly.".to_string(),
    }];

    let mut stream = client.stream_chat(messages).await.unwrap();
    let mut received = Vec::new();
    while let Some(Ok(chunk)) = stream.next().await {
        received.push(chunk);
    }

    assert!(!received.is_empty());
}
