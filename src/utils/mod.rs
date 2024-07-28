use rusqlite::Connection;

pub fn connect() -> Connection {
    let conn = Connection::open_in_memory().expect("Failed creating db");
    conn.execute(
        "CREATE TABLE users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        email TEXT NOT NULL,
        password TEXT NOT NULL);",
        [],
    )
    .expect("Failed creating table \"users\"");

    conn
}
