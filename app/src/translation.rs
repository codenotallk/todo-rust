use std::{collections::HashMap, fs};

use serde_json::Value;

pub struct Translation {
    tokens: HashMap<&'static str, String>,
}

impl Translation {
    pub fn new(file: Option<&str>) -> Self {
        let mut tokens = Translation::load_default();

        if let Some(file) = file {
            tokens = Translation::load_file(file, tokens);
        }

        Self { tokens }
    }

    pub fn get_message(&self, token: &'static str) -> String {
        self.tokens.get(token).unwrap().clone()
    }

    fn load_default() -> HashMap<&'static str, String> {
        let mut tokens: HashMap<&'static str, String> = HashMap::new();

        tokens.insert("error.command", "Invalid command\n\n".to_owned());
        tokens.insert("error.option", "Invalid option.\n\n".to_owned());
        tokens.insert("error.canceled", "Canceled\n\n".to_owned());
        tokens.insert("error.task.add", "Couldn't add a new Task\n\n".to_owned());
        tokens.insert(
            "error.task.remove",
            "Couldn't remove the Task\n\n".to_owned(),
        );
        tokens.insert(
            "error.task.update",
            "Couldn't update the Task\n\n".to_owned(),
        );
        tokens.insert(
            "error.task.complete",
            "Couldn't complete the Task\n\n".to_owned(),
        );
        tokens.insert(
            "error.task.id",
            "Please. Type a valid ID number.\n\n".to_owned(),
        );

        tokens.insert(
            "question.overwrite",
            "Would you like to overwrite? (yes/no): ".to_owned(),
        );
        tokens.insert(
            "question.modification",
            "You have did some modifications. Do you want to quit anyway? (yes/no): ".to_owned(),
        );
        tokens.insert(
            "question.task.add",
            "You are about to add a new task. Are you sure? (yes/no): ".to_owned(),
        );
        tokens.insert(
            "question.task.remove",
            "Would you like to remove? (yes/no): ".to_owned(),
        );
        tokens.insert(
            "question.task.update",
            "Would you like to update? (yes/no): ".to_owned(),
        );
        tokens.insert(
            "question.task.complete",
            "Would you like to complete? (yes/no): ".to_owned(),
        );
        tokens.insert(
            "success.task.add",
            "New Task added successfully\n\n".to_owned(),
        );
        tokens.insert(
            "success.task.remove",
            "Task removed successfully\n\n".to_owned(),
        );
        tokens.insert(
            "success.task.update",
            "Task updated successfully\n\n".to_owned(),
        );
        tokens.insert(
            "success.task.complete",
            "Task completed successfully\n\n".to_owned(),
        );
        tokens.insert(
            "id.remove",
            "Type the task id to delete or exit to cancel: ".to_owned(),
        );
        tokens.insert(
            "id.update",
            "Type the task id to update or exit to cancel: ".to_owned(),
        );
        tokens.insert(
            "id.complete",
            "Type the task id to complete or exit to cancel: ".to_owned(),
        );
        tokens.insert("task.name", "Type the task name: ".to_owned());
        tokens.insert("task.description", "Type the task description: ".to_owned());
        tokens.insert("input.yes", "yes".to_owned());
        tokens.insert("input.no", "no".to_owned());
        tokens.insert("input.exit", "exit".to_owned());
        tokens.insert("menu.add", "Add      To add a new task\n".to_owned());
        tokens.insert("menu.remove", "Remove   To remove a task\n".to_owned());
        tokens.insert("menu.update", "Update   To update a task\n".to_owned());
        tokens.insert("menu.display", "Display  To display tasks\n".to_owned());
        tokens.insert("menu.complete", "Complete To complete a task\n".to_owned());
        tokens.insert("menu.save", "Save     To save the tasks\n".to_owned());
        tokens.insert("menu.exit", "Exit     To quit application\n\n".to_owned());

        tokens
    }

    fn load_file(
        file: &str,
        tokens: HashMap<&'static str, String>,
    ) -> HashMap<&'static str, String> {
        let mut __tokens = tokens;

        match fs::read_to_string(file) {
            Ok(data) => {
                let value: Value = serde_json::from_str(&data).expect("Invalid JSON");

                let map: HashMap<String, String> =
                    serde_json::from_value(value).expect("Cannot convert JSON into HashMap");

                for (key, value) in &map {
                    if let Some(__value) = __tokens.get_mut(key.as_str()) {
                        *__value = value.clone();
                    }
                }
            }
            Err(_) => todo!(),
        }

        __tokens
    }
}
