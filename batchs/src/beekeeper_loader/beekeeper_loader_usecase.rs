use crate::beekeeper_loader_request::BeekeeperLoaderRequestDto;
use common::infrastructure::gateway::filesystem::load_master_data::load_master_data;
use common_type::models::beekeeper::Beekeeper as ModelBeekeeper;
use log::info;

pub async fn run(request_dto: BeekeeperLoaderRequestDto<'_>) {
    let file_name = request_dto.file_name;
    let db_file_name = request_dto.db_file_name;

    let connection_pool =
        common::infrastructure::db::sqlx::get_sqlite_pool(db_file_name.to_string());

    let master_data: String = load_master_data(file_name);
    info!("master data {}", master_data);
    for line in master_data.lines() {
        info!("Processing line: {}", line);
        let beekeeper_master_data: Vec<&str> = line.split(',').collect();
        if beekeeper_master_data.is_empty() {
            continue;
        }
        let beekeeper_name: &str = beekeeper_master_data[0];
        let beekeeper_name_en: Option<&str> = if beekeeper_master_data[1].is_empty() {
            None
        } else {
            Some(beekeeper_master_data[1])
        };
        let beekeeper_prefecture: &str = beekeeper_master_data[2];
        let beekeeper_city: Option<&str> = if beekeeper_master_data[3].is_empty() {
            None
        } else {
            Some(beekeeper_master_data[3])
        };

        let prefecture_id = if beekeeper_prefecture.is_empty() {
            None
        } else {
            let prefecture_opt = common::repository::prefectures::get_prefecture_by_name(
                beekeeper_prefecture,
                &connection_pool,
            )
            .await;
            match prefecture_opt {
                Ok(prefecture) => Some(prefecture.id),
                Err(e) => {
                    info!("Error getting prefecture ID: {}", e);
                    None
                }
            }
        };

        let model_beekeeper = ModelBeekeeper::from_string_csv(
            beekeeper_name,
            beekeeper_name_en,
            beekeeper_city,
            prefecture_id,
        );

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
