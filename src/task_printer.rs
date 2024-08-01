use crate::task::Task;
use colored::Colorize;

pub struct TaskPrinter<'a> {
    task: &'a Task,
    show_labels: &'a bool,
}

impl<'a> TaskPrinter<'a> {
    pub fn new(task: &'a Task, show_labels: &'a bool) -> Self {
        Self { task, show_labels }
    }

    pub fn print(&self) {
        match (self.show_labels, &self.task.label) {
            (true, Some(label)) => {
                println!(
                    "[{}]: {}  {}",
                    self.task.id,
                    self.task.content,
                    format!("#{}", label).on_yellow().black(),
                );
            }
            _ => {
                println!("[{}]: {}", self.task.id, self.task.content);
            }
        };
    }
}
