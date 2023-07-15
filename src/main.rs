extern crate gtk;
mod enums;
mod components;
mod entities;
use gtk::prelude::*;
use components::message::createMessage;
use enums::actor::Actor;
use entities::window::{create_window, create_divisor, show_window};
use entities::chat::create_chat;
use entities::prompt_input::create_prompt_input;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let (window, main_container) = create_window();  

    let chat_container = create_chat(&main_container); 
    
    create_divisor(&main_container);   
    
    create_prompt_input(&main_container);

    createMessage(&chat_container, Actor::Bot, String::from("Olá humano"));
    createMessage(&chat_container, Actor::User, String::from("Olá Robô"));    

    show_window(&window, &main_container);

    gtk::main();
}