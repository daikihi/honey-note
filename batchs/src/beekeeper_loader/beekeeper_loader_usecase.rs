use crate::beekeeper_loader_request::beekeeper_loader_request_dto;
use common::infrastructure::gateway::filesystem::load_master_data::load_master_data;
use common::models::beekeeper::Beekeeper as ModelBeekeeper;
use log::info;

pub async fn run(requestDto: beekeeper_loader_request_dto<'_>) {
    let file_name = requestDto.file_name;
    let db_file_name = requestDto.db_file_name;

    let connection_pool =
        common::infrastructure::db::sqlx::get_sqlite_pool(db_file_name.to_string());

    let master_data: String = load_master_data(file_name);
    info!("master data {}", master_data);
    for line in master_data.lines() {
        info!("Processing line: {}", line);
        let model_beekeeper = ModelBeekeeper::from_string_csv(line);

        info!("Loaded beekeeper: {:?}", &model_beekeeper);
        let has_beekeeper =
            common::repository::beekeepers::has_beekeeper(&model_beekeeper, &connection_pool).await;
        if !has_beekeeper {
            info!("Inserting new beekeeper: {:?}", &model_beekeeper);
            common::repository::beekeepers::insert_beekeeper(&model_beekeeper, &connection_pool)
                .await;
        } else {
            info!("Beekeeper already exists: {:?}", &model_beekeeper);
            continue;
        }
    }
}
