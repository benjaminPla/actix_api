use actix_web::{HttpResponse, Responder, web};
use crate::types::authentication::{Authentication, LoginRequest};

pub async fn login(
    req: web::Json<LoginRequest>
    ) -> impl Responder {
    let token = Authentication::generate_token(&req.email);
    HttpResponse::Ok().json(token)
}
