use std::{
    env,
    io::{self, Write},
};

use display::DisplayMessage;
use prompt::Prompt;
use reader::Reader;
use repository::Repository;
use task::Task;
use task_mapper::TaskMapper;

mod action_args;
mod action_manager;
mod display;
mod menu;
mod prompt;
mod reader;
mod task;
mod task_manager;
mod translation;
mod repository;
mod task_mapper;

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

impl Repository for App {
    fn save (&mut self, tasks: Vec<Task>) -> bool {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("tasks.json");

        let tasks_mapper: Vec<_> = tasks.into_iter().map(|task| TaskMapper::from(task)).collect();

        match file {
            Ok(file) => {
                serde_json::to_writer_pretty(file, &tasks_mapper).unwrap();
                true
            }

            Err(_) => false,
        }
    }

    fn load (&mut self) -> Vec<Task> {
        match std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("tasks.json")
        {
            Ok(file) => {
                let tasks_mapper: Vec<TaskMapper> = match serde_json::from_reader(file) {
                    Ok(tasks) => tasks,
                    Err(e) => panic!("An error occurred: {}", e),
                };

                let tasks: Vec<Task> = tasks_mapper.into_iter().map(|task| task.into()).collect();
                tasks

            }
            Err(_) => Vec::new(),
        }
    }
}

fn main() {
    let app = App;

    let args: Vec<String> = env::args().collect();

    let mut file = None;

    if args.len() > 1 {
        file = Some(args[1].as_str());
    }

    let mut prompt = Prompt::new(file, Box::new(app.clone()), Box::new(app.clone()), Box::new(app.clone()));

    prompt.run();
}
