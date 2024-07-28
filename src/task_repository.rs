use crate::{storage::Storage, task::Task};

pub struct TaskRepository<'a> {
    storage: &'a Box<dyn Storage>,
}

impl<'a> TaskRepository<'a> {
    pub fn new(storage: &'a Box<dyn Storage>) -> Self {
        Self { storage }
    }

    pub fn add(&self, content: &str, label: &Option<String>) {
        self.storage.add(content, label);
    }

    pub fn delete(&self, id: usize) {
        self.storage.delete(id);
    }

    pub fn delete_all(&self) {
        self.storage.clear();
    }

    pub fn get_all(&self, label: &Option<String>) -> Vec<Task> {
        self.storage.list(&label)
    }

    pub fn search(&self, key: &str, label: &Option<String>) -> Vec<Task> {
        let key = key.to_lowercase();

        self.storage.search(&key, &label)
    }
}
