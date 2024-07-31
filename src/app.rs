use crate::{sqlite::sqlite_storage::SqliteStorage, storage::Storage};
use dotenv::dotenv;

pub struct App {
    pub storage: Box<dyn Storage>,
}

impl App {
    pub fn new() -> Self {
        Self {
            storage: Box::new(SqliteStorage::new()),
        }
    }

    pub fn bootstrap(&self) {
        dotenv().ok();
    }
}
