use actix_web::{HttpResponse,HttpRequest, Responder, web};
use crate::types::users::{CreateUserRequest, UserResponse};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub async fn create_user(
    db:web::Data<Arc<Mutex<Connection>>>,
    req: web::Json<CreateUserRequest>
    ) -> impl Responder {
    let user = UserResponse::new(db, req.email.clone(), req.password.clone());
    HttpResponse::Created().json(user)
}

pub async fn get_users(
    db:web::Data<Arc<Mutex<Connection>>>,
    req: HttpRequest
    ) -> impl Responder {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_value) = auth_header.to_str() {
            if !auth_value.is_empty() {
                let users = UserResponse::get_users(db);
                return HttpResponse::Ok().json(users);
            }
        }
    }
    HttpResponse::Unauthorized().finish()
}
