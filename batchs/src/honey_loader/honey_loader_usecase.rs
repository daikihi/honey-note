use common::repository;
use common::repository::beekeepers::BeekeeperRepository;
use common_type::models::beekeeper::Beekeeper;
use std::collections::HashSet;

use crate::honey_loader_models::JsonHoney;
use crate::{honey_loader_gateway, honey_loader_request};

pub async fn run(
    request_dto: honey_loader_request::HoneyLoaderRequestDto<'_>,
    user_id: i32,
) -> Result<(), String> {
    log::info!(
        "honey_loader リクエスト: {:?}, user_id={}",
        request_dto,
        user_id
    );

    // ハチミツデータの読み込みとフィルタリング
    let non_empty_honeys = load_and_filter_honeys(request_dto.file_name)?;

    // データベース接続
    let pool = sqlx::SqlitePool::connect(&request_dto.db_file_name)
        .await
        .map_err(|e| format!("データベース接続に失敗しました: {}", e))?;

    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    // 養蜂家の処理
    process_and_insert_beekeepers(&non_empty_honeys, user_id, &mut tx).await?;

    // ハチミツの処理
    process_and_insert_honeys(&non_empty_honeys, user_id, &pool, &mut tx).await?;

    // トランザクションをコミット
    tx.commit().await.map_err(|e| e.to_string())?;
    log::info!("ハニーのデータベースへの挿入が完了しました");
    Ok(())
}

/// JSONファイルを読み込み、養蜂場名を正規化して空のハチミツ名を除外する
fn load_and_filter_honeys(file_name: &str) -> Result<Vec<JsonHoney>, String> {
    let json_honeys = honey_loader_gateway::run(file_name)?;
    log::info!(
        "honey_loader 完了: {} 件のハニーを読み込みました",
        json_honeys.len()
    );

    let mut non_empty_honeys = Vec::new();
    for mut honey in json_honeys {
        if !honey.name.as_deref().is_some_and(|name| !name.is_empty()) {
            continue;
        }
        let name = honey
            .beekeeper
            .as_deref()
            .ok_or_else(|| format!("ハチミツ ID {} に養蜂場名がありません", honey.id))?
            .trim();
        if name.is_empty() {
            return Err(format!("ハチミツ ID {} の養蜂場名が空です", honey.id));
        }
        honey.beekeeper = Some(name.to_string());
        non_empty_honeys.push(honey);
    }
    log::info!("非空のハニー: {} 件", non_empty_honeys.len());

    Ok(non_empty_honeys)
}

/// 養蜂家データを処理し、データベースに挿入する
async fn process_and_insert_beekeepers(
    honeys: &[JsonHoney],
    user_id: i32,
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
) -> Result<(), String> {
    let beekeepers: HashSet<&str> = honeys
        .iter()
        .filter_map(|json_honey| json_honey.beekeeper.as_deref())
        .collect();

    for beekeeper in beekeepers {
        repository::beekeepers::get_or_insert_beekeeper_id(beekeeper, user_id, &mut **tx)
            .await
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// ハチミツデータを処理し、データベースに挿入する
async fn process_and_insert_honeys(
    honeys: &[JsonHoney],
    user_id: i32,
    pool: &sqlx::SqlitePool,
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
) -> Result<(), String> {
    let mut model_honeys: Vec<common_type::models::honey::Honey> = Vec::new();

    let bk_repo = common::repository::beekeepers::BeekeeperRepositorySqlite { pool: pool.clone() };

    for json_honey in honeys {
        let name = json_honey
            .beekeeper
            .as_deref()
            .ok_or_else(|| format!("ハチミツ ID {} に養蜂場名がありません", json_honey.id))?;
        let beekeeper_id = bk_repo
            .get_beekeeper_id_by_name(name, user_id, &mut **tx)
            .await
            .ok_or_else(|| format!("養蜂場 ID の取得に失敗しました: {}", name))?;

        model_honeys.push(common_type::models::honey::Honey {
            id: Some(json_honey.id),
            name_jp: json_honey.name.clone().unwrap_or_default(),
            name_en: None,
            flower_names: vec![],
            beekkeeper: Some(Beekeeper {
                id: Some(beekeeper_id),
                name_jp: name.to_string(),
                name_en: None,
                founding_year: None,
                location_prefecture_id: None,
                location_city: None,
                website_url: None,
                note: None,
            }),
            origin_country: json_honey.country.clone(),
            origin_region: json_honey.prefecture.clone(),
            harvest_year: None,
            purchase_date: None,
            note: None,
        });
    }

    for model_honey in model_honeys {
        repository::honeys::insert_honey_if_not_exists(&model_honey, user_id, &mut **tx)
            .await
            .map_err(|e| {
                format!(
                    "ハチミツの挿入に失敗しました: {}: {}",
                    model_honey.name_jp, e
                )
            })?;
        log::info!("ハニーをデータベースに挿入: {:?}", model_honey);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::repository::honeys::{HoneyRepository, HoneyRepositorySqlite};
    use std::io::Write;
    use tempfile::{NamedTempFile, TempDir};

    async fn database() -> (TempDir, sqlx::SqlitePool, String) {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("honey.db");
        let url = format!("sqlite://{}", path.display());
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect_with(
                sqlx::sqlite::SqliteConnectOptions::new()
                    .filename(&path)
                    .create_if_missing(true),
            )
            .await
            .unwrap();
        sqlx::migrate!("../resources/db/migrations")
            .run(&pool)
            .await
            .unwrap();
        (dir, pool, url)
    }

    fn input(lines: &[&str]) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        for line in lines {
            writeln!(file, "{}", line).unwrap();
        }
        file
    }

    async fn load(file: &NamedTempFile, url: &str, user_id: i32) -> Result<(), String> {
        run(
            honey_loader_request::HoneyLoaderRequestDto::new(file.path().to_str().unwrap(), url),
            user_id,
        )
        .await
    }

    #[tokio::test]
    async fn loads_filtered_honeys_with_normalized_shared_beekeeper() {
        let (_dir, pool, url) = database().await;
        let file = input(&[
            r#"{"id":1,"name":"百花蜜","beekeeper":"  山の養蜂場  ","country":"日本"}"#,
            r#"{"id":2,"name":"アカシア","beekeeper":"山の養蜂場","prefecture":"長野"}"#,
            r#"{"id":3,"name":"","beekeeper":null}"#,
        ]);
        load(&file, &url, 1).await.unwrap();
        load(&file, &url, 1).await.unwrap();

        let beekeepers: Vec<(i32, String)> =
            sqlx::query_as("SELECT id, name_jp FROM beekeeper WHERE user_id = 1")
                .fetch_all(&pool)
                .await
                .unwrap();
        assert_eq!(beekeepers.len(), 1);
        assert_eq!(beekeepers[0].1, "山の養蜂場");
        let honeys: Vec<(String, Option<i32>, Option<String>, Option<String>)> = sqlx::query_as(
            "SELECT name_jp, beekeeper_id, origin_country, origin_region FROM honey WHERE user_id = 1 ORDER BY id",
        )
        .fetch_all(&pool)
        .await
        .unwrap();
        assert_eq!(honeys.len(), 2);
        assert!(honeys.iter().all(|h| h.1 == Some(beekeepers[0].0)));
        assert_eq!(honeys[0].2.as_deref(), Some("日本"));
        assert_eq!(honeys[1].3.as_deref(), Some("長野"));

        load(&file, &url, 2).await.unwrap();
        let count: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM beekeeper WHERE name_jp = '山の養蜂場'")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(count.0, 2);
    }

    #[tokio::test]
    async fn invalid_beekeeper_or_json_fails_without_writing() {
        let (_dir, pool, url) = database().await;
        for invalid in [
            r#"{"id":1,"name":"蜜","beekeeper":null}"#,
            r#"{"id":1,"name":"蜜","beekeeper":" 　"}"#,
            "not json",
        ] {
            let file = input(&[r#"{"id":2,"name":"正常","beekeeper":"養蜂場"}"#, invalid]);
            assert!(load(&file, &url, 1).await.is_err());
        }
        let beekeeper_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM beekeeper")
            .fetch_one(&pool)
            .await
            .unwrap();
        let honey_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM honey")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!((beekeeper_count.0, honey_count.0), (0, 0));
    }

    #[tokio::test]
    async fn honey_insert_failure_rolls_back_beekeeper_and_honey() {
        let (_dir, pool, url) = database().await;
        sqlx::query(
            "CREATE TRIGGER fail_honey BEFORE INSERT ON honey WHEN NEW.name_jp = '失敗' BEGIN SELECT RAISE(ABORT, 'test failure'); END",
        )
        .execute(&pool)
        .await
        .unwrap();
        let file = input(&[
            r#"{"id":1,"name":"正常","beekeeper":"養蜂場"}"#,
            r#"{"id":2,"name":"失敗","beekeeper":"養蜂場"}"#,
        ]);
        assert!(load(&file, &url, 1).await.is_err());
        for table in ["beekeeper", "honey"] {
            let query = format!("SELECT COUNT(*) FROM {}", table);
            let count: (i64,) = sqlx::query_as(&query).fetch_one(&pool).await.unwrap();
            assert_eq!(count.0, 0, "{} was not rolled back", table);
        }
    }

    #[tokio::test]
    async fn database_rejects_duplicate_and_untrimmed_beekeeper_names() {
        let (_dir, pool, _url) = database().await;
        sqlx::query("INSERT INTO beekeeper (name_jp, user_id) VALUES ('養蜂場', 1)")
            .execute(&pool)
            .await
            .unwrap();
        for query in [
            "INSERT INTO beekeeper (name_jp, user_id) VALUES ('養蜂場', 1)",
            "INSERT INTO beekeeper (name_jp, user_id) VALUES (' 養蜂場', 1)",
            "INSERT INTO beekeeper (name_jp, user_id) VALUES ('　養蜂場', 1)",
            "INSERT INTO beekeeper (name_jp, user_id) VALUES ('', 1)",
            "UPDATE beekeeper SET name_jp = '養蜂場 ' WHERE user_id = 1",
        ] {
            assert!(sqlx::query(query).execute(&pool).await.is_err(), "{query}");
        }
        sqlx::query("INSERT INTO beekeeper (name_jp, user_id) VALUES ('養蜂場', 2)")
            .execute(&pool)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn migration_stops_for_existing_blank_name() {
        let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::query("CREATE TABLE beekeeper (id INTEGER PRIMARY KEY, name_jp TEXT NOT NULL, user_id INTEGER)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO beekeeper (name_jp) VALUES ('')")
            .execute(&pool)
            .await
            .unwrap();
        let migration = include_str!(
            "../../../resources/db/migrations/0006_2026-10-10_beekeeper_name_constraints.sql"
        );
        let mut tx = pool.begin().await.unwrap();
        assert!(sqlx::raw_sql(migration).execute(&mut *tx).await.is_err());
        tx.rollback().await.unwrap();
        let name: (String,) = sqlx::query_as("SELECT name_jp FROM beekeeper")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(name.0, "");
    }

    #[tokio::test]
    async fn honey_repository_creation_reuses_normalized_beekeeper() {
        let (_dir, pool, _url) = database().await;
        let repository = HoneyRepositorySqlite { pool: pool.clone() };
        for honey_name in ["一番蜜", "二番蜜"] {
            let detail = serde_json::from_value(serde_json::json!({
                "basic": {
                    "name_jp": honey_name, "beekeeper_name": "  養蜂場  ",
                    "flower_names": [], "harvest_year": null, "country": null,
                    "region": null, "honey_type": null, "volume": null,
                    "purchase_date": null
                },
                "dynamic": [], "created_at": null, "updated_at": null
            }))
            .unwrap();
            repository.insert_honey(detail, 1).await.unwrap();
        }
        let beekeeper: (i32, String) =
            sqlx::query_as("SELECT id, name_jp FROM beekeeper WHERE user_id = 1")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(beekeeper.1, "養蜂場");
        let honey_count: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM honey WHERE beekeeper_id = ? AND user_id = 1")
                .bind(beekeeper.0)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(honey_count.0, 2);
    }

    #[tokio::test]
    async fn concurrent_creation_returns_one_shared_beekeeper() {
        let (_dir, pool, _url) = database().await;
        let create = || async {
            let mut connection = pool.acquire().await.unwrap();
            repository::beekeepers::get_or_insert_beekeeper_id(" 養蜂場 ", 1, &mut connection)
                .await
                .unwrap()
        };
        let (first, second) = tokio::join!(create(), create());
        assert_eq!(first, second);
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM beekeeper WHERE user_id = 1")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count.0, 1);
    }
}
