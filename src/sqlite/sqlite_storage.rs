use super::migration::run_migrations;
use crate::storage::Storage;
use crate::task::Task;
use dirs::data_local_dir;
use rusqlite::{params, Connection};
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
        let (sql, params) = match label {
            Some(l) => (
                "INSERT INTO tasks (content, label) VALUES (?, ?)",
                params![text, Box::new(l)],
            ),
            None => ("INSERT INTO tasks (content) VALUES (?)", params![text]),
        };

        self.connection.execute(sql, params).unwrap();
    }

    fn search(&self, text: &str, label: &Option<String>) -> Vec<Task> {
        let (sql, params) = match label {
            Some(l) => (
                "SELECT id, content FROM tasks WHERE content LIKE ? AND label = ?",
                params![format!("%{}%", text), Box::new(l)],
            ),
            None => (
                "SELECT id, content FROM tasks WHERE content LIKE ?",
                params![format!("%{}%", text)],
            ),
        };

        let mut stmt = self.connection.prepare(sql).unwrap();
        let tasks = stmt
            .query_map(params, |row| {
                Ok(Task {
                    id: row.get(0)?,
                    content: row.get(1)?,
                })
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

    fn list(&self, label: &Option<String>) -> Vec<Task> {
        let (sql, params) = match label {
            Some(l) => (
                "SELECT id, content FROM tasks WHERE label = ?",
                params![Box::new(l)],
            ),
            None => ("SELECT id, content FROM tasks", params![]),
        };

        let mut stmt = self.connection.prepare(sql).unwrap();
        let tasks = stmt
            .query_map(params, |row| {
                Ok(Task {
                    id: row.get(0)?,
                    content: row.get(1)?,
                })
            })
            .unwrap()
            .map(|task| task.unwrap())
            .collect();

        tasks
    }
}