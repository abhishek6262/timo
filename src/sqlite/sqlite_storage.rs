use super::migration::run_migrations;
use crate::storage::Storage;
use crate::task::Task;
use dirs::data_local_dir;
use rusqlite::{params, Connection, Statement, ToSql};
use std::path::PathBuf;

fn get_db_name() -> String {
    // We'll set the DB_NAME environment only while the development
    // mode. In production, we'll use the default value.
    dotenv::var("DB_NAME").unwrap_or(".timo.db".to_string())
}

fn get_db_path() -> PathBuf {
    let mut storage_path = data_local_dir().unwrap();
    let db_name = get_db_name();

    storage_path.push(db_name);
    storage_path
}

fn fetch_tasks(stmt: &mut Statement, params: &[&dyn ToSql]) -> Vec<Task> {
    stmt.query_map(params, |row| {
        Ok(Task {
            id: row.get(0)?,
            content: row.get(1)?,
            label: row.get(2)?,
        })
    })
    .unwrap()
    .map(|task| task.unwrap())
    .collect()
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
                "SELECT id, content, label FROM tasks WHERE content LIKE ? AND label = ?",
                params![format!("%{}%", text), Box::new(l)],
            ),
            None => (
                "SELECT id, content, label FROM tasks WHERE content LIKE ?",
                params![format!("%{}%", text)],
            ),
        };

        fetch_tasks(&mut self.connection.prepare(sql).unwrap(), params)
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
                "SELECT id, content, label FROM tasks WHERE label = ?",
                params![Box::new(l)],
            ),
            None => ("SELECT id, content, label FROM tasks", params![]),
        };

        fetch_tasks(&mut self.connection.prepare(sql).unwrap(), params)
    }
}
