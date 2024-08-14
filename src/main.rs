use std::{
    env,
    io::{self, Write},
};

use display::DisplayMessage;
use prompt::Prompt;
use reader::Reader;

mod action_args;
mod action_manager;
mod display;
mod menu;
mod prompt;
mod reader;
mod task;
mod task_manager;
mod translation;

#[derive(Clone)]
struct App;

impl DisplayMessage for App {
    fn show(&self, message: String) {
        print!("{}", message);
        io::stdout().flush().unwrap();
    }
}

impl Reader for App {
    fn read(&mut self) -> String {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.replace("\n", "").to_owned();
        input
    }
}

fn main() {
    let app = App;

    let args: Vec<String> = env::args().collect();

    let mut file = None;

    if args.len() > 1 {
        file = Some(args[1].as_str());
    }

    let mut prompt = Prompt::new(file, Box::new(app.clone()), Box::new(app.clone()));

    prompt.run();
}
