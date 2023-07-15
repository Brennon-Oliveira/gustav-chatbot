extern crate gtk;

use crate::enums::actor::Actor;

use gtk::prelude::*;
use gtk::{Box, Label, Separator};

pub fn createMessage(parent: &Box, actor: Actor, message: String){
    let message_container: Box = Box::new(gtk::Orientation::Horizontal, 10);
    let message: Label = Label::new(Some(&message));

    if actor == Actor::Bot {
        message_container.set_halign(gtk::Align::Start);
    } else if actor == Actor::User {
        message_container.set_halign(gtk::Align::End);
    }

    message_container.add(&message);
    
    parent.add(&message_container);
    parent.add(&Separator::new(gtk::Orientation::Vertical));

}
