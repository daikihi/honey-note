use std::env::args;

use log::info;
mod flower_loader_master_gateway;
mod flower_loader_request;
mod flower_loader_usecase;

use flower_loader_request::FlowerLoaderRequestDto;

#[tokio::main]
async fn main() {
    println!("flower_loader 起動");
    env_logger::init();

    let args: Vec<String> = args().collect();

    let master_file_name = args.get(1).expect("マスターファイル名を指定してください");
    let db_file_name = args.get(2).expect("DBファイル名を指定してください");
    let user_id = args.get(3).expect("ユーザーIDを指定してください").parse::<i32>().expect("ユーザーIDは数値で指定してください");
    info!(
        "マスターファイル名: {}, DBファイル名: {}, user_id: {}",
        master_file_name, db_file_name, user_id
    );
    let dto: FlowerLoaderRequestDto =
        FlowerLoaderRequestDto::new(master_file_name.to_string(), db_file_name.to_string());
    let _ = flower_loader_usecase::run(dto, user_id).await;

    info!("flower_loader 完了");
}
