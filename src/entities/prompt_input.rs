use gtk::prelude::*;
use gtk::{Box,Entry as GtkEntry, traits::{ContainerExt, WidgetExt}};

pub fn create_prompt_input(main_container: &Box)->GtkEntry{
    let prompt_input: GtkEntry = GtkEntry::new();
    main_container.add(&prompt_input);

    return prompt_input;
}

