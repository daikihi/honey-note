extern crate log;
mod prefecture_loader_request;
mod prefecture_usecase;

use log::info;
use std::env;

use common::infrastructure::db::sqlx::get_sqlite_pool;
use prefecture_loader_request::PrefectureLoaderRequestDto;

#[tokio::main]
async fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    info!("args: {:?}", args);

    if args.len() != 3 {
        panic!("Usage: prefecture_loader <file_name>");
    }

    let file_name = &args[1];
    let db_file_name = &args[2];
    info!("prefecture_loader is starting to run ...{:?}", file_name);

    let pool = get_sqlite_pool(db_file_name.to_string());
    let _dao: PrefectureLoaderRequestDto = PrefectureLoaderRequestDto {
        file_name: file_name.to_string(),
        pool: &pool,
    };

    let _ = prefecture_usecase::run(_dao).await;

    info!("complete prefecture_loader");
}
