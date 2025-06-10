use crate::prefecture_loader_request::PrefectureLoaderRequestDto;
use std::fs;
use common::models::prefectures::Prefecture as PrefectureModel;
use log::{error, info};

pub async fn run(_dao: PrefectureLoaderRequestDto) {
    let _file_name = _dao.file_name;
    let content = match fs::read_to_string(&_file_name) {
        Ok(_content) => _content,
        Err(e) => {
            error!("Error reading file {}: {}", _file_name, e);
            return;
        }
    };

    let connection_pool = common::infrastructure::db::sqlx::create_sqlite_pool().await;
    info!("Database connection pool created successfully");
    
    let lines = content.lines();
    for line in lines {
        info!("Processing line: {}", line);
        let prefecture_opt: Option<PrefectureModel> = PrefectureModel::from_string_csv(line);
        match prefecture_opt {
            Some(prefecture) => {
                info!("Loaded prefecture: {:?}", prefecture);
                let has_prefecture = common::repository::prefectures::has_prefecture(&prefecture, &connection_pool).await;
                match has_prefecture {
                    Ok(true) => {
                        info!("Prefecture already exists: {:?}", prefecture);
                        continue
                    }
                    Ok(false) => {
                        info!("Inserting new prefecture: {:?}", prefecture);
                        common::repository::prefectures::insert_prefecture(&prefecture, &connection_pool).await;
                    }
                    Err(e) => {
                        error!("Error checking prefecture existence: {}", e);
                        continue;
                    }
                    
                }

            }
            None => {
                error!("Failed to parse line: {}", line);
            }
        }
    }

}