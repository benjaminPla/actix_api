use actix_web::web;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Serialize)]
pub struct User {
    email: String,
    id: u32,
    is_admin: bool,
}

impl User {
    pub fn new_admin(db: Arc<Mutex<Connection>>) {
        let conn = db.lock().unwrap();
        conn.execute(
            "INSERT INTO users (email, is_admin, password) VALUES (?1, ?2, ?3);",
            params!["benjaminpla.dev@gmail.com", true, "12345"],
        )
        .expect("Error on `new_admin`");
    }

    pub fn create_user(
        db: web::Data<Arc<Mutex<Connection>>>,
        email: &str,
        password: &str,
    ) -> Result<Self, rusqlite::Error> {
        let conn = db.lock().unwrap();
        conn.execute(
            "INSERT INTO users (email, password) VALUES (?1, ?2);",
            params![email, password],
        )?;
        let id = conn.last_insert_rowid();
        let mut stmt = conn.prepare("SELECT email, id, is_admin FROM users WHERE id = ?1;")?;
        let user = stmt.query_row(params![id], |row| {
            Ok(Self {
                email: row.get(0)?,
                id: row.get(1)?,
                is_admin: row.get(2)?,
            })
        })?;
        Ok(user)
    }

    pub fn get_user(
        db: web::Data<Arc<Mutex<Connection>>>,
        email: &str,
        password: &str,
    ) -> Result<Self, rusqlite::Error> {
        let conn = db.lock().unwrap();
        let mut stmt =
            conn.prepare("SELECT email, id, is_admin FROM users WHERE email = ?1 AND password = ?2;")?;
        let user = stmt.query_row(params![email, password], |row| {
            Ok(Self {
                email: row.get(0)?,
                id: row.get(1)?,
                is_admin: row.get(2)?,
            })
        })?;
        Ok(user)
    }

    pub fn get_users(db: web::Data<Arc<Mutex<Connection>>>) -> Result<Vec<User>, rusqlite::Error> {
        let conn = db.lock().unwrap();
        let mut stmt = conn.prepare("SELECT email, id, is_admin FROM users;")?;
        let users_iter = stmt.query_map([], |row| {
            Ok(Self {
                email: row.get(0)?,
                id: row.get(1)?,
                is_admin: row.get(2)?,
            })
        })?;
        let users: Vec<User> = users_iter.map(|user| user.unwrap()).collect();
        Ok(users)
    }
}
