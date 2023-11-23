use gptsearch::gpt_search;
use iced::widget::{button, column, text, text_input};
use iced::{Alignment, Element, Sandbox, Settings};
use std::env;

pub fn main() -> iced::Result {
    dotenvy::dotenv().unwrap();
    Query::run(Settings::default())
}

struct Query {
    openai_key: String,
    search_text: String,
    result_text: String,
}

#[derive(Debug, Clone)]
enum Message {
    Submit,
    InputChanged(String),
}

impl Sandbox for Query {
    type Message = Message;

    fn new() -> Self {
        Self {
            openai_key: env::var("OPENAI_API_KEY").unwrap(),
            search_text: String::new(),
            result_text: "Your Answer Here".to_owned(),
        }
    }

    fn title(&self) -> String {
        String::from("GPT Search")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Submit => {
                let result = gpt_search(&self.search_text, &self.openai_key);
                self.result_text = match result {
                    Some(text) => text,
                    None => "There was an issue with the API call.".to_owned(),
                };
            },
            Message::InputChanged(text) => {
                self.search_text = text;
            },
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            text_input("Search text...", &self.search_text)
                .on_input(Message::InputChanged)
                .padding(10)
                .size(30),
            button("Submit").on_press(Message::Submit),
            text(&self.result_text),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}