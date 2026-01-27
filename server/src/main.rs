mod controllers;
mod use_case;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware::Logger, App, HttpServer};
use common::libs::config::models::server::load_config;
use common::libs::config::models::server::Server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    env_logger::init();

    let server_env_opt: Option<Server> = load_config("local");

    match server_env_opt {
        Some(c) => {
            HttpServer::new(|| {
                App::new()
                    .wrap(Logger::default()) // ← for access logs
                    .wrap(Cors::permissive())
                    .service(controllers::honey_controller::get_all_honeys)
                    .service(controllers::health_checking::health_check)
                    .service(controllers::prefecture_controller::get_all_prefectures)
                    .service(controllers::beekeeper_controller::get_all_beekeepers)
                    .service(controllers::flower_controller::get_all_flowers)
                    .service(controllers::honey_controller::put_new_honey)
                    .service(controllers::honey_controller::put_edit_honey)
                    .service(Files::new(
                        "/honey_note/javascript",
                        "server/src/assets/javascript",
                    ))
                    .service(Files::new("/honey_note/css", "server/src/assets/css"))
                    .service(Files::new("/honey_note/icons", "server/src/assets/icons"))
                    .service(Files::new("/honey_note", "server/src/assets/html"))
                    .service(Files::new("/honey_note/", "server/src/assets/"))
            })
            .bind((c.host_name.as_str(), c.port))?
            .run()
            .await
        }
        None => {
            eprintln!("server cannnot read config files");
            std::process::exit(1);
        }
    }
}
