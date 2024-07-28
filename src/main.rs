mod controllers;
mod types;
mod utils;

use actix_web::{App, HttpServer, web};
use controllers::{create_user,server_status};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn: Connection =  utils::connect();
    let db = Arc::new(Mutex::new(conn));
        HttpServer::new(move|| {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(server_status)
            .service(web::scope("/user")
                .route("/create_user", web::post().to(create_user))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
