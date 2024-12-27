use inklings::{Client, provider::{OpenAIProvider, AnthropicProvider}};

#[tokio::main]
async fn main() {
    // OpenAI example
    let openai_provider = OpenAIProvider::new(
        std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set"),
        None
    );
    let client = Client::new(Box::new(openai_provider));
    let response = client.complete("Tell me a joke").await.unwrap();
    println!("OpenAI response: {}", response);

    // Anthropic example
    let anthropic_provider = AnthropicProvider::new(
        std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY not set"),
        None
    );
    let client = Client::new(Box::new(anthropic_provider));
    let response = client.complete("Tell me a joke").await.unwrap();
    println!("Anthropic response: {}", response);
}
