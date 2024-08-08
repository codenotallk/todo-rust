use crate::task::{set_counter, Task};


pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new () -> Self {
        Self {
            tasks: Vec::new(),
        }
    }

    pub fn add (&mut self, name: &str, description: &str) -> bool {

        match Task::new (name, description) {
            Ok(task) => {
                self.tasks.push(task);
                true
            }

            Err(_) => false,
        }
    }

    pub fn get_by (&mut self, index: u32) -> Result<&Task, &'static str> {

        match self.tasks.get(index as usize) {
            Some(task) => Ok(task),
            None => Err ("Not found"),
        }
    }

    pub fn complete_by (&mut self, id: u32) -> bool {

        for task in &mut self.tasks {
            if task.id == id {
                task.set_done(true);
                return true;
            }
        }

        false
    }

    pub fn update_by (&mut self, id: u32, name: &str, description: &str) -> bool {
        for task in &mut self.tasks {
            if task.id == id {
                match task.update(name, description) {
                    Ok(_) =>  return true,
                    Err (_) => (),
                }
            }
        }

        false
    }

    pub fn remove_by (&mut self, id: u32) -> bool {

        let len = self.tasks.len();

        if let Some (position) = self.tasks.iter().position(|task| task.id == id) {
            self.tasks.remove(position);
        }

        len > self.tasks.len()
    }

    pub fn get_amount (&self) -> u32 {
        self.tasks.len() as u32
    }

    pub fn get_tasks (&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn set_tasks (&mut self, tasks: Vec<Task>) {

        self.tasks = tasks;

        if let Some (value) = self.tasks.iter().max_by_key(|task| task.id) {
            set_counter (value.id.clone() as u32);
        }
    }
}