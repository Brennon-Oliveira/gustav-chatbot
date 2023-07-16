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

    fn getResponse(question: &str)->&str{
        if(question == "Olá"){
            return "Olá, tudo bem?";
        }
        if(question == "Tudo bem"){
            return "Que bom!";
        }
        if(question == "Qual o seu nome?"){
            return "Meu nome é Gustav";
        }
        if(question == "O que você faz?"){
            return "Eu sou um robô";
        }
        return "Não entendi, desculpe";
    }

    fn answer_this(&mut self, question: &str){
        let response = String::from(Robot::getResponse(question));
        &mut self.chat.lock().unwrap().add_message(Actor::Bot, response);
    }
}

impl TextTypedObserver for Robot {
    fn update(&mut self, text: &str) {
        self.answer_this(text);
    }
}

