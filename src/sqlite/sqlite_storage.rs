use super::migration::run_migrations;
use crate::storage::Storage;
use crate::task::Task;
use dirs::data_local_dir;
use rusqlite::Connection;
use std::path::PathBuf;

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
        let mut connection = Connection::open(&get_db_path()).unwrap();

        run_migrations(&mut connection);

        Self { connection }
    }

    fn add(&self, text: &str, label: &Option<String>) {
        match label {
            Some(l) => self
                .connection
                .execute(
                    "INSERT INTO tasks (content, label) VALUES (?, ?)",
                    [text, l],
                )
                .unwrap(),

            None => self
                .connection
                .execute("INSERT INTO tasks (content) VALUES (?)", [text])
                .unwrap(),
        };
    }

    fn search(&self, text: &str, label: &Option<String>) -> Vec<Task> {
        let tasks = match label {
            Some(l) => {
                let mut stmt = self
                    .connection
                    .prepare(
                        "SELECT id, content, label FROM tasks WHERE content LIKE ? AND label = ?",
                    )
                    .unwrap();

                stmt.query_map([format!("%{}%", text), l.to_owned()], |row| {
                    Ok(Task {
                        id: row.get(0)?,
                        content: row.get(1)?,
                    })
                })
                .unwrap()
                .map(|task| task.unwrap())
                .collect()
            }

            None => {
                let mut stmt = self
                    .connection
                    .prepare("SELECT id, content, label FROM tasks WHERE content LIKE ?")
                    .unwrap();

                stmt.query_map([format!("%{}%", text)], |row| {
                    Ok(Task {
                        id: row.get(0)?,
                        content: row.get(1)?,
                    })
                })
                .unwrap()
                .map(|task| task.unwrap())
                .collect()
            }
        };

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

    fn list(&self, label: &Option<String>) -> Vec<Task> {
        let tasks = match label {
            Some(l) => {
                let mut stmt = self
                    .connection
                    .prepare("SELECT id, content, label FROM tasks WHERE label = ?")
                    .unwrap();

                stmt.query_map([l], |row| {
                    Ok(Task {
                        id: row.get(0)?,
                        content: row.get(1)?,
                    })
                })
                .unwrap()
                .map(|task| task.unwrap())
                .collect()
            }

            None => {
                let mut stmt = self
                    .connection
                    .prepare("SELECT id, content, label FROM tasks")
                    .unwrap();

                stmt.query_map((), |row| {
                    Ok(Task {
                        id: row.get(0)?,
                        content: row.get(1)?,
                    })
                })
                .unwrap()
                .map(|task| task.unwrap())
                .collect()
            }
        };

        tasks
    }
}
