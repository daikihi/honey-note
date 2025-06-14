mod controllers;

use actix_web::{App, HttpServer};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!("Starting server ...... ");
    HttpServer::new(|| {
        App::new()
            .service(controllers::health_checking::health_check)
            .service(controllers::prefecture_controller::get_all_prefectures)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
