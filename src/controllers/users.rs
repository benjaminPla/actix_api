use actix_web::{HttpResponse, HttpRequest, Responder, web};
use crate::types::authentication::{Authentication, TokenValidationError};
use crate::types::users::User;
use rusqlite::Connection;
use serde::Deserialize;
use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    is_admin: Option<bool>,
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

   let claims = match Authentication::validate_token(&token) {
        Ok(token_data) => token_data.claims,
        Err(TokenValidationError::Expired) => return HttpResponse::Unauthorized().body("Token has expired"),
        Err(TokenValidationError::Invalid) => return HttpResponse::Unauthorized().body("Invalid token"),
        Err(TokenValidationError::Other) => return HttpResponse::InternalServerError().finish(),
    };


    if !claims.user.is_admin {
        return HttpResponse::Forbidden().finish();
    }

    match User::get_users(db) {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_user_by_id(db: web::Data<Arc<Mutex<Connection>>>, 
    req: HttpRequest,
    body: web::Json<UpdateUserRequest>,
    path: web::Path<u32>,
    ) -> impl Responder{
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

   let claims = match Authentication::validate_token(&token) {
        Ok(token_data) => token_data.claims,
        Err(TokenValidationError::Expired) => return HttpResponse::Unauthorized().body("Token has expired"),
        Err(TokenValidationError::Invalid) => return HttpResponse::Unauthorized().body("Invalid token"),
        Err(TokenValidationError::Other) => return HttpResponse::InternalServerError().finish(),
    };

    if !claims.user.is_admin {
        return HttpResponse::Forbidden().finish();
    }

    let id = path.into_inner();
    let is_admin =  match body.is_admin {
        Some(value) => value,
        None => return HttpResponse::BadRequest().finish()
    };

    match User::update_user_by_id(db, id, is_admin) {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_user_by_id(db: web::Data<Arc<Mutex<Connection>>>,
    req: HttpRequest,
    path: web::Path<u32>
        ) -> impl Responder{
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

   let claims = match Authentication::validate_token(&token) {
        Ok(token_data) => token_data.claims,
        Err(TokenValidationError::Expired) => return HttpResponse::Unauthorized().body("Token has expired"),
        Err(TokenValidationError::Invalid) => return HttpResponse::Unauthorized().body("Invalid token"),
        Err(TokenValidationError::Other) => return HttpResponse::InternalServerError().finish(),
    };

    if !claims.user.is_admin {
        return HttpResponse::Forbidden().finish();
    }

    let id = path.into_inner();

    match User::delete_user_by_id(db, id) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}
