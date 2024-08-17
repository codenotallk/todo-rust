use std::collections::HashMap;

use colored::Colorize;
use todo::{
    action_args::{ActionArgs, ActionArgsBuilder},
    action_manager::ActionManger,
    display::DisplayMessage,
    reader::Reader,
    repository::Repository,
};

use crate::{
    menu::{menu_logo, menu_show},
    translation::Translation,
};

type CommandHandler = fn(&mut Prompt);

pub struct Prompt {
    display: Box<dyn DisplayMessage>,
    reader: Box<dyn Reader>,
    commands: HashMap<&'static str, CommandHandler>,
    action_manager: ActionManger,
    run: bool,
    modifications: bool,
    translation: Translation,
}

enum Style {
    Error,
    Success,
    Fancy,
    Default,
}

impl Prompt {
    pub fn new(
        file: Option<&str>,
        display: Box<dyn DisplayMessage>,
        reader: Box<dyn Reader>,
        repository: Box<dyn Repository>,
    ) -> Self {
        Self {
            display,
            reader,
            commands: Self::commands_mapper(),
            run: true,
            action_manager: ActionManger::new(repository),
            modifications: false,
            translation: Translation::new(file),
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
        let menu = menu_show(&self.translation);

        while self.run {
            self.print(logo.as_str(), Style::Fancy);
            self.print(menu.as_str(), Style::Default);

            self.show();

            let command = self.read_command();

            self.process_command(command.as_str());
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

    fn process_command(&mut self, command: &str) {
        match self.commands.get(&command) {
            Some(f) => {
                f(self);
            }
            None => self.print(
                self.translation.get_message("error.command").as_str(),
                Style::Error,
            ),
        }
    }

    fn command_add(&mut self) {
        let args = self.get_args().with_command("add").build();

        if self.wanna_proceed(self.translation.get_message("question.task.add").as_str()) == true {
            if self.action_manager.process(args, &*self.display) == true {
                self.modifications = true;
                self.print(
                    self.translation.get_message("success.task.add").as_str(),
                    Style::Success,
                );
            }
        }
    }

    fn wanna_proceed(&mut self, message: &str) -> bool {
        loop {
            self.print(message, Style::Default);

            let input = self.read();

            match input.as_str() {
                "yes" => return true,
                "no" => {
                    self.print(
                        self.translation.get_message("error.canceled").as_str(),
                        Style::Error,
                    );
                    return false;
                }
                _ => {
                    self.print(
                        self.translation.get_message("error.option").as_str(),
                        Style::Error,
                    );
                }
            }
        }
    }

    fn get_args(&mut self) -> ActionArgsBuilder {
        self.print(
            self.translation.get_message("task.name").as_str(),
            Style::Default,
        );
        let name = self.read();

        self.print(
            self.translation.get_message("task.description").as_str(),
            Style::Default,
        );
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
        match self.ask_id(self.translation.get_message("id.remove").as_str()) {
            Some(id) => {
                let args = ActionArgsBuilder::new()
                    .with_command("remove")
                    .with_first(id)
                    .build();

                if self.wanna_proceed(
                    self.translation
                        .get_message("question.task.remove")
                        .as_str(),
                ) == true
                {
                    if self.action_manager.process(args, &*self.display) == true {
                        self.print(
                            self.translation.get_message("success.task.remove").as_str(),
                            Style::Success,
                        );
                        self.modifications = true;
                    } else {
                        self.print(
                            self.translation.get_message("error.task.remove").as_str(),
                            Style::Error,
                        );
                    }
                }
            }
            None => (),
        }
    }

    fn ask_id(&mut self, message: &str) -> Option<String> {
        loop {
            self.print(message, Style::Default);

            let input = self.read();

            match input.as_str() {
                "exit" => {
                    self.print(
                        self.translation.get_message("error.canceled").as_str(),
                        Style::Error,
                    );
                    return None;
                }

                _ => match input.parse::<u32>().is_ok() {
                    true => {
                        return Some(input);
                    }
                    false => self.print(
                        self.translation.get_message("error.task.id").as_str(),
                        Style::Error,
                    ),
                },
            }
        }
    }

    fn command_update(&mut self) {
        match self.ask_id(self.translation.get_message("id.update").as_str()) {
            Some(id) => {
                let args = self
                    .get_args()
                    .with_command("update")
                    .with_third(id)
                    .build();

                if self.wanna_proceed(
                    self.translation
                        .get_message("question.task.update")
                        .as_str(),
                ) == true
                {
                    if self.action_manager.process(args, &*self.display) == true {
                        self.print(
                            self.translation.get_message("success.task.update").as_str(),
                            Style::Success,
                        );
                        self.modifications = true;
                    } else {
                        self.print(
                            self.translation.get_message("error.task.update").as_str(),
                            Style::Error,
                        );
                    }
                }
            }
            None => (),
        }
    }

    fn command_complete(&mut self) {
        match self.ask_id(self.translation.get_message("id.complete").as_str()) {
            Some(id) => {
                let args = ActionArgsBuilder::new()
                    .with_command("complete")
                    .with_first(id)
                    .build();

                if self.wanna_proceed(
                    self.translation
                        .get_message("question.task.complete")
                        .as_str(),
                ) == true
                {
                    if self.action_manager.process(args, &*self.display) == true {
                        self.print(
                            self.translation
                                .get_message("success.task.complete")
                                .as_str(),
                            Style::Success,
                        );
                        self.modifications = true;
                    } else {
                        self.print(
                            self.translation.get_message("error.task.complete").as_str(),
                            Style::Error,
                        );
                    }
                }
            }
            None => (),
        }
    }

    fn command_save(&mut self) {
        if self.modifications == true
            && self.wanna_proceed(self.translation.get_message("question.overwrite").as_str())
                == true
        {
            let args = ActionArgs::new("save");

            self.action_manager.process(args, &*self.display);

            self.modifications = false;
        }
    }

    fn command_exit(&mut self) {
        if self.modifications == false {
            self.run = false;
        } else if self.wanna_proceed(
            self.translation
                .get_message("question.modification")
                .as_str(),
        ) == true
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
