use serde::{Deserialize, Serialize};

use crate::task::Task;

#[derive(Serialize, Deserialize)]
pub struct TaskMapper {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub done: bool,
}

impl From <Task> for TaskMapper {
    fn from(value: Task) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            done: value.done,
        }
    }
}

impl Into<Task> for TaskMapper {
    fn into(self) -> Task {
        Task {
            id: self.id,
            name: self.name,
            description: self.description,
            done: self.done,
        }
    }
}