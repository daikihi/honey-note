mod honey_loader_gateway;
mod honey_loader_models;
mod honey_loader_request;
mod honey_loader_usecase;

#[tokio::main]
async fn main() {
    println!("honey_loader 起動");
    env_logger::init();
    let args = std::env::args().collect::<Vec<String>>();
    let file_name = args.get(1).expect("ファイル名を指定してください");
    let db_file_name = args
        .get(2)
        .expect("データベースのファイル名を指定してください");
    let request_dto = honey_loader_request::HoneyLoaderRequestDto::new(file_name, db_file_name);
    let _use_case_result = honey_loader_usecase::run(request_dto).await;
}
