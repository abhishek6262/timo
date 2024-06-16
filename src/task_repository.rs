use std::io::BufRead;

use crate::{storage::Storage, task::Task};

pub struct TaskRepository<'a> {
    storage: &'a Storage,
}

impl<'a> TaskRepository<'a> {
    pub fn new(storage: &'a Storage) -> Self {
        Self { storage }
    }

    pub fn add(&self, content: &str) {
        self.storage.write(content);
    }

    pub fn delete(&self, id: &usize) {
        let id = id.to_owned();
        let tasks = self.get_all().into_iter().filter(|task| task.id != id);

        self.storage.clear();

        tasks.for_each(|task| self.add(&task.content));
    }

    pub fn delete_all(&self) {
        self.storage.clear();
    }

    pub fn get_all(&self) -> Vec<Task> {
        self.storage
            .read()
            .lines()
            .enumerate()
            .map(|task| {
                let id = task.0 + 1;
                let content = task.1.unwrap();

                Task::from(id, content)
            })
            .collect()
    }

    pub fn search(&self, key: &str) -> Vec<Task> {
        let key = key.to_lowercase();

        self.get_all()
            .into_iter()
            .filter(|task| task.content.to_lowercase().contains(&key))
            .collect()
    }
}
