use crate::{
    flower_loader_master_gateway::load_master_data, flower_loader_request::FlowerLoaderRequestDto,
};

use common::infrastructure::db::sqlx::get_sqlite_pool;
use common::repository::flowers::{FlowerRepository, FlowerRepositorySqlite};
use log::info;

pub async fn run(dto: FlowerLoaderRequestDto, user_id: i32) {
    info!("flower_loader_usecase::run start, user_id={}", user_id);
    let master_file_name = dto.master_file_name.as_str();
    let db_file_name = dto.db_file_name;

        let master_data: String = load_master_data(&master_file_name);
        let connection_pool = get_sqlite_pool(db_file_name.to_string());

        let mut tx = connection_pool.begin().await.expect("Failed to begin transaction");

        for line in master_data.lines() {
            if line.is_empty() {
                // parse error or empty line does not need to be processed
                continue;
            }

            use common_type::models::flowers;
            let flower = flowers::create_model_flower_from_name(line);

            let repo = FlowerRepositorySqlite { pool: connection_pool.clone() };
            match repo.has_flower(&flower, user_id, &mut *tx).await {
                Ok(true) => {
                    log::info!("Flower already exists for this user: {:?}", line);
                }
                Ok(false) => {
                    log::info!("Inserting new flower: {:?}", line);
                    let _ = repo.insert_flower(&flower, user_id, &mut *tx).await;
                }
                Err(e) => {
                    log::error!("Error checking flower existence: {}", e);
                }
            }
        }
        tx.commit().await.expect("Failed to commit transaction");
}
