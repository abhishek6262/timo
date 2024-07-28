use crate::{sqlite::sqlite_storage::SqliteStorage, storage::Storage};

pub struct App {
    pub storage: Box<dyn Storage>,
}

impl App {
    pub fn new() -> Self {
        Self {
            storage: Box::new(SqliteStorage::new()),
        }
    }
}
