use crate::app::App;
use crate::task_printer::TaskPrinter;
use crate::task_repository::TaskRepository;

pub struct TaskService<'a> {
    task_repository: TaskRepository<'a>,
}

impl<'a> TaskService<'a> {
    pub fn new(app: &'a App) -> Self {
        let task_repository = TaskRepository::new(&app.storage);

        Self { task_repository }
    }

    pub fn add_task(&self, text: &Vec<String>, label: &Option<String>) {
        let content = text.join(" ");
        self.task_repository.add(&content, &label);
    }

    pub fn clear_tasks(&self, _: &bool) {
        self.task_repository.delete_all();
    }

    pub fn remove_task(&self, ids: &Vec<usize>) {
        for id in ids {
            self.task_repository.delete(id.to_owned());
        }
    }

    pub fn search_task(&self, key: &Vec<String>, label: &Option<String>, show_labels: &bool) {
        let key = key.join(" ");

        for task in self.task_repository.search(&key, &label) {
            TaskPrinter::new(&task, &show_labels).print();
        }
    }

    pub fn list_tasks(&self, label: &Option<String>, show_labels: &bool) {
        for task in self.task_repository.get_all(&label) {
            TaskPrinter::new(&task, &show_labels).print();
        }
    }
}
