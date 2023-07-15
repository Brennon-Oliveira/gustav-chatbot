use std::sync::{Arc, Mutex};

use gtk::prelude::*;
use gtk::{Box, Label, Separator};
use crate::Actor;

use super::keyboard_listener::TextTypedObserver;

pub struct Message{
    actor: Actor,
    content: String
}

pub struct Chat{
    messages: Vec<Message>,
    chat_container: Arc<Mutex<Box>>

}

impl Chat {
    pub fn new(main_container: &Box)-> Self{
        let chat_container = Arc::new(Mutex::new(Box::new(gtk::Orientation::Vertical, 10)));
        let chat_container_clone = Arc::clone(&chat_container);
        chat_container.lock().unwrap().set_margin_start(10);
        chat_container.lock().unwrap().set_margin_end(10);
        chat_container.lock().unwrap().set_margin_top(10);
        chat_container.lock().unwrap().set_margin_bottom(10);
        main_container.add(&*chat_container.lock().unwrap());
        return Chat { messages: Vec::new(), chat_container };
    }

    pub fn add_message(&mut self, actor: Actor, message: String){
        let message_container: Box = Box::new(gtk::Orientation::Horizontal, 10);
    
        if actor == Actor::Bot {
            message_container.set_halign(gtk::Align::Start);
        } else if actor == Actor::User {
            message_container.set_halign(gtk::Align::End);
        }
        let message_label: Label = Label::new(Some(&message));
        
        message_container.add(&message_label);
        
        println!("Message: {}", message);
        self.messages.push(Message { actor, content: message });
        {
            let mut chat_container = self.chat_container.lock().unwrap();
            chat_container.add(&message_container);
            chat_container.add(&Separator::new(gtk::Orientation::Vertical));
        }
    }

    
}

impl TextTypedObserver for Chat {
    fn update(&mut self, text: &str) {
        self.add_message(Actor::User, String::from(text));
    }
}

