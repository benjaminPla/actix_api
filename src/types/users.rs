use actix_web::web;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Serialize)]
pub struct User {
    email: String,
    id: u32,
    password: String,
}

impl User {
    pub fn new(db: web::Data<Arc<Mutex<Connection>>>, email: String, password: String) -> Self {
        let conn = db.lock().unwrap();
        conn.execute(
            "INSERT INTO users (email, password) VALUES (?1, ?2)",
            params![email, password],
        )
        .unwrap();
        let id = conn.last_insert_rowid();
        let mut stmt = conn
            .prepare("SELECT id, email, password FROM users WHERE id = ?1")
            .unwrap();
        let user = stmt
            .query_row(params![id], |row| {
                Ok(Self {
                    id: row.get(0).unwrap(),
                    email: row.get(1).unwrap(),
                    password: row.get(2).unwrap(),
                })
            })
            .unwrap();
        user
    }
}
