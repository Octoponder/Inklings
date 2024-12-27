use inklings::{
    Client,
    provider::Provider,
    types::{Message, Role},
};
use futures::StreamExt;

pub async fn test_specific_response<P: Provider + Send + Sync + 'static>(provider: P) {
    let client = Client::new(Box::new(provider));

    let messages = vec![
        Message {
            role: Role::User,
            content: "Respond with exactly and only the word 'pineapple'".to_string(),
        },
        Message {
            role: Role::Assistant,
            content: "".to_string(),
        },
    ];

    let result = client.chat(messages).await.unwrap();
    assert!(result.to_lowercase().contains("pineapple"));
    assert!(!result.to_lowercase().contains("xylophone")); // Very unlikely to contain this word
}

pub async fn test_stream_specific_response<P: Provider + Send + Sync + 'static>(provider: P) {
    let client = Client::new(Box::new(provider));

    let messages = vec![
        Message {
            role: Role::User,
            content: "Respond with exactly and only the word 'banana'".to_string(),
        },
        Message {
            role: Role::Assistant,
            content: "".to_string(),
        },
    ];

    let mut stream = client.stream_chat(messages).await.unwrap();
    let mut full_response = String::new();
    
    while let Some(Ok(chunk)) = stream.next().await {
        full_response.push_str(&chunk);
    }

    assert!(full_response.to_lowercase().contains("banana"));
    assert!(!full_response.to_lowercase().contains("xylophone")); // Very unlikely to contain this word
}
