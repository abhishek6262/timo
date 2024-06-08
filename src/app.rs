use crate::storage::Storage;

pub struct App {
    pub storage: Storage,
}

impl App {
    pub fn new() -> Self {
        Self {
            storage: Storage::new(),
        }
    }
}
