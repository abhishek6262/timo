use crate::task::Task;

pub trait Storage {
    fn new() -> impl Storage
    where
        Self: Sized;

    fn add(&self, text: &str);

    fn search(&self, text: &str) -> Vec<Task>;

    fn delete(&self, id: usize);

    fn clear(&self);

    fn list(&self) -> Vec<Task>;
}
