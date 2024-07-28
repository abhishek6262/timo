use crate::sqlite_storage::SqliteStorage;
use crate::storage::Storage;

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
