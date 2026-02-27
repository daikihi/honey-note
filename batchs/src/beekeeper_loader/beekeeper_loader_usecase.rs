use crate::beekeeper_loader_request::BeekeeperLoaderRequestDto;
use common::errors::AppError;
use common::infrastructure::gateway::filesystem::load_master_data::load_master_data;
use common_type::models::beekeeper::Beekeeper as ModelBeekeeper;
use log::{error, info};

pub async fn run(request_dto: BeekeeperLoaderRequestDto<'_>) -> Result<(), AppError> {
    let file_name = request_dto.file_name;
    let connection_pool = request_dto.pool;

    let master_data: String = load_master_data(file_name);
    if master_data.is_empty() {
        return Err(AppError::InvalidInput(format!(
            "Master data is empty or could not be read: {}",
            file_name
        )));
    }

    info!("master data {}", master_data);
    let mut tx = connection_pool.begin().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
    for line in master_data.lines() {
        info!("Processing line: {}", line);
        let beekeeper_master_data: Vec<&str> = line.split(',').collect();
        if beekeeper_master_data.len() < 4 {
            error!("Invalid CSV format at line: {}", line);
            return Err(AppError::InvalidInput(format!(
                "Invalid CSV format: {}",
                line
            )));
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
                    error!("Error getting prefecture ID for {}: {}", beekeeper_prefecture, e);
                    return Err(e);
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
            common::repository::beekeepers::has_beekeeper(&model_beekeeper, &mut *tx).await;
        if !has_beekeeper {
            info!("Inserting new beekeeper: {:?}", &model_beekeeper);
            let _ = common::repository::beekeepers::insert_beekeeper(&model_beekeeper, &mut *tx)
                .await;
        } else {
            info!("Beekeeper already exists: {:?}", &model_beekeeper);
            continue;
        }
    }
    tx.commit().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(())
}
