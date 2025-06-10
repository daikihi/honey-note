extern crate log;
mod prefecture_loader_request;
mod prefecture_usecase;

use log::info;
use std::env;

use prefecture_loader_request::PrefectureLoaderRequestDto;


#[tokio::main]
async fn main() {
     env_logger::init();
     let args: Vec<String> = env::args().collect();
     info!("args: {:?}", args);

     if args.len() != 2 {
        panic!("Usage: prefecture_loader <file_name>");
     }

     let file_name = &args[1];
    info!("prefecture_loader is starting to run ...{:?}", file_name);

    let _dao: PrefectureLoaderRequestDto = PrefectureLoaderRequestDto {
        file_name: file_name.to_string(),
    };

    let _ = prefecture_usecase::run(_dao).await;

    info!("complete prefecture_loader");
}