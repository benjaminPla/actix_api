use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

pub fn connect_db() -> Connection {
    let conn = Connection::open_in_memory().expect("Failed creating db");
    conn.execute(
        "CREATE TABLE users (
        email TEXT NOT NULL,
        is_admin BOOLEAN NOT NULL DEFAULT FALSE,
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        password TEXT NOT NULL
        );",
        [],
    )
    .expect("Failed creating table \"users\"");

    conn
}

pub fn initialize_db(db: Arc<Mutex<Connection>>) {
    let conn = db.lock().unwrap();
    conn.execute(
        "INSERT INTO users (email, is_admin, password) VALUES (?1, ?2, ?3);",
        params!["benjaminpla.dev@gmail.com", true, "12345"],
    )
    .unwrap();
}
