use std::collections::HashMap;

use colored::Colorize;

use crate::{
    action_args::{ActionArgs, ActionArgsBuilder},
    action_manager::ActionManger,
    display::DisplayMessage,
    menu::{menu_logo, menu_show},
    reader::Reader,
};

type CommandHandler = fn(&mut Prompt);

pub struct Prompt {
    display: Box<dyn DisplayMessage>,
    reader: Box<dyn Reader>,
    commands: HashMap<&'static str, CommandHandler>,
    action_manager: ActionManger,
    run: bool,
    modifications: bool,
}

enum Style {
    Error,
    Success,
    Fancy,
    Default,
}

impl Prompt {
    pub fn new(display: Box<dyn DisplayMessage>, reader: Box<dyn Reader>) -> Self {
        Self {
            display,
            reader,
            commands: Self::commands_mapper(),
            run: true,
            action_manager: ActionManger::new(),
            modifications: false,
        }
    }

    fn commands_mapper() -> HashMap<&'static str, CommandHandler> {
        let mut commands: HashMap<&'static str, CommandHandler> = HashMap::new();

        commands.insert("add", Prompt::command_add);
        commands.insert("display", Prompt::command_display);
        commands.insert("remove", Prompt::command_remove);
        commands.insert("update", Prompt::command_update);
        commands.insert("complete", Prompt::command_complete);
        commands.insert("save", Prompt::command_save);
        commands.insert("exit", Prompt::command_exit);

        commands
    }

    pub fn run(&mut self) {
        let logo = menu_logo();
        let menu = menu_show();

        while self.run {
            self.print(logo.as_str(), Style::Fancy);
            self.print(menu.as_str(), Style::Default);

            self.show();

            let command = self.read_command();

            match self.process_command(command.as_str()) {
                Ok(_) => {}
                Err(err) => self.print(err, Style::Error),
            }
        }
    }

    fn show(&mut self) {
        self.print("(todo) > ", Style::Fancy);
    }

    fn read(&mut self) -> String {
        self.reader.read()
    }

    fn read_command(&mut self) -> String {
        self.read().to_lowercase()
    }

    fn process_command(&mut self, command: &str) -> Result<(), &'static str> {
        match self.commands.get(&command) {
            Some(f) => {
                f(self);
                Ok(())
            }
            None => Err("Command not found.\n\n"),
        }
    }

    fn command_add(&mut self) {
        let args = self.get_args().with_command("add").build();

        if self.wanna_proceed("You are about to add a new task. Are you sure? (yes/no): ") == true {
            if self.action_manager.process(args, &*self.display) == true {
                self.modifications = true;
                self.print("New Task add successfully.\n\n", Style::Success);
            }
        }
    }

    fn wanna_proceed(&mut self, message: &'static str) -> bool {
        loop {
            self.print(message, Style::Default);

            let input = self.read();

            match input.as_str() {
                "yes" => return true,
                "no" => {
                    self.print("Canceled.\n\n", Style::Error);
                    return false;
                }
                _ => {
                    self.print("Invalid option.\n", Style::Error);
                }
            }
        }
    }

    fn get_args(&mut self) -> ActionArgsBuilder {
        self.print("Type the task name: ", Style::Default);
        let name = self.read();

        self.print("Type the task description: ", Style::Default);
        let description = self.read();

        ActionArgsBuilder::new()
            .with_first(name)
            .with_second(description)
    }

    fn command_display(&mut self) {
        let args = ActionArgs::new("display");

        self.action_manager.process(args, &*self.display);
    }

    fn command_remove(&mut self) {
        match self.ask_id("Type the task id to delete or exit to cancel: ") {
            Some(id) => {
                let args = ActionArgsBuilder::new()
                    .with_command("remove")
                    .with_first(id)
                    .build();

                if self.wanna_proceed("Would you like to remove? (yes/no): ") == true {
                    if self.action_manager.process(args, &*self.display) == true {
                        self.print("Task removed successfully.\n\n", Style::Success);
                        self.modifications = true;
                    } else {
                        self.print("Couldn't remove the task.\n\n", Style::Error);
                    }
                }
            }
            None => (),
        }
    }

    fn ask_id(&mut self, message: &'static str) -> Option<String> {
        loop {
            self.print(message, Style::Default);

            let input = self.read();

            match input.as_str() {
                "exit" => {
                    self.print("Canceled.\n\n", Style::Error);
                    return None;
                }

                _ => match input.parse::<u32>().is_ok() {
                    true => {
                        return Some(input);
                    }
                    false => self.print("Is not a number.\n\n", Style::Error),
                },
            }
        }
    }

    fn command_update(&mut self) {
        match self.ask_id("Type the task id to update or exit to cancel: ") {
            Some(id) => {
                let args = self
                    .get_args()
                    .with_command("update")
                    .with_third(id)
                    .build();

                if self.wanna_proceed("Would you like to update? (yes/no): ") == true {
                    if self.action_manager.process(args, &*self.display) == true {
                        self.print("Task update successfully.\n\n", Style::Success);
                        self.modifications = true;
                    } else {
                        self.print("Couldn't update the task.\n\n", Style::Error);
                    }
                }
            }
            None => (),
        }
    }

    fn command_complete(&mut self) {
        match self.ask_id("Type the task id to complete or exit to cancel: ") {
            Some(id) => {
                let args = ActionArgsBuilder::new()
                    .with_command("complete")
                    .with_first(id)
                    .build();

                if self.wanna_proceed("Would you like to complete? (yes/no): ") == true {
                    if self.action_manager.process(args, &*self.display) == true {
                        self.print("Task completed successfully.\n\n", Style::Success);
                        self.modifications = true;
                    } else {
                        self.print("Couldn't complete the task.\n\n", Style::Error);
                    }
                }
            }
            None => (),
        }
    }

    fn command_save(&mut self) {
        if self.modifications == true
            && self.wanna_proceed("Would you like to overwrite? (yes/no): ") == true
        {
            let args = ActionArgs::new("save");

            self.action_manager.process(args, &*self.display);

            self.modifications = false;
        }
    }

    fn command_exit(&mut self) {
        if self.modifications == false {
            self.run = false;
        } else if self.wanna_proceed("You have modifications. Do you wann quit anyway? (yes/no): ")
            == true
        {
            self.run = false;
        }
    }

    fn print(&mut self, message: &str, style: Style) {
        let message = match style {
            Style::Error => message.red(),
            Style::Success => message.green(),
            Style::Fancy => message.cyan(),
            Style::Default => message.white(),
        };

        self.display.show(message.to_string());
    }
}
