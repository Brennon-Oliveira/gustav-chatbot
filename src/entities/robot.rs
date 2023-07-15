use super::keyboard_listener::TextTypedObserver;

pub struct Robot {}

impl Robot {
    pub fn new()->Self{
        Robot{}
    }

    fn answer_this(&self, question: &str){
        println!("Question: {}", question);
    }
}

impl TextTypedObserver for Robot {
    fn update(&self, text: &str) {
        self.answer_this(text);
    }
}

