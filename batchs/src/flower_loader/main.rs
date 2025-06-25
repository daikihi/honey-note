mod flower_loader_request;
mod flower_loader_usecase;
mod resources_master_gateway;

use crate::flower_loader_request::FlowerLoaderRequestDto;
use log::info;
use std::env;

fn main() {
    info!("flower_loader");
    env_logger::init();
    // get program parameters
    let args: Vec<String> = env::args().collect();
    info!("args: {:?}", args);

    // the number of arguments is dynamically changed because of the execution style

    let args_len = &args.len();
    let master_file_path = &args[args_len - 2];
    let db_file_path = &args[args_len - 1];
    let dto: FlowerLoaderRequestDto = flower_loader_request::FlowerLoaderRequestDto::new(
        master_file_path.to_string(),
        db_file_path.to_string(),
    );
    let _result = flower_loader_usecase::usecase::run(dto);
    info!("flower_loader execution end");
}
