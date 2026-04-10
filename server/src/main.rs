#[path = "controllers.rs"]
mod controllers;
mod middleware;
mod use_case;

use actix_cors::Cors;
use actix_files::Files;
// for signing key
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{middleware::Logger, web, App, HttpServer};
use common::libs::config::models::server::load_config;
use common::libs::config::models::server::Server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let server_env_opt: Option<Server> = load_config("local");

    let c = match server_env_opt {
        Some(c) => c,
        None => {
            eprintln!("server cannot read config files");
            std::process::exit(1);
        }
    };

    let path = common::infrastructure::db::sqlx::DB_FILE_NAME;
    let pool = common::infrastructure::db::sqlx::get_sqlite_pool(path.to_string());

    let governor_conf = GovernorConfigBuilder::default()
        .seconds_per_request(2) // 2秒に1リクエスト（0.5 req/s）
        .burst_size(20) // 最大20回バースト許可
        .finish()
        .unwrap();

    // Cookie signing key (in production, this should be from config/env)
    // let secret_key = Key::generate();
    let secret_key = std::env::var("SESSION_SECRET_KEY")
        .map(|k| Key::from(k.as_bytes()))
        .unwrap_or_else(|_| Key::generate()); // 開発環境だけ generate でも許容
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("honey_note_session".to_string())
                    .cookie_secure(false) // Set to true if using HTTPS
                    .build(),
            )
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .wrap(Governor::new(&governor_conf))
            .configure(configure_routes)
    })
    .bind((c.host_name.as_str(), c.port))?
    .run()
    .await
}

fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(controllers::auth_controller::signup)
        .service(controllers::auth_controller::login)
        .service(controllers::auth_controller::logout)
        .service(controllers::auth_controller::me)
        .service(controllers::honey_controller::get_all_honeys)
        .service(controllers::honey_controller::get_honey_by_id)
        .service(controllers::health_checking::health_check)
        .service(controllers::prefecture_controller::get_all_prefectures)
        .service(controllers::beekeeper_controller::get_all_beekeepers)
        .service(controllers::beekeeper_controller::get_beekeeper_by_id)
        .service(controllers::beekeeper_controller::put_new_beekeeper)
        .service(controllers::beekeeper_controller::put_edit_beekeeper)
        .service(controllers::flower_controller::get_all_flowers)
        .service(controllers::flower_controller::get_flower_by_id)
        .service(controllers::flower_controller::put_new_flower)
        .service(controllers::flower_controller::put_edit_flower)
        .service(controllers::honey_controller::put_new_honey)
        .service(controllers::honey_controller::put_edit_honey)
        .service(Files::new(
            "/honey_note/javascript",
            "server/src/assets/javascript",
        ))
        .service(Files::new("/honey_note/css", "server/src/assets/css"))
        .service(Files::new("/honey_note/icons", "server/src/assets/icons"))
        .service(Files::new("/honey_note", "server/src/assets/html"))
        .service(Files::new("/honey_note/", "server/src/assets/"));
}
