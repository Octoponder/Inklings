use inklings::{Client, provider::OpenAIProvider, types::{Message, Role}};

#[tokio::main]
async fn main() {
    let provider = OpenAIProvider::new(
        std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set"),
        None,
    );
    let client = Client::new(Box::new(provider));

    let messages = vec![
        Message {
            role: Role::System,
            content: "You are a helpful assistant.".to_string(),
        },
        Message {
            role: Role::User,
            content: "What's the weather like?".to_string(),
        },
    ];

    let response = client.chat(messages).await.unwrap();
    println!("Response: {}", response);
}
