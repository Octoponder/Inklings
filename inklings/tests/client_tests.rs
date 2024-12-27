use inklings::{
    Client,
    provider::MockProvider,
    types::{Message, Role, Error},
};

#[tokio::test]
async fn test_complete() {
    let expected_response = "Hello, world!".to_string();
    let provider = MockProvider::new(expected_response.clone());
    let client = Client::new(Box::new(provider));

    let result = client.complete("Test prompt").await.unwrap();
    assert_eq!(result, expected_response);
}

#[tokio::test]
async fn test_chat() {
    let expected_response = "Hello, user!".to_string();
    let provider = MockProvider::new(expected_response.clone());
    let client = Client::new(Box::new(provider));

    let messages = vec![Message {
        role: Role::User,
        content: "Hi!".to_string(),
    }];

    let result = client.chat(messages).await.unwrap();
    assert_eq!(result, expected_response);
}

#[tokio::test]
async fn test_provider_error() {
    let provider = MockProvider::with_error("Test error".to_string());
    let client = Client::new(Box::new(provider));

    let result = client.complete("Test prompt").await;
    assert!(matches!(result, Err(Error::ProviderError(_))));
}

#[tokio::test]
async fn test_chat_with_multiple_messages() {
    let expected_response = "Response".to_string();
    let provider = MockProvider::new(expected_response.clone());
    let client = Client::new(Box::new(provider));

    let messages = vec![
        Message {
            role: Role::System,
            content: "System prompt".to_string(),
        },
        Message {
            role: Role::User,
            content: "User message".to_string(),
        },
        Message {
            role: Role::Assistant,
            content: "Assistant response".to_string(),
        },
    ];

    let result = client.chat(messages).await.unwrap();
    assert_eq!(result, expected_response);
}
