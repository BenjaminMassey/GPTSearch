extern crate reqwest;
extern crate regex;
use chatgpt::prelude::*;
use std::env;
use chatgpt::types::CompletionResponse;
use std::io::Read;
use std::collections::HashMap;
use std::error::Error;
use scraper::{Html, Selector};
use regex::Regex;

#[tokio::main]
async fn main() -> Result<()> {

    let args: Vec<String> = env::args().collect();

    let search = &args[1] as &str;

    let base = "https://www.google.com/search?q=".to_owned();

    let search_url = base + search;

    let response = reqwest::get(search_url)
        .await?
        .text()
        .await?;

    let document = scraper::Html::parse_document(&response);

   let title_selector = scraper::Selector::parse("a").unwrap();

    let titles = document.select(&title_selector).map(|x| x.html());

    let mut results = Vec::new();

    titles
        .zip(1..101)
        .for_each(|(item, number)| results.push(item));

    // <a\s[^>]*href=["']([^"']+)["'][^>]*>
    let qu = '"'.to_string();
    let mut raw_r = r"<a\s[^>]*href=[".to_string();
    raw_r.push_str(&qu);
    raw_r.push_str(&(r"']([^".to_string()));
    raw_r.push_str(&qu);
    raw_r.push_str(&(r"']+)[".to_string())); 
    raw_r.push_str(&qu);
    raw_r.push_str(&(r"'][^>]*>".to_string())); 
    let r = &raw_r[..];
    
    let re = Regex::new(r).unwrap();

    for result in results.iter() {
        match re.captures(result) {
            Some(caps) => println!("Found: {}", caps.get(1).unwrap().as_str()),
            None => println!("Found nothing...")
        }
    }
    

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