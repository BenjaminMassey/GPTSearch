extern crate reqwest;
extern crate regex;
use chatgpt::prelude::*;
use std::env;
use chatgpt::types::CompletionResponse;
use std::io::Read;
use std::collections::HashMap;
use std::cmp;
use std::error::Error;
use scraper::{Html, Selector};
use regex::Regex;

#[tokio::main]
async fn main() -> Result<()> {
    
    let args: Vec<String> = env::args().collect();

    let search = &args[1] as &str;

    let base = "https://www.google.com/search?q=".to_owned();

    let search_url = base + search;

    println!("\nAsking Google your question...");

    let response = reqwest::get(search_url)
        .await?
        .text()
        .await?;

    println!("\nCompiling search results...\n");

    let document = scraper::Html::parse_document(&response);

    let title_selector = scraper::Selector::parse("a").unwrap();

    let titles = document.select(&title_selector).map(|x| x.html());

    let mut results = Vec::new();

    titles
        .zip(1..101)
        .for_each(|(item, number)| results.push(item));

    let raw_r = r#"<a\s[^>]*href=["']([^"']+)["'][^>]*>"#;
    let r = &raw_r[..];
    
    let mut google_urls = Vec::new();

    let re = Regex::new(r).unwrap();

    for result in results.iter() {
        match re.captures(result) {
            Some(caps) => {
                let potential = caps.get(1).unwrap().as_str();
                if &potential[0..7] == "/url?q=" {
                    google_urls.push(&potential[7..])
                }
            },
            None => println!("Found nothing...")
        }
    }

    assert!(google_urls.len() > 2, "Did not get enough workable URLs from Google");

    let mut urls = Vec::new();

    let cutoff = google_urls.len() - 3;
    for (i, url) in google_urls.iter().enumerate() {
        let scraped_url = &url[..(url.find("&amp;").unwrap() as usize)];
        urls.push(scraped_url);
        if i >= cutoff {
            break;
        }
    }

    let mut url_results = Vec::new();

    for url in urls {

        let url_response = reqwest::get(url)
            .await?
            .text()
            .await?;

        let url_document = scraper::Html::parse_document(&url_response);

        let paragraph_selector = scraper::Selector::parse("p").unwrap();

        let paragraphs = url_document.select(&paragraph_selector).map(|x| x.text().collect::<Vec<_>>().join(" "));

        paragraphs
            .zip(1..101)
            .for_each(|(item, number)| url_results.push(item));

    }

    println!("Compiled {} paragraphs.", url_results.len());
    
    let google_results_uncut = &url_results.join(" ");

    let google_results = &google_results_uncut[0..cmp::min(25000, google_results_uncut.len())];

    println!("{} characters long (25000 max).", google_results.len());

    //println!("{google_results}");

    let chatgpt_query = format!(
        r#"I would like a succinct answer to the question "{search}"
           using the following web paragraphs as your data: "{google_results}"
           Answer me with a single sentence."#);

    let key = &args[2];

    let client = ChatGPT::new_with_config(
        key,
        ModelConfigurationBuilder::default()
            .engine(ChatGPTEngine::Gpt4)
            .build()
            .unwrap(),
    )?;

    println!("\nSending ChatGPT query...\n");

    let response: CompletionResponse = client
        .send_message(chatgpt_query)
        .await?;

    println!("Answer: {}", response.message().content);
    
    Ok(())
}