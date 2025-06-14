use crate::prefecture_loader_request::PrefectureLoaderRequestDto;
use common::models::prefectures::Prefecture as PrefectureModel;
use log::{error, info};
use std::fs;

pub async fn run<'a>(_dao: PrefectureLoaderRequestDto<'a>) {
    let _file_name = _dao.file_name;
    let content = match fs::read_to_string(&_file_name) {
        Ok(_content) => _content,
        Err(e) => {
            error!("Error reading file {}: {}", _file_name, e);
            return;
        }
    };

    let connection_pool = _dao.pool;
    info!("Database connection pool created successfully");

    let lines = content.lines();
    for line in lines {
        info!("Processing line: {}", line);
        let prefecture_opt: Option<PrefectureModel> = PrefectureModel::from_string_csv(line);
        match prefecture_opt {
            Some(prefecture) => {
                info!("Loaded prefecture: {:?}", prefecture);
                let has_prefecture =
                    common::repository::prefectures::has_prefecture(&prefecture, &connection_pool)
                        .await;
                match has_prefecture {
                    Ok(true) => {
                        info!("Prefecture already exists: {:?}", prefecture);
                        continue;
                    }
                    Ok(false) => {
                        info!("Inserting new prefecture: {:?}", prefecture);
                        common::repository::prefectures::insert_prefecture(
                            &prefecture,
                            &connection_pool,
                        )
                        .await;
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

#[cfg(test)]
mod tests {
    use sqlx::query;
    use sqlx::sqlite::SqlitePoolOptions;

    use super::*;
    use common::infrastructure::db::sqlx::prefecture;
    use common::repository::prefectures;

    #[tokio::test]
    async fn test_run() {
        use tempfile::NamedTempFile;

        // 1. 一時ファイル作成
        let tmpfile = NamedTempFile::new().unwrap();
        let db_path = format!("sqlite://{}", tmpfile.path().to_str().unwrap());

        // 正しい初期化方法: get_or_init で async ブロックを使う
        let _pool = SqlitePoolOptions::new()
            .connect(&db_path)
            .await
            .expect("Failed to connect to the in-memory database");

        query(
            "CREATE TABLE prefecture (
                id INTEGER PRIMARY KEY,
                name_jp TEXT NOT NULL,
                name_en TEXT NOT NULL
            )",
        )
        .execute(&_pool)
        .await
        .unwrap();

        let csv_path = "../resources/master_data/japanese_prefectures.csv";
        println!("CARGO_MANIFEST_DIR: {}", env!("CARGO_MANIFEST_DIR"));
        assert!(
            std::path::Path::new(&csv_path).exists(),
            "CSVファイルが存在しません"
        );
        let content = std::fs::read_to_string(&csv_path).expect("CSVファイルが読めません");
        println!("CSV内容: {}", content);

        let _dao = PrefectureLoaderRequestDto {
            file_name: csv_path.to_string(),
            pool: &_pool, // Use shared in-memory database for testing
        };
        let _ = run(_dao).await;

        let prefectures: Result<Vec<prefecture::Prefecture>, sqlx::Error> =
            prefectures::get_all_prefectures(&_pool).await;

        assert!(
            prefectures.is_ok(),
            "Failed to fetch prefectures from the database"
        );
        match prefectures {
            Ok(_prefectures) => {
                assert!(
                    _prefectures.iter().cloned().len() == 47,
                    "Expected 47 prefectures, found {}",
                    _prefectures.len()
                );
            }
            Err(e) => {
                panic!("Error fetching prefectures: {}", e);
            }
        }

        info!("Test run completed successfully");
        tmpfile.close().unwrap(); // Clean up the temporary file
    }
}
