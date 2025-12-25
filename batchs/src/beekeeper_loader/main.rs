mod beekeeper_loader_request;
mod beekeeper_loader_usecase;

use crate::beekeeper_loader_request::BeekeeperLoaderRequestDto;

#[tokio::main]
async fn main() {
    println!("beekeeper_loader 起動");
    env_logger::init();
    let args = std::env::args().collect::<Vec<String>>();
    let master_data_file = args
        .get(1)
        .expect("マスターデータファイルのパスを指定してください");
    let db_url = args.get(2).expect("データベースのURLを指定してください");
    let pool = common::infrastructure::db::sqlx::get_sqlite_pool(db_url.to_string());
    let _dao = BeekeeperLoaderRequestDto::new(master_data_file, pool);
    if let Err(e) = beekeeper_loader_usecase::run(_dao).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    println!("beekeeper_loader 終了");
}

#[cfg(test)]
#[path = "../../test/beekeeper_loader/main_test.rs"]
mod test_beekeeper_load;
