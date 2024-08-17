use crate::task::Task;

pub trait Repository {
    fn save(&mut self, tasks: Vec<Task>) -> bool;
    fn load(&mut self) -> Vec<Task>;
}
