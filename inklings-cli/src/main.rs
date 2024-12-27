use inklings::{Client, provider::{OpenAIProvider, AnthropicProvider}};
use std::env;

#[tokio::main]
async fn main() {
    let prompt = env::args()
        .nth(1)
        .unwrap_or_else(|| "Tell me a joke".to_string());

    // OpenAI example
    let openai_provider = OpenAIProvider::new(
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set"),
        None
    );
    let client = Client::new(Box::new(openai_provider));
    match client.complete(&prompt).await {
        Ok(response) => println!("OpenAI response: {}", response),
        Err(e) => eprintln!("OpenAI error: {}", e),
    }

    // Anthropic example
    let anthropic_provider = AnthropicProvider::new(
        env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY not set"),
        None
    );
    let client = Client::new(Box::new(anthropic_provider));
    match client.complete(&prompt).await {
        Ok(response) => println!("Anthropic response: {}", response),
        Err(e) => eprintln!("Anthropic error: {}", e),
    }
}
