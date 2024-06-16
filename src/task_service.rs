use crate::app::App;
use crate::task_repository::TaskRepository;

pub struct TaskService<'a> {
    task_repository: TaskRepository<'a>,
}

impl<'a> TaskService<'a> {
    pub fn new(app: &'a App) -> Self {
        let task_repository = TaskRepository::new(&app.storage);

        Self { task_repository }
    }

    pub fn add_task(&self, text: &Vec<String>) {
        let content = text.join(" ");
        self.task_repository.add(&content);
    }

    pub fn clear_tasks(&self) {
        self.task_repository.delete_all();
    }

    pub fn remove_task(&self, indexes: &Vec<usize>) {
        for index in indexes {
            self.task_repository.delete(index);
        }
    }

    pub fn search_task(&self, key: &Vec<String>) {
        let key = key.join(" ");

        for task in self.task_repository.search(&key) {
            println!("[{}]: {}", task.id, task.content);
        }
    }

    pub fn list_tasks(&self) {
        for task in self.task_repository.get_all() {
            println!("[{}]: {}", task.id, task.content);
        }
    }
}
