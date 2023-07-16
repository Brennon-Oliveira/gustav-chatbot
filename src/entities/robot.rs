use crate::enums::actor::Actor;
use std::sync::{Arc, Mutex};
use super::{keyboard_listener::TextTypedObserver, chat::Chat};

pub struct Robot {
    chat: Arc<Mutex<Chat>>
}

impl Robot {
    pub fn new(chat: Arc<Mutex<Chat>>)->Self{
        Robot{
            chat
        }
    }

    fn answer_this(&mut self, question: &str){
        &mut self.chat.lock().unwrap().add_message(Actor::Bot, format!("You said: {}", question));
    }
}

impl TextTypedObserver for Robot {
    fn update(&mut self, text: &str) {
        self.answer_this(text);
    }
}

