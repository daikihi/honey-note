mod controllers;
mod use_case;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware::Logger, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default()) // ← アクセスログ中間層
            .wrap(Cors::permissive())
            .service(controllers::honey_controller::get_honey_details)
            .service(controllers::honey_controller::get_all_honeys)
            .service(controllers::health_checking::health_check)
            .service(controllers::prefecture_controller::get_all_prefectures)
            .service(controllers::beekeeper_controller::get_all_beekeepers)
            .service(controllers::flower_controller::get_all_flowers)
            .service(Files::new(
                "/honey_note/javascript",
                "server/src/assets/javascript",
            ))
            .service(Files::new("/honey_note/css", "server/src/assets/css"))
            .service(Files::new("/honey_note/icons", "server/src/assets/icons"))
            .service(Files::new("/honey_note", "server/src/assets/html"))
            .service(Files::new("/honey_note/", "server/src/assets/"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
