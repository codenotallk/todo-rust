use std::{fmt::Display, sync::Mutex};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub done: bool,
}

lazy_static::lazy_static! {
    static ref DYNAMIC_ID: Mutex <u32> = Mutex::new(0);
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}. [{}] - {} - {}",
                self.id,
                if self.done == true { 'X' } else { ' ' },
                self.name,
                self.description)
    }
}

impl Task {
    pub fn new(name: &str, description: &str) -> Result<Self, &'static str> {
        if check_string(name) && check_string(description) {
            let mut dynamic_id = DYNAMIC_ID.lock().unwrap();
            *dynamic_id += 1;

            Ok(Self {
                id: *dynamic_id,
                name: name.to_owned(),
                description: description.to_owned(),
                done: false,
            })
        } else {
            Err("Cannot create a Task")
        }
    }

    pub fn update(&mut self, name: &str, description: &str) -> Result<(), &'static str> {
        if check_string(name) && check_string(description) {
            self.name = name.to_owned();
            self.description = description.to_owned();
            self.done = false;

            Ok(())
        } else {
            Err("Cannot update the Task")
        }
    }

    pub fn set_done(&mut self, done: bool) {
        self.done = done;
    }
}

fn check_string(field: &str) -> bool {
    if field.trim().is_empty() {
        return false;
    }

    true
}

pub fn set_counter(max_id: u32) {
    let mut dynamic_id = DYNAMIC_ID.lock().unwrap();
    *dynamic_id += max_id;
}
