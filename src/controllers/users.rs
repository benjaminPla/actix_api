use actix_web::{HttpResponse, HttpRequest, Responder, web};
use crate::types::authentication::Authentication;
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
    db: web::Data<Arc<Mutex<Connection>>>,
    req: HttpRequest
) -> impl Responder {
    let auth_header = match req.headers().get("Authorization") {
        Some(header) => header,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let token = match auth_header.to_str() {
        Ok(value) => value,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    if token.is_empty() {
        return HttpResponse::Unauthorized().finish();
    }

    let claims = match Authentication::validate_token(token) {
        Ok(token_data) => token_data.claims,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };


    if !claims.user.is_admin {
        return HttpResponse::Forbidden().finish();
    }

    match User::get_users(db) {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
