use actix_web::{HttpResponse, Responder, web};
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
