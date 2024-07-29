use actix_web::{HttpResponse,HttpRequest, Responder, web};
use crate::types::users::User;
use rusqlite::Connection;
use serde::Deserialize;
use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
}

pub async fn create_user(
    db:web::Data<Arc<Mutex<Connection>>>,
    req: web::Json<CreateUserRequest>
    ) -> impl Responder {
    let email = &req.email;
    let password = &req.password;

    match User::create_user(db, email, password) {
        Ok(user) => {
            HttpResponse::Created().json(user)
        },
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

pub async fn get_users(
    db:web::Data<Arc<Mutex<Connection>>>,
    req: HttpRequest
    ) -> impl Responder {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_value) = auth_header.to_str() {
            if !auth_value.is_empty() {
                return match User::get_users(db) {
                    Ok(users) => {
                        HttpResponse::Ok().json(users)
                    },
                    Err(_) => HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
    HttpResponse::Unauthorized().finish()
}
