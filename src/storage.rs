use crate::task::Task;

pub trait Storage {
    fn new() -> impl Storage
    where
        Self: Sized;

    fn add(&self, text: &str, label: &Option<String>);

    fn search(&self, text: &str, label: &Option<String>) -> Vec<Task>;

    fn delete(&self, id: usize);

    fn clear(&self);

    fn list(&self, label: &Option<String>) -> Vec<Task>;
}
