use crate::storage::Storage;
use crate::task::Task;
use dirs::data_local_dir;
use rusqlite::Connection;
use std::path::PathBuf;

fn create_tasks_table(connection: &Connection) {
    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id          INTEGER PRIMARY KEY,
                content     TEXT NOT NULL
            )",
            (),
        )
        .unwrap();
}

fn ensure_db_exists(connection: &Connection) {
    create_tasks_table(&connection);
}

fn get_db_path() -> PathBuf {
    let mut storage_path = data_local_dir().unwrap();

    storage_path.push(".timo.db");
    storage_path
}

pub struct SqliteStorage {
    connection: Connection,
}

impl Storage for SqliteStorage {
    fn new() -> impl Storage {
        let connection = Connection::open(&get_db_path()).unwrap();

        ensure_db_exists(&connection);

        Self { connection }
    }

    fn add(&self, text: &str) {
        self.connection
            .execute("INSERT INTO tasks (content) VALUES (?)", [text])
            .unwrap();
    }

    fn search(&self, text: &str) -> Vec<Task> {
        let mut stmt = self
            .connection
            .prepare("SELECT id, content FROM tasks WHERE content LIKE ?")
            .unwrap();

        let tasks = stmt
            .query_map([format!("%{}%", text)], |row| {
                Ok(Task::from(row.get(0)?, row.get(1).unwrap()))
            })
            .unwrap()
            .map(|task| task.unwrap())
            .collect();

        tasks
    }

    fn delete(&self, id: usize) {
        self.connection
            .execute("DELETE FROM tasks where id = ?", [id])
            .unwrap();
    }

    fn clear(&self) {
        self.connection.execute("DELETE FROM tasks", ()).unwrap();
    }

    fn list(&self) -> Vec<Task> {
        let mut stmt = self
            .connection
            .prepare("SELECT id, content FROM tasks")
            .unwrap();

        let tasks = stmt
            .query_map((), |row| Ok(Task::from(row.get(0)?, row.get(1).unwrap())))
            .unwrap()
            .map(|task| task.unwrap())
            .collect();

        tasks
    }
}
