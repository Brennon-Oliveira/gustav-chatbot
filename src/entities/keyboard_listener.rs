
use std::sync::{Arc, Mutex};
use gtk::prelude::*;
use gdk::keys;
use gtk::{Entry as GtkEntry, Window, Box as BoxGtk};
use crate::constants::ENTER;


pub trait TextTypedObserver {
    fn update(&mut self, text: &str);
}

pub struct KeyboardListener{
    observers: Vec<Box<dyn TextTypedObserver>>
}

impl KeyboardListener{
    pub fn new()->Self{
        let  keyboard_listener = KeyboardListener { observers: Vec::new() };

        return keyboard_listener;
    }

    pub fn subscribe(&mut self, observer: Box<dyn TextTypedObserver>){
        self.observers.push(observer);
    }

    pub fn notify(&mut self, text: &str){
        for observer in &mut self.observers{
            observer.update(text);
        }
    }
}

pub fn create_keyboard_listener(input: &GtkEntry, window: Arc<Mutex<Window>>)->Arc<Mutex<KeyboardListener>>{
    let keyboard_listener = Arc::new(Mutex::new(KeyboardListener::new()));

    let kl_clone = Arc::clone(&keyboard_listener);
    input.connect_key_press_event(move |entry, event:&gdk::EventKey | {
        let key_code = event.keycode().unwrap_or_default();
        let key_val = entry.text();      
        if(key_code == ENTER){
            kl_clone.lock().unwrap().notify(&key_val);
            entry.set_text("");
            window.lock().unwrap().show_all();
        }  
        Inhibit(false)
        
    });

    return keyboard_listener;
}