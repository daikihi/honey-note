use crate::{
    flower_loader_master_gateway::load_master_data, flower_loader_request::FlowerLoaderRequestDto,
};

use common::infrastructure::db::sqlx::get_sqlite_pool;
use log::info;

pub async fn run(dto: FlowerLoaderRequestDto) {
    info!("flower_loader_usecase::run start");
    let master_file_name = dto.master_file_name.as_str();
    let db_file_name = dto.db_file_name;

    let master_data: String = load_master_data(&master_file_name);

    for line in master_data.lines() {
        if line.is_empty() {
            // parse error or empty line does not need to be processed
            continue;
        }


        use common_type::models::flowers;
        let flower = flowers::create_model_flower_from_name(line);

        let connection_pool = get_sqlite_pool(db_file_name.to_string());
        let has_flower = common::repository::flowers::has_flower(&flower, &connection_pool).await;
        match has_flower {
            Ok(true) => {
                log::info!("Flower already exists: {:?}", line);
            }
            Ok(false) => {
                log::info!("Inserting new flower: {:?}", line);
                let _ = common::repository::flowers::insert_flower(&flower, &connection_pool).await;
            }
            Err(e) => {
                log::error!("Error checking flower existence: {}", e);
            }
        }
    }
}
