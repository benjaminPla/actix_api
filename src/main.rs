mod controllers;
mod types;
mod utils;

use actix_web::{App, HttpServer,HttpResponse, web};
use crate::controllers::authentication::login;
use crate::controllers::users::{create_user, get_users};
use crate::types::users::User;
use crate::utils::connect_db;
use dotenv::dotenv;
use rusqlite::Connection;
use std::env;
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let host= env::var("HOST").expect("Missing `HOST` env variable");
    let port = env::var("PORT").expect("Missing `PORT` env variable");

    let conn: Connection =  connect_db();
    let db = Arc::new(Mutex::new(conn));
    User::new_admin(db.clone());

    HttpServer::new(move|| {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/status", web::get().to(|| async { HttpResponse::Ok() }))
            .route("/login", web::post().to(login))
            .service(web::scope("/users")
                .route("/get_users", web::get().to(get_users))
                .route("/create_user", web::post().to(create_user))
            )
    })
    .bind((host, port.parse().expect("Error parsing `port`")))?
    .run()
    .await
}
