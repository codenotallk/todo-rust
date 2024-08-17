use std::collections::HashMap;

use crate::{
    action_args::ActionArgs, display::DisplayMessage, repository::Repository,
    task_manager::TaskManager,
};

type ActionHandler =
    fn(am: &mut ActionManger, args: ActionArgs, display: &dyn DisplayMessage) -> bool;

pub struct ActionManger {
    actions: HashMap<&'static str, ActionHandler>,
    manager: TaskManager,
    repository: Box<dyn Repository>,
}

impl ActionManger {
    pub fn new(repository: Box<dyn Repository>) -> Self {
        let mut action_manager = Self {
            actions: Self::actions_mapper(),
            manager: TaskManager::new(),
            repository,
        };

        action_manager.load();

        action_manager
    }

    pub fn process(&mut self, args: ActionArgs, display: &dyn DisplayMessage) -> bool {
        let command = args.command.clone().unwrap();

        match self.select_action(command.as_str()) {
            Ok(f) => f(self, args, display),
            Err(_) => false,
        }
    }

    fn select_action(&mut self, action: &str) -> Result<ActionHandler, &'static str> {
        match self.actions.get(&action) {
            Some(f) => Ok(*f),
            None => Err("Command not found."),
        }
    }

    fn actions_mapper() -> HashMap<&'static str, ActionHandler> {
        let mut actions: HashMap<&'static str, ActionHandler> = HashMap::new();

        actions.insert("add", ActionManger::add);
        actions.insert("display", ActionManger::display);
        actions.insert("remove", ActionManger::remove);
        actions.insert("update", ActionManger::update);
        actions.insert("complete", ActionManger::complete);
        actions.insert("save", ActionManger::save);

        actions
    }

    fn load(&mut self) {
        let tasks = self.repository.load();

        self.manager.set_tasks(tasks);
    }

    fn add(&mut self, args: ActionArgs, _display: &dyn DisplayMessage) -> bool {
        let name = args.first.unwrap();
        let description = args.second.unwrap();

        self.manager.add(name.as_str(), description.as_str())
    }

    fn display(&mut self, _args: ActionArgs, display: &dyn DisplayMessage) -> bool {
        self.manager.get_tasks().iter().for_each(|task| {
            let message = format!("{}", task);

            display.show(message);
        });

        true
    }

    fn remove(&mut self, args: ActionArgs, _display: &dyn DisplayMessage) -> bool {
        let id = args.first.unwrap().parse::<u32>().unwrap();
        self.manager.remove_by(id)
    }

    fn update(&mut self, args: ActionArgs, _display: &dyn DisplayMessage) -> bool {
        let id = args.third.unwrap().parse::<u32>().unwrap();

        let name = args.first.unwrap();
        let description = args.second.unwrap();

        self.manager
            .update_by(id, name.as_str(), description.as_str())
    }

    fn complete(&mut self, args: ActionArgs, _display: &dyn DisplayMessage) -> bool {
        let id = args.first.unwrap().parse::<u32>().unwrap();
        self.manager.complete_by(id)
    }

    fn save(&mut self, _args: ActionArgs, _display: &dyn DisplayMessage) -> bool {
        self.repository.save(self.manager.get_tasks_store())
    }
}

#[cfg(test)]
mod tests {
    use crate::{action_args::ActionArgsBuilder, display::DisplayMessage, repository::Repository};

    use super::ActionManger;

    #[derive(Clone)]
    struct Test;

    impl Repository for Test {
        fn save(&mut self, _tasks: Vec<crate::task::Task>) -> bool {
            true
        }

        fn load(&mut self) -> Vec<crate::task::Task> {
            Vec::new()
        }
    }

    impl DisplayMessage for Test {
        fn show(&self, _message: String) {}
    }

    #[test]
    fn test_add() {
        let test = Test;

        let args = ActionArgsBuilder::new()
            .with_command("add")
            .with_first("Test".to_string())
            .with_second("Description".to_string())
            .build();

        let mut action_manager = ActionManger::new(Box::new (test.clone()));

        assert_eq!(action_manager.process(args, &test), true);
    }

    #[test]
    fn test_add_name_empty() {
        let test = Test;

        let args = ActionArgsBuilder::new()
            .with_command("add")
            .with_first("".to_string())
            .with_second("Description".to_string())
            .build();

        let mut action_manager = ActionManger::new(Box::new (test.clone()));

        assert_eq!(action_manager.process(args, &test), false);
    }

    #[test]
    fn test_add_description_empty() {
        let test = Test;

        let args = ActionArgsBuilder::new()
            .with_command("add")
            .with_first("Test".to_string())
            .with_second("".to_string())
            .build();

        let mut action_manager = ActionManger::new(Box::new (test.clone()));

        assert_eq!(action_manager.process(args, &test), false);
    }
}
