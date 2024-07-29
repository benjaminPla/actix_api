use crate::utils::hash_password;
use actix_web::web;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Serialize)]
pub struct User {
    pub email: String,
    pub id: u32,
    pub is_admin: bool,
}

#[derive(Serialize, Debug)]
pub struct UserWithPassword {
    pub email: String,
    pub id: u32,
    pub is_admin: bool,
    pub password: String,
}

impl User {
    pub fn new_admin(db: Arc<Mutex<Connection>>) {
        let password = "12345";
        let hashed_password = hash_password(password);
        let conn = db.lock().unwrap();
        conn.execute(
            "INSERT INTO users (email, is_admin, password) VALUES (?1, ?2, ?3);",
            params!["benjaminpla.dev@gmail.com", true, hashed_password],
        )
        .expect("Error on `new_admin`");
    }

    pub fn create_user(
        db: web::Data<Arc<Mutex<Connection>>>,
        email: &str,
        password: &str,
    ) -> Result<Self, rusqlite::Error> {
        let hashed_password = hash_password(password);
        let conn = db.lock().unwrap();
        conn.execute(
            "INSERT INTO users (email, password) VALUES (?1, ?2);",
            params![email, hashed_password],
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

impl UserWithPassword {
    pub fn get_user_by_email(
        db: web::Data<Arc<Mutex<Connection>>>,
        email: &str,
    ) -> Result<Self, rusqlite::Error> {
        let conn = db.lock().unwrap();
        let mut stmt =
            conn.prepare("SELECT email, id, is_admin, password FROM users WHERE email = ?1;")?;
        let user = stmt.query_row(params![email], |row| {
            Ok(Self {
                email: row.get(0)?,
                id: row.get(1)?,
                is_admin: row.get(2)?,
                password: row.get(3)?,
            })
        })?;
        Ok(user)
    }
}
