mod controllers;
mod types;
mod utils;

use actix_web::{App, HttpServer,HttpResponse, web};
use crate::controllers::create_user;
use crate::utils::{initialize_db,connect_db};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn: Connection =  connect_db();
    let db = Arc::new(Mutex::new(conn));
    initialize_db(db.clone());

    HttpServer::new(move|| {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/status", web::get().to(|| async { HttpResponse::Ok() }))
            .service(web::scope("/users")
                .route("/create_user", web::post().to(create_user))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
