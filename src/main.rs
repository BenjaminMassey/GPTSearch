use chatgpt::prelude::*;
use std::env;
use chatgpt::types::CompletionResponse;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let key = &args[1];

    let client = ChatGPT::new(key)?;

    let response: CompletionResponse = client
        .send_message("Describe in five words the Rust programming language.")
        .await?;

    println!("Response: {}", response.message().content);

    Ok(())
}