use actix_web::{get, HttpResponse, Responder, web};
use crate::types::status::{Status,StatusOption};
use crate::types::users::User;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

#[get("/status")]
pub async fn server_status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: StatusOption::UP
    })
}

// pub async fn create_user(conn: &Connection) -> impl Responder {
pub async fn create_user(db:web::Data<Arc<Mutex<Connection>>>) -> impl Responder {
    HttpResponse::Ok().json(User::new(db,"benjaminpla.dev@gmail.com".to_string(), "12345".to_string()))
}
