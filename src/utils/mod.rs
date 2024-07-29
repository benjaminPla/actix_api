use rusqlite::Connection;

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
