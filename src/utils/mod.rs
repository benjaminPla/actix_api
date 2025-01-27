use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
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

pub fn hash_password(password: &str) -> String {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Error on `hash_password`")
        .to_string();
    password_hash
}

pub fn verify_password(hashed_password: &str, password: &str) -> bool {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hashed_password).expect("Error parsing hashed password");
    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
