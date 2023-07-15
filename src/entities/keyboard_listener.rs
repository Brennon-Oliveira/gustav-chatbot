
use gtk::prelude::*;
use gtk::Entry as GtkEntry;
use crate::constants::ENTER;


pub trait TextTypedObserver {
    fn update(&self, text: &str);
}

pub struct KeyboardListener<'a>{
    observers: Vec<Box<dyn TextTypedObserver + 'a>>
}

impl<'a> KeyboardListener<'a>{
    pub fn new()->Self{
        let  keyboard_listener = KeyboardListener { observers: Vec::new() };

        return keyboard_listener;
    }

    pub fn subscribe(&mut self, observer: Box<dyn TextTypedObserver + 'a>){
        self.observers.push(observer);
    }

    pub fn notify(&mut self, text: &str){
        for observer in &self.observers{
            observer.update(text);
        }
    }
}
