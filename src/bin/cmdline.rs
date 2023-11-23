use gptsearch::*;
use std::env;

fn main() {
    dotenvy::dotenv().unwrap();

    let args: Vec<_> = env::args().collect();

    let result = gpt_search(
        &args[1], 
        &env::var("OPENAI_API_KEY").unwrap(),
    );

    println!("GPTSearch Answer: {result:?}");
}