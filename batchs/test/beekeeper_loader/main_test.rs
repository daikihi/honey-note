#[cfg(test)]
mod test_beekeeper_load {
    use crate::beekeeper_loader_request::BeekeeperLoaderRequestDto;
    use crate::beekeeper_loader_usecase;
    use common::repository::beekeepers::get_all_beekeepers;
    use common_type::models::prefectures::Prefecture as PrefectureModel;
    use sqlx::SqlitePool;
    use std::io::Write;
    use tempfile::NamedTempFile;


    use crate::test::common::batch_test_base;

    async fn setup_test_db(db_url: &str) -> SqlitePool {
        batch_test_base::test_batch::setup_test_db(db_url).await
    }

    #[tokio::test]
    async fn test_beekeeper_load_run() {
        // 正常なCSVファイルを読み込み、データベースに養蜂場情報が正しく登録されることを確認する
        // Load a valid CSV file and verify that the beekeeper information is correctly registered in the database.
        let db_url = "sqlite::memory:";
        let pool = setup_test_db(db_url).await;

        common::repository::prefectures::insert_prefecture(
            &PrefectureModel {
                id: 1,
                name_jp: "北海道".to_string(),
                name_en: "Hokkaido".to_string(),
            },
            &pool,
        )
        .await;

        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "テスト養蜂場,Test Beekeeper,北海道,札幌市").unwrap();
        let file_path = file.path().to_str().unwrap();

        let dto = BeekeeperLoaderRequestDto::new(file_path, pool.clone());

        // execute testing
        let result = beekeeper_loader_usecase::run(dto).await;
        assert!(result.is_ok(), "Error: {:?}", result.err());

        // verify
        let beekeepers = get_all_beekeepers(&pool).await.unwrap();
        assert_eq!(beekeepers.len(), 1);
        assert_eq!(beekeepers[0].name_jp, "テスト養蜂場");
        assert_eq!(beekeepers[0].name_en, Some("Test Beekeeper".to_string()));
        assert_eq!(beekeepers[0].location_prefecture_id, Some(1));
        assert_eq!(beekeepers[0].location_city, Some("札幌市".to_string()));
    }

    #[tokio::test]
    async fn test_beekeeper_load_empty_csv() {
        // 空のCSVファイルを読み込んだ際に、バリデーションエラーが発生することを確認する
        // Verify that a validation error occurs when an empty CSV file is loaded.
        let db_url = "sqlite::memory:";
        let pool = setup_test_db(db_url).await;

        let file = NamedTempFile::new().unwrap();
        let file_path = file.path().to_str().unwrap();

        let dto = BeekeeperLoaderRequestDto::new(file_path, pool.clone());

        let result = beekeeper_loader_usecase::run(dto).await;
        assert!(result.is_err());
        if let Err(common::errors::AppError::InvalidInput(msg)) = result {
            assert!(msg.contains("Master data is empty"));
        } else {
            panic!("Unexpected error type: {:?}", result);
        }
    }

    #[tokio::test]
    async fn test_beekeeper_load_invalid_csv_format() {
        // 不正なフォーマットのCSVファイルを読み込んだ際に、バリデーションエラーが発生することを確認する
        // Verify that a validation error occurs when a CSV file with an invalid format is loaded.
        let db_url = "sqlite::memory:";
        let pool = setup_test_db(db_url).await;

        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "Invalid Data,Only Two Columns").unwrap();
        let file_path = file.path().to_str().unwrap();

        let dto = BeekeeperLoaderRequestDto::new(file_path, pool.clone());

        let result = beekeeper_loader_usecase::run(dto).await;
        assert!(result.is_err());
        if let Err(common::errors::AppError::InvalidInput(msg)) = result {
            assert!(msg.contains("Invalid CSV format"));
        } else {
            panic!("Unexpected error type: {:?}", result);
        }
    }

    #[tokio::test]
    async fn test_beekeeper_load_missing_prefecture() {
        // 存在しない都道府県名を含むCSVを読み込んだ際に、エラーが発生することを確認する
        // Verify that an error occurs when loading a CSV containing a non-existent prefecture name.
        let db_url = "sqlite::memory:";
        let pool = setup_test_db(db_url).await;

        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "テスト養蜂場,Test Beekeeper,存在しない県,札幌市").unwrap();
        let file_path = file.path().to_str().unwrap();

        let dto = BeekeeperLoaderRequestDto::new(file_path, pool.clone());

        let result = beekeeper_loader_usecase::run(dto).await;
        assert!(result.is_err());
        if let Err(common::errors::AppError::DatabaseError(_)) = result {
            // OK: 都道府県が見つからない場合は現状 DatabaseError になる (AppError::NotFound ではない理由はリポジトリの実装に依存)
        } else {
            panic!("Unexpected error type: {:?}", result);
        }
    }
}
