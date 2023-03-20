extern crate reqwest;
use chatgpt::prelude::*;
use std::env;
use chatgpt::types::CompletionResponse;
use std::io::Read;
use std::collections::HashMap;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<()> {

    let args: Vec<String> = env::args().collect();

    let search = &args[1] as &str;

    let base = "https://www.google.com/search?q=".to_owned();

    let search_url = base + search;

    let resp = reqwest::get(search_url)
        .await?
        .text()
        .await?;

    println!("{:#?}", resp);

    /*
    let args: Vec<String> = env::args().collect();

    let key = &args[1];

    let client = ChatGPT::new(key)?;

    let response: CompletionResponse = client
        .send_message("Describe in five words the Rust programming language.")
        .await?;

    println!("Response: {}", response.message().content);
    */
    Ok(())
}