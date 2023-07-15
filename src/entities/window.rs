use std::sync::{Arc, Mutex};
use gtk::prelude::*;
use gtk::{Box, Window, WindowType};


fn create_main_container() -> Box{
    return Box::new(gtk::Orientation::Vertical, 5);
}

pub fn create_window() -> (Arc<Mutex<Window>>, Box){
    let window: Arc<Mutex<Window>>= Arc::new(Mutex::new(Window::new(WindowType::Toplevel)));
    window.lock().unwrap().set_title("Gustav");
    window.lock().unwrap().set_default_size(720, 1080);
    window.lock().unwrap().connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    let main_container: Box = create_main_container();

    return (window, main_container);
}

pub fn create_divisor(main_container: &Box){
    let spacer: Box = Box::new(gtk::Orientation::Vertical, 1);
    spacer.set_vexpand(true);
    main_container.add(&spacer);
}

pub fn show_window(window: Arc<Mutex<Window>>, main_container: &Box){
    window.lock().unwrap().add(main_container);
    window.lock().unwrap().show_all();
}