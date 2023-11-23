extern crate regex;
extern crate reqwest;
use regex::Regex;
use std::cmp;

pub fn gpt_search(search_text: &str, openai_key: &str) -> Option<String> {
    // Do Google search

    let base = "https://www.google.com/search?q=".to_owned();

    let search_url = base + search_text;

    let response = reqwest::blocking::get(search_url).unwrap().text().unwrap();

    // Parse up the HTML of the search page

    let document = scraper::Html::parse_document(&response);

    // Gather the raw URLs from the HTML page

    let title_selector = scraper::Selector::parse("a").unwrap();

    let titles = document.select(&title_selector).map(|x| x.html());

    let mut results = Vec::new();

    titles
        .zip(1..101)
        .for_each(|(item, _number)| results.push(item));

    // Parse through the messy raw URLs to get more useful forms

    let regex = r#"<a\s[^>]*href=["']([^"']+)["'][^>]*>"#;

    let mut google_urls = Vec::new();

    let re = Regex::new(regex).unwrap();

    for result in results.iter() {
        match re.captures(result) {
            Some(caps) => {
                let potential = caps.get(1).unwrap().as_str();
                if &potential[0..7] == "/url?q=" {
                    google_urls.push(&potential[7..])
                }
            }
            None => println!("Found nothing..."),
        }
    }

    assert!(
        google_urls.len() > 2,
        "Did not get enough workable URLs from Google"
    );

    let mut urls = Vec::new();

    let cutoff = google_urls.len() - 3;
    for (i, url) in google_urls.iter().enumerate() {
        let scraped_url = &url[..url.find("&amp;").unwrap()];
        urls.push(scraped_url);
        if i >= cutoff {
            break;
        }
    }

    // Hit these result URLs and scrape all of the paragraph text out of them

    let mut url_results = Vec::new();

    for url in urls {
        // Note that pages are okay to be thrown away (continue), as
        // things are generally limited by ChatGPT query length anyway

        let raw_url_response = reqwest::blocking::get(url);

        if raw_url_response.is_err() {
            continue;
        }

        let url_response = raw_url_response.unwrap().text();

        if url_response.is_err() {
            continue;
        }

        let url_document = scraper::Html::parse_document(&url_response.unwrap());

        let paragraph_selector = scraper::Selector::parse("p").unwrap();

        let paragraphs = url_document
            .select(&paragraph_selector)
            .map(|x| x.text().collect::<Vec<_>>().join(" "));

        paragraphs
            .zip(1..101)
            .for_each(|(item, _number)| url_results.push(item));
    }

    // Combine these many paragraphs and add that data to the ChatGPT query

    let google_results_uncut = &url_results.join(" ");

    let google_results = &google_results_uncut[0..cmp::min(25000, google_results_uncut.len())];

    let chatgpt_query = format!(
        r#"I would like a succinct answer to the question "{search_text}"
           using the following web paragraphs as your data: "{google_results}"
           Answer me with a single sentence."#
    );

    let chatgpt_query = chatgpt_query.replace('"', "'");

    let system_query = "Bean, do you have a cord in your mouth?";

    let client = reqwest::blocking::Client::new();

    let body_json = serde_json::json!({
        "model": "gpt-4",
        "messages": [
          {
            "role": "system",
            "content": system_query
          },
          {
            "role": "user",
            "content": chatgpt_query
          }
        ]
    });

    let body = body_json.to_string();

    let req = client
        .post("https://api.openai.com/v1/chat/completions")
        .body(body)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", openai_key));

    let response_text = &req.send().unwrap().text().unwrap();

    let base_json: serde_json::Value = serde_json::from_str(response_text).ok()?;
    
    Some (
        base_json
            .as_object()?["choices"]
            .as_array()?[0]
            .as_object()?["message"]
            .as_object()?["content"]
            .as_str()?
            .to_owned()
    )
}