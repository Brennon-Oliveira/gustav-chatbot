use gtk::prelude::*;
use gtk::Entry as GtkEntry;
use crate::constants::ENTER;


pub trait TextTypedObeserver {
    fn update(&self, text: &str);
}

pub struct KeyboardListener<'a>{
    input: &'a GtkEntry,
    observers: Vec<Box<dyn TextTypedObeserver + 'a>>
}

impl<'a> KeyboardListener<'a>{
    pub fn new(input: &'a GtkEntry)->Self{
        KeyboardListener { input, observers: Vec::new() }
    }

    pub fn subscribe(&mut self, observer: Box<dyn TextTypedObeserver + 'a>){
        self.observers.push(observer);
    }

    pub fn notify(&mut self, text: &str){
        for observer in &self.observers{
            observer.update(text);
        }
    }
}

pub fn create_keyboard_listener(input: &GtkEntry){

    input.connect_key_press_event(|_, event| {
        let key = event.keyval();
        // if(key. == ENTER){
            
        // }
        println!("Tecla pressionada: {:?}", key);
        Inhibit(false)
    });
}
