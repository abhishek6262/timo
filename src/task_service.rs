use std::io::BufRead;

use crate::app::App;

pub struct TaskService<'a> {
    app: &'a App,
}

impl<'a> TaskService<'a> {
    pub fn new(app: &'a App) -> Self {
        Self {
            app,
        }
    }

    pub fn add_task(&self, text: &Vec<String>) {
        self.app.storage.write(&text.join(" "));
    }

    pub fn clear_tasks(&self) {
        self.app.storage.clear();
    }

    pub fn remove_task(&self, indexes: &Vec<usize>) {
        let texts: Vec<String> = self.app
            .storage
            .read()
            .lines()
            .enumerate()
            .filter(|predicate| !indexes.contains(&predicate.0))
            .map(|predicate| predicate.1.unwrap())
            .collect();

        self.app.storage.clear();

        if texts.len() > 0 {
            self.app.storage.write(&texts.join("\n"));
        }
    }

    pub fn search_task(&self, key: &Vec<String>) {
        let key = key.join(" ").to_lowercase();
        let mut texts = vec![];

        for line in self.app.storage.read().lines() {
            let text = line.unwrap();
            let search_text = text.to_lowercase();

            if search_text.contains(&key) {
                texts.push(text);
            }
        }

        for (index, line) in texts.iter().enumerate() {
            println!("[{index}]: {line}");
        }
    }

    pub fn list_tasks(&self) {
        for (index, line) in self.app.storage.read().lines().enumerate() {
            println!("[{index}]: {}", line.unwrap());
        }
    }
}