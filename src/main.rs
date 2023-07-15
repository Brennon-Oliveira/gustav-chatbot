extern crate gtk;
use std::sync::{Arc, Mutex};

use gtk::prelude::*;
mod enums;
mod components;
mod entities;
mod constants;
use components::message::createMessage;
use entities::robot::Robot;
use enums::actor::Actor;
use entities::window::{create_window, create_divisor, show_window};
use entities::chat::create_chat;
use entities::prompt_input::create_prompt_input;
use entities::keyboard_listener::KeyboardListener;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let (window, main_container) = create_window();  

    let chat_container = create_chat(&main_container); 
    
    create_divisor(&main_container);   
    
    let prompt_input = create_prompt_input(&main_container);

    let keyboard_listener = Arc::new(Mutex::new(KeyboardListener::new()));
let robot = Robot::new();
keyboard_listener.lock().unwrap().subscribe(Box::new(robot));

let kl_clone = Arc::clone(&keyboard_listener);
prompt_input.connect_key_press_event(move |_, event| {
    let key = event.keyval();
    kl_clone.lock().unwrap().notify(&key.to_string());
    println!("Tecla pressionada: {:?}", key);
    Inhibit(false)
});


    createMessage(&chat_container, Actor::Bot, String::from("Olá humano"));
    createMessage(&chat_container, Actor::User, String::from("Olá Robô"));    

    show_window(&window, &main_container);

    gtk::main();
}