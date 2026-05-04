use common::repository;
use common::repository::beekeepers::BeekeeperRepository;
use common_type::models::beekeeper::Beekeeper;

use crate::honey_loader_models::JsonHoney;
use crate::{honey_loader_gateway, honey_loader_request};

pub async fn run(request_dto: honey_loader_request::HoneyLoaderRequestDto<'_>, user_id: i32) {
    log::info!(
        "honey_loader リクエスト: {:?}, user_id={}",
        request_dto,
        user_id
    );

    // ハチミツデータの読み込みとフィルタリング
    let non_empty_honeys = load_and_filter_honeys(request_dto.file_name).await;

    // データベース接続
    let pool = sqlx::SqlitePool::connect(&request_dto.db_file_name)
        .await
        .expect("データベース接続に失敗しました");

    let mut tx = pool.begin().await.expect("データベース接続に失敗しました");

    // 養蜂家の処理
    process_and_insert_beekeepers(&non_empty_honeys, user_id, &pool, &mut tx).await;

    // ハチミツの処理
    process_and_insert_honeys(&non_empty_honeys, user_id, &pool, &mut tx).await;

    // トランザクションをコミット
    tx.commit().await.expect("Failed to commit transaction");
    log::info!("ハニーのデータベースへの挿入が完了しました");
}

/// JSONファイルからハチミツデータを読み込み、空のレコードを除外する
async fn load_and_filter_honeys(file_name: &str) -> Vec<JsonHoney> {
    let json_honeys: Vec<JsonHoney> = honey_loader_gateway::run(file_name);
    log::info!(
        "honey_loader 完了: {} 件のハニーを読み込みました",
        json_honeys.len()
    );

    let non_empty_honeys: Vec<JsonHoney> = json_honeys
        .into_iter()
        .filter(|honey| {
            let name = honey.name.clone();
            name.or_else(|| Some("".to_string())) != Some("".to_string())
        })
        .collect();
    log::info!("非空のハニー: {} 件", non_empty_honeys.len());

    non_empty_honeys
}

/// 養蜂家データを処理し、データベースに挿入する
async fn process_and_insert_beekeepers(
    honeys: &[JsonHoney],
    user_id: i32,
    pool: &sqlx::SqlitePool,
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
) {
    let beekeepers = honeys
        .iter()
        .filter_map(|json_honey| json_honey.beekeeper.clone())
        .filter(|b| b != &"".to_string())
        .collect::<Vec<_>>();

    for beekeeper in beekeepers {
        let bk = Beekeeper {
            id: None,
            name_jp: beekeeper.to_string(),
            name_en: None,
            founding_year: None,
            location_prefecture_id: None,
            location_city: None,
            website_url: None,
            note: None,
        };

        let bk_repo =
            common::repository::beekeepers::BeekeeperRepositorySqlite { pool: pool.clone() };
        let is_exist = bk_repo.has_beekeeper(&bk, user_id, &mut **tx).await;

        if !is_exist {
            let _ = bk_repo.insert_beekeeper(&bk, user_id, &mut **tx).await;
            log::info!("養蜂家をデータベースに挿入: {:?}", beekeeper);
        } else {
            log::info!("養蜂家は既に存在します: {:?}", beekeeper);
        }
    }
}

/// ハチミツデータを処理し、データベースに挿入する
async fn process_and_insert_honeys(
    honeys: &[JsonHoney],
    user_id: i32,
    pool: &sqlx::SqlitePool,
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
) {
    let mut model_honeys: Vec<common_type::models::honey::Honey> = Vec::new();

    for json_honey in honeys {
        let bk_repo =
            common::repository::beekeepers::BeekeeperRepositorySqlite { pool: pool.clone() };
        let beekeeper_id = bk_repo
            .get_beekeeper_id_by_name(
                &json_honey.beekeeper.clone().unwrap_or_default(),
                user_id,
                &mut **tx,
            )
            .await;

        model_honeys.push(common_type::models::honey::Honey {
            id: Some(json_honey.id),
            name_jp: json_honey.name.clone().unwrap_or_default(),
            name_en: None,
            beekkeeper: json_honey.beekeeper.clone().map(|b| {
                common_type::models::beekeeper::Beekeeper {
                    id: beekeeper_id,
                    name_jp: b,
                    name_en: None,
                    founding_year: None,
                    location_prefecture_id: None,
                    location_city: None,
                    website_url: None,
                    note: None,
                }
            }),
            origin_country: json_honey.country.clone(),
            origin_region: json_honey.prefecture.clone(),
            harvest_year: None,
            purchase_date: None,
            note: None,
        });
    }

    for model_honey in model_honeys {
        let _ =
            repository::honeys::insert_honey_if_not_exists(&model_honey, user_id, &mut **tx).await;
        log::info!("ハニーをデータベースに挿入: {:?}", model_honey);
    }
}
