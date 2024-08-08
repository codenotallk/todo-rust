use std::collections::HashMap;

use crate::{display::DisplayMessage, task::Task, task_manager::TaskManager};



#[derive(Debug)]
pub struct ActionArgs {
    pub command: Option<String>,
    pub first: Option<String>,
    pub second: Option<String>,
    pub third: Option<String>
}

type ActionHandler = fn (am: &mut ActionManger, args: ActionArgs, display: &dyn DisplayMessage) -> bool;

pub struct ActionManger {
    actions: HashMap<&'static str, ActionHandler>,
    manager: TaskManager,
}

impl ActionManger {
    pub fn new () -> Self {
        Self {
            actions: Self::actions_mapper (),
            manager: Self::load(),
        }
    }

    pub fn process (&mut self, args: ActionArgs, display: &dyn DisplayMessage) -> bool {

        let command = args.command.clone().unwrap();

        match self.select_action (command.as_str()) {
            Ok(f) => f (self, args, display),
            Err(_) => false,
        }
    }

    fn select_action (&mut self, action: &str) -> Result<ActionHandler, &'static str> {

        match self.actions.get(&action) {
            Some(f) => Ok(*f),
            None => Err ("Command not found."),
        }
    }

    fn actions_mapper () -> HashMap<&'static str, ActionHandler> {
        let mut actions: HashMap<&'static str, ActionHandler> = HashMap::new();

        actions.insert("add", ActionManger::add);
        actions.insert("display", ActionManger::display);
        actions.insert("remove", ActionManger::remove);
        actions.insert("update", ActionManger::update);
        actions.insert("complete", ActionManger::complete);
        actions.insert("save", ActionManger::save);

        actions
    }

    fn load () -> TaskManager {
        let mut task_manager = TaskManager::new();

        match std::fs::OpenOptions::new().write(false).create(true).read(true).open("tasks.json") {
            Ok(file) => {
                let tasks: Vec<Task> = match serde_json::from_reader(file) {
                    Ok(tasks) => tasks,
                    Err(e) => panic!("An error occurred: {}", e),
                };
        
                task_manager.set_tasks (tasks);
                task_manager
            },
            Err(_) => TaskManager::new(),
        }
    }

    fn add (&mut self, args: ActionArgs, _display: &dyn DisplayMessage) -> bool {
         
        let name = args.first.unwrap();
        let description = args.second.unwrap();

        self.manager.add(name.as_str(), description.as_str())
    }

    fn display (&mut self, _args: ActionArgs, display: &dyn DisplayMessage) -> bool {

        for i in 0..self.manager.get_amount() {

            let task = self.manager.get_by(i).unwrap();

            let message = format!("{}. [{}] - {} - {}\n",
            task.id,
            if task.done == true { 'X' } else {' '},
            task.name,
            task.description
            );

            display.show(message);
        }

        true
    }

    fn remove (&mut self, args: ActionArgs, _display: &dyn DisplayMessage) -> bool {
        let id = args.first.unwrap().parse::<u32>().unwrap();
        self.manager.remove_by(id)
    }

    fn update (&mut self, args: ActionArgs, _display: &dyn DisplayMessage) -> bool {
        let id = args.third.unwrap().parse::<u32>().unwrap();

        let name = args.first.unwrap();
        let description = args.second.unwrap();

        self.manager.update_by(id, name.as_str(), description.as_str())
        
    }

    fn complete (&mut self, args: ActionArgs, _display: &dyn DisplayMessage) -> bool {
        let id = args.first.unwrap().parse::<u32>().unwrap();
        self.manager.complete_by (id)
    }

    fn save (&mut self, _args: ActionArgs, _display: &dyn DisplayMessage) -> bool {
        
        let file = std::fs::OpenOptions::new().write(true).create(true).truncate(true).open("tasks.json");

        match file {
            Ok(file) => {
                serde_json::to_writer_pretty(file, &self.manager.get_tasks()).unwrap();
                true
            }

            Err(_) => false,
        }
    }
}