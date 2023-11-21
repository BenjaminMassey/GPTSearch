use gptsearch::gpt_search;
use iced::widget::{button, column, text, text_input};
use iced::{Alignment, Element, Sandbox, Settings};
use chatgpt::*;

pub fn main() -> iced::Result {
    Query::run(Settings::default())
}

struct Query {
    search_text: String,
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
            search_text: String::new(),
        }
    }

    fn title(&self) -> String {
        String::from("GPT Search")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Submit => {
                //let _ = gpt_search("", "");
                println!("{}", self.search_text);
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
            button("Submit").on_press(Message::Submit)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}