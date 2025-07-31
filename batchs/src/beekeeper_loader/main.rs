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
    let _dao = BeekeeperLoaderRequestDto::new(master_data_file, db_url);
    beekeeper_loader_usecase::run(_dao).await;
    println!("beekeeper_loader 終了");
}
