use common::repository;
use common_type::models::beekeeper::Beekeeper;

use crate::honey_loader_models::JsonHoney;
use crate::{honey_loader_gateway, honey_loader_request};
use futures::stream::{self, StreamExt}; // 必要なライブラリをインポート

pub async fn run(request_dto: honey_loader_request::HoneyLoaderRequestDto<'_>) {
    // ログ出力
    log::info!("honey_loader リクエスト: {:?}", request_dto);
    let _json_honeys: Vec<JsonHoney> = honey_loader_gateway::run(request_dto.file_name);
    log::info!(
        "honey_loader 完了: {} 件のハニーを読み込みました",
        _json_honeys.len()
    );
    let _non_empty_honeys: Vec<JsonHoney> = _json_honeys
        .into_iter()
        .filter(|honey| {
            let _name = honey.name.clone();
            _name.or_else(|| Some("".to_string())) != Some("".to_string())
        })
        .collect();
    log::info!("非空のハニー: {} 件", _non_empty_honeys.len());
    let pool = sqlx::SqlitePool::connect(&request_dto.db_file_name)
        .await
        .expect("データベース接続に失敗しました");

    let beekeepers = _non_empty_honeys
        .iter()
        .filter_map(|json_honey| json_honey.beekeeper.clone())
        .filter(|b| b != &"".to_string())
        .collect::<Vec<_>>();

    for beekeeper in beekeepers {
        let bk = &Beekeeper {
            id: None,
            name_jp: beekeeper.to_string(),
            name_en: None,
            founding_year: None,
            location_prefecture_id: None,
            location_city: None,
            website_url: None,
            note: None,
        };

        let is_exist = common::repository::beekeepers::has_beekeeper(bk, &pool).await;

        if !is_exist {
            common::repository::beekeepers::insert_beekeeper(bk, &pool).await;
            log::info!("養蜂家をデータベースに挿入: {:?}", beekeeper);
        } else {
            log::info!("養蜂家は既に存在します: {:?}", beekeeper);
        }
    }

    let model_honeys: Vec<common_type::models::honey::Honey> =
        stream::iter(_non_empty_honeys.iter())
            .then(|json_honey| {
                let pool_clone = pool.clone(); // `pool` のクローンを作成
                async move {
                    let beekeeper_id = common::repository::beekeepers::get_beekeeper_id_by_name(
                        &json_honey.beekeeper.clone().unwrap_or_default(),
                        &pool_clone, // クローンを使用
                    )
                    .await;

                    common_type::models::honey::Honey {
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
                    }
                }
            })
            .collect::<Vec<_>>() // Vecに収集
            .await;

    for model_honey in model_honeys {
        let _ = repository::honeys::insert_honey_if_not_exists(&model_honey, &pool).await;
        log::info!("ハニーをデータベースに挿入: {:?}", model_honey);
    }
    log::info!("ハニーのデータベースへの挿入が完了しました");
}
