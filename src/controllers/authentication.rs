use actix_web::{HttpResponse, Responder, web};
use crate::types::authentication::Authentication;
use crate::types::users::User;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(
    db:web::Data<Arc<Mutex<Connection>>>,
    req: web::Json<LoginRequest>
) -> impl Responder {
    let email = &req.email;
    let password = &req.password;

    match User::get_user(db, email, password) {
        Ok(user) => {
            let token = Authentication::generate_token(user);
            HttpResponse::Ok().json(token)
        },
        Err(_) => HttpResponse::Unauthorized().finish()
    }
}
