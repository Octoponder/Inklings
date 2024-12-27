# Inklings Library

A unified Rust API for various Large Language Model (LLM) providers. Currently supports OpenAI and Anthropic APIs with a consistent interface. Support is planned for all common LLM providers. 

The goal of this library is to make it as easy as possible to use multiple different LLM providers while being very easy get started with. It is supposed to be easy to use on all platforms and with all common programming languages. For this reason there will be thin language bindings for both Python and JavaScript to make 

## Features

- Unified interface for multiple LLM providers
- Async/await based API

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
inklings-lib = "0.1.0"
```

## Quick Start

The library provides two main ways to interact with LLMs: simple completions and chat-based interactions.

### Simple Completion
Use `complete()` for quick, single-prompt interactions:

```rust
use inklings_lib::{Client, provider::OpenAIProvider};

#[tokio::main]
async fn main() {
    // Create a provider (OpenAI in this example)
    let provider = OpenAIProvider::new(
        std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set"),
        None, // Uses default model (gpt-4o-mini)
    );

    let client = Client::new(Box::new(provider));

    let response = client.complete("Tell me a joke").await.unwrap();
    println!("Response: {}", response);
}
```

### Chat Interface
Use `chat()` when you need more control over the conversation flow, including system prompts and message history:

```rust
use inklings_lib::{Client, provider::OpenAIProvider, types::{Message, Role}};

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
            content: "You are a helpful assistant who speaks like Shakespeare.".to_string(),
        },
        Message {
            role: Role::User,
            content: "Tell me a joke".to_string(),
        },
    ];

    let response = client.chat(messages).await.unwrap();
    println!("Response: {}", response);
}
```

The chat interface gives you more flexibility by allowing you to:
- Set system-level instructions  
- Maintain conversation context
- Control the role of each message (System, User, or Assistant)

### CLI Example

The repository includes a simple CLI example demonstrating the library usage:

```bash
# Set your API keys
export OPENAI_API_KEY=your_openai_key
export ANTHROPIC_API_KEY=your_anthropic_key

# Run with a custom prompt
cargo run -p inklings-cli -- "What is the meaning of life? Answer briefly"
```

This will query all available LLM providers and show their responses.

### Streaming Interface

```rust
use futures::StreamExt;
use inklings_lib::{Client, provider::OpenAIProvider, types::Message};

#[tokio::main]
async fn main() {
    let provider = OpenAIProvider::new(
        std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set"),
        None,
    );
    let client = Client::new(Box::new(provider));

    let messages = vec![Message {
        role: Role::User,
        content: "Tell me a story.".to_string(),
    }];

    let mut stream = client.stream_chat(messages).await.unwrap();
    while let Some(Ok(chunk)) = stream.next().await {
        print!("{}", chunk);
    }
}
```

## Supported Providers

### OpenAI
- Default model: gpt-4o-mini
- Requires OPENAI_API_KEY environment variable

### Anthropic
- Default model: claude-3-5-haiku-20241022
- Requires ANTHROPIC_API_KEY environment variable

### Mock Provider
- Useful for testing
- Can be configured to return specific responses or errors

## Usage Examples

### Chat Interface

```rust
use inklings_lib::{
    Client,
    provider::OpenAIProvider,
    types::{Message, Role},
};

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
```

### Using Different Providers

```rust
// OpenAI
let openai = OpenAIProvider::new(api_key, Some("gpt-4o".to_string()));

// Anthropic
let anthropic = AnthropicProvider::new(api_key, Some("claude-3-5-sonnet-20241022".to_string()));

// Mock (for testing)
let mock = MockProvider::new("Mocked response".to_string());
```

## Testing

The library includes several types of tests:

### Unit Tests
Run with:
```bash
cargo test
```

### Integration Tests with Real APIs
These tests are marked with `#[ignore]` and require API keys:

```bash
# Run ignored tests (requires API keys)
cargo test -- --ignored

# Run all tests including ignored ones
cargo test -- --include-ignored
```

### Mock Testing
The `MockProvider` allows testing without real API calls:

```rust
let provider = MockProvider::new("Expected response".to_string());
// or
let provider = MockProvider::with_error("Error message".to_string());
```
