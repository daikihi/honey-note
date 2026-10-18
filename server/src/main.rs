#[path = "controllers.rs"]
mod controllers;
mod middleware;
mod use_case;

use actix_cors::Cors;
use actix_files::Files;
// for signing key
use actix_governor::governor::middleware::NoOpMiddleware;
use actix_governor::{Governor, GovernorConfig, GovernorConfigBuilder, PeerIpKeyExtractor};
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{middleware::Logger, web, App, HttpServer};
use common::libs::config::models::server::{load_config, RateLimit};

#[derive(Debug, PartialEq, Eq)]
enum StartupOption {
    Environment(&'static str),
    Help,
}

fn parse_startup_args(args: &[String]) -> Result<StartupOption, String> {
    match args {
        [] => Ok(StartupOption::Environment("local")),
        [flag] if flag == "--help" || flag == "-h" => Ok(StartupOption::Help),
        [flag] if flag == "--env" => Err("--env の値がありません".to_string()),
        [flag, value] if flag == "--env" => match value.as_str() {
            "local" => Ok(StartupOption::Environment("local")),
            "prod" => Ok(StartupOption::Environment("prod")),
            _ => Err(format!(
                "環境名 '{value}' は指定できません（local または prod）"
            )),
        },
        _ => Err(format!("不正な引数です: {}", args.join(" "))),
    }
}

fn usage() -> &'static str {
    "使用方法: server [--env local|prod]\n  --env local|prod  起動環境を選択（省略時は local）\n  --help            この使用方法を表示"
}

fn is_production(env: &str) -> bool {
    env == "prod"
}

fn build_governor_config(
    config: &RateLimit,
) -> Result<GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware>, String> {
    // Governor内部では、全件回復までの時間をu64のナノ秒で保持する。
    u64::from(config.request_interval_ms)
        .checked_mul(u64::from(config.burst_size))
        .and_then(|milliseconds| milliseconds.checked_mul(1_000_000))
        .ok_or_else(|| {
            "rate_limit.request_interval_ms と burst_size の積は18446744073709以下にしてください"
                .to_string()
        })?;
    GovernorConfigBuilder::default()
        .milliseconds_per_request(u64::from(config.request_interval_ms))
        .burst_size(config.burst_size)
        .finish()
        .ok_or_else(|| "レート制限の回復間隔と連続許可件数は1以上にしてください".to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let env = match parse_startup_args(&args) {
        Ok(StartupOption::Environment(env)) => env,
        Ok(StartupOption::Help) => {
            println!("{}", usage());
            return Ok(());
        }
        Err(error) => {
            eprintln!("{error}\n{}", usage());
            std::process::exit(1);
        }
    };

    let config = match load_config(env) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("環境 '{env}' の設定を読み込めません: {error}");
            std::process::exit(1);
        }
    };

    let is_production = is_production(env);

    let governor_conf = match build_governor_config(&config.rate_limit) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("環境 '{env}' のレート制限を設定できません: {error}");
            std::process::exit(1);
        }
    };

    // 本番では64バイトの固定鍵を必須とし、ローカルでは未設定時に生成する。
    let secret_key = match std::env::var("SESSION_SECRET_KEY") {
        Ok(k) => {
            let key_bytes = k.as_bytes();
            if key_bytes.len() != 64 {
                eprintln!(
                    "SESSION_SECRET_KEY は64バイトで指定してください（現在: {}バイト）",
                    key_bytes.len()
                );
                std::process::exit(1);
            }
            Key::from(key_bytes)
        }
        Err(std::env::VarError::NotPresent) if is_production => {
            eprintln!("本番環境では SESSION_SECRET_KEY が必要です");
            std::process::exit(1);
        }
        Err(std::env::VarError::NotPresent) => {
            log::warn!("SESSION_SECRET_KEY が未設定のため、ローカル用の鍵を生成します");
            Key::generate()
        }
        Err(error) => {
            eprintln!("SESSION_SECRET_KEY を読み取れません: {error}");
            std::process::exit(1);
        }
    };

    let pool = common::infrastructure::db::sqlx::get_sqlite_pool(config.database_url.clone());
    if let Err(error) = pool.acquire().await {
        eprintln!(
            "環境 '{env}' の DB '{}' に接続できません: {error}",
            config.database_url
        );
        std::process::exit(1);
    }

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("honey_note_session".to_string())
                    .cookie_secure(is_production)
                    .build(),
            )
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .wrap(Governor::new(&governor_conf))
            .configure(configure_routes)
    })
    .bind((config.host_name.as_str(), config.port))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::{build_governor_config, is_production, parse_startup_args, StartupOption};
    use common::libs::config::models::server::RateLimit;

    fn parse(args: &[&str]) -> Result<StartupOption, String> {
        parse_startup_args(
            &args
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>(),
        )
    }

    #[test]
    fn selects_startup_environment() {
        assert_eq!(parse(&[]), Ok(StartupOption::Environment("local")));
        assert_eq!(
            parse(&["--env", "local"]),
            Ok(StartupOption::Environment("local"))
        );
        assert_eq!(
            parse(&["--env", "prod"]),
            Ok(StartupOption::Environment("prod"))
        );
    }

    #[test]
    fn rejects_invalid_arguments() {
        assert!(parse(&["--env", "invalid"]).is_err());
        assert!(parse(&["--env"]).is_err());
        assert!(parse(&["--other"]).is_err());
    }

    #[test]
    fn shows_help_and_detects_production() {
        assert_eq!(parse(&["--help"]), Ok(StartupOption::Help));
        assert!(!is_production("local"));
        assert!(is_production("prod"));
    }

    #[test]
    fn rate_limit_rejects_zero_settings_without_panicking() {
        assert!(build_governor_config(&RateLimit {
            request_interval_ms: 0,
            burst_size: 20,
        })
        .is_err());
        assert!(build_governor_config(&RateLimit {
            request_interval_ms: 2000,
            burst_size: 0,
        })
        .is_err());
    }

    #[test]
    fn rate_limit_checks_internal_duration_limit_without_panicking() {
        assert!(build_governor_config(&RateLimit {
            request_interval_ms: u32::MAX,
            burst_size: u32::MAX,
        })
        .is_err());
        assert!(build_governor_config(&RateLimit {
            request_interval_ms: u32::MAX,
            burst_size: 4294,
        })
        .is_ok());
        assert!(build_governor_config(&RateLimit {
            request_interval_ms: u32::MAX,
            burst_size: 4295,
        })
        .is_err());
    }

    #[actix_web::test]
    async fn rate_limit_uses_configured_burst_and_interval() {
        use actix_governor::Governor;
        use actix_web::{http::StatusCode, test, web, App, HttpResponse};

        let config = build_governor_config(&RateLimit {
            request_interval_ms: 60_000,
            burst_size: 2,
        })
        .unwrap();
        let app = test::init_service(
            App::new()
                .wrap(Governor::new(&config))
                .route("/", web::get().to(|| async { HttpResponse::Ok().finish() })),
        )
        .await;

        for _ in 0..2 {
            let request = test::TestRequest::get()
                .uri("/")
                .peer_addr("127.0.0.1:12345".parse().unwrap())
                .to_request();
            assert_eq!(
                test::call_service(&app, request).await.status(),
                StatusCode::OK
            );
        }

        let request = test::TestRequest::get()
            .uri("/")
            .peer_addr("127.0.0.1:12345".parse().unwrap())
            .to_request();
        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
        let remaining_seconds = response
            .headers()
            .get("x-ratelimit-after")
            .unwrap()
            .to_str()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        // 実時間で待たず、設定した60秒間隔が制限応答へ反映されることを確認する。
        assert!((30..=60).contains(&remaining_seconds));
    }
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
