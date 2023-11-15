use gptsearch::*;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<_> = env::args().collect();

    let result = gpt_search(&args[1], &args[2]).await.unwrap();

    println!("GPTSearch Answer: {result}");
}