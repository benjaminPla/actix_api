use actix_web::{HttpResponse, Responder, web};
use crate::types::authentication::Authentication;
use crate::types::users::{User,UserWithPassword};
use crate::utils::verify_password;
use rusqlite::Connection;
use serde::Deserialize;
use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(
    db: web::Data<Arc<Mutex<Connection>>>,
    req: web::Json<LoginRequest>
) -> impl Responder {
    let email = &req.email;
    let password = &req.password;
    match UserWithPassword::get_user_by_email(db, email) {
        Ok(user_with_password) => {
            if verify_password(&user_with_password.password, password) {
                let user = User {
                    email: user_with_password.email,
                    id: user_with_password.id,
                    is_admin: user_with_password.is_admin,
                };
                let token = Authentication::generate_token(user);
                HttpResponse::Ok().json(token)
            } else {
                HttpResponse::Unauthorized().finish()
            }
        },
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}
