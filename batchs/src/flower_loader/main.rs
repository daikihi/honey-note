use std::env::args;

use log::info;
mod flower_loader_usecase;
mod flower_loader_request;
mod flower_loader_master_gateway;

use flower_loader_request::FlowerLoaderRequestDto;

fn main() {
    println!("flower_loader 起動");
    env_logger::init();

    let args: Vec<String> = args().collect();

    let master_file_name = args.get(1).expect("マスターファイル名を指定してください");
    let db_file_name = args.get(2).expect("DBファイル名を指定してください");    
    info!(
        "マスターファイル名: {}, DBファイル名: {}",
        master_file_name, db_file_name
    );
    let dto = FlowerLoaderRequestDto::new(master_file_name.to_string(), db_file_name.to_string());
    let _ = flower_loader_usecase::run(dto);

    info!("flower_loader 完了");
}
