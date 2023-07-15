
use gtk::prelude::*;
use gtk::Box;

pub fn create_chat(main_container: &Box) -> Box{
    let chat_container: Box = Box::new(gtk::Orientation::Vertical, 10);
    chat_container.set_margin_start(10);
    chat_container.set_margin_end(10);
    chat_container.set_margin_top(10);
    chat_container.set_margin_bottom(10);
    main_container.add(&chat_container);
    return chat_container;
}
