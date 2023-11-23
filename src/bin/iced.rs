use gptsearch::gpt_search;
use iced::widget::{button, column, text, text_input};
use iced::{Alignment, Element, Sandbox, Settings};
use std::{env, thread};
use std::sync::{Arc, Mutex};

pub fn main() -> iced::Result {
    dotenvy::dotenv().unwrap();
    Query::run(Settings::default())
}

struct Query {
    openai_key: String,
    search_text: String,
    result_text: Arc<Mutex<String>>,
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
            result_text: Arc::new(Mutex::new("Your Answer Here".to_owned())),
        }
    }

    fn title(&self) -> String {
        String::from("GPT Search")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Submit => {
                let search = self.search_text.clone();
                let key = self.openai_key.clone();
                let arc_text = Arc::clone(&self.result_text);
                thread::spawn(move || {
                    println!("thread started");
                    let result = gpt_search(&search, &key);
                    let mut text = arc_text.lock().unwrap();
                    *text = result.unwrap();
                    println!("thread ended");
                    // TODO: tell iced gui that this has updated
                });
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
            text(self.result_text.lock().unwrap()),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}