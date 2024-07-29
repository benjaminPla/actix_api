use actix_web::web;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize)]
pub struct UserResponse {
    email: String,
    id: u32,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
}

impl UserResponse {
    pub fn new(db: web::Data<Arc<Mutex<Connection>>>, email: String, password: String) -> Self {
        let conn = db.lock().unwrap();
        conn.execute(
            "INSERT INTO users (email, password) VALUES (?1, ?2);",
            params![email, password],
        )
        .unwrap();
        let id = conn.last_insert_rowid();
        let mut stmt = conn
            .prepare("SELECT email, id, password FROM users WHERE id = ?1;")
            .unwrap();
        let user = stmt
            .query_row(params![id], |row| {
                Ok(Self {
                    email: row.get(0).unwrap(),
                    id: row.get(1).unwrap(),
                })
            })
            .unwrap();
        user
    }

    pub fn get_users(db: web::Data<Arc<Mutex<Connection>>>) -> Vec<UserResponse> {
        let conn = db.lock().unwrap();
        let mut stmt = conn.prepare("SELECT email, id FROM users;").unwrap();
        let users_iter = stmt
            .query_map([], |row| {
                Ok(Self {
                    email: row.get(0).unwrap(),
                    id: row.get(1).unwrap(),
                })
            })
            .unwrap();
        let users: Vec<UserResponse> = users_iter.map(|user| user.unwrap()).collect();
        users
    }
}
