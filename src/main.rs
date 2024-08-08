use std::io::{self, Write};

use display::DisplayMessage;
use prompt::Prompt;
use reader::Reader;

mod task;
mod task_manager;
mod display;
mod action_manager;
mod prompt;
mod reader;
mod menu;

#[derive(Clone)]
struct App;

impl DisplayMessage for App {
    fn show (&self, message: String) {
        print!("{}", message);
        io::stdout().flush().unwrap();
    }
}

impl Reader for App {
    fn read (&mut self) -> String {
        let mut input = String::new ();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.replace("\n", "").to_owned();
        input
    }
}

fn main() {
    let app = App;

    let mut prompt = Prompt::new(Box::new(app.clone()), Box::new(app.clone()));

    prompt.run();
}
