mod controllers;

use actix_files::Files;
use actix_web::{App, HttpServer};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!("Starting server ...... ");
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/honey_note/icons", "server/src/assets/icons"))
            .service(Files::new("/honey_note", "server/src/assets/html"))
            .service(controllers::health_checking::health_check)
            .service(controllers::prefecture_controller::get_all_prefectures)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
