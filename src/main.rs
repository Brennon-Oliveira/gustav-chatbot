extern crate gtk;

use gtk::prelude::*;
mod enums;
mod entities;
mod constants;
use std::sync::{Arc, Mutex};
use entities::robot::Robot;
use enums::actor::Actor;
use entities::window::{create_window, create_divisor, show_window};
use entities::chat::Chat;
use entities::prompt_input::create_prompt_input;
use entities::keyboard_listener::create_keyboard_listener;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let (window, main_container) = create_window();  

    let mut chat_container= Arc::new(Mutex::new(Chat::new(&main_container))); 

    create_divisor(&main_container);   
    
    let prompt_input = create_prompt_input(&main_container);

    let keyboard_listener = create_keyboard_listener(&prompt_input, window.clone());
    let robot = Robot::new(chat_container.clone());
    
    keyboard_listener.lock().unwrap().subscribe(Box::new(chat_container));
    keyboard_listener.lock().unwrap().subscribe(Box::new(robot));

    show_window(window, &main_container);

    gtk::main();
}