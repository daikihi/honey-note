use common_type::models::beekeeper::Beekeeper as ModelBeekeeper;
use common_type::models::honey::Honey as ModelHoney;

use crate::infrastructure::db::sqlx::beekeeper::Beekeeper;
use crate::infrastructure::db::sqlx::honey::Honey;
use crate::infrastructure::db::sqlx::{beekeeper, honey};

pub async fn insert_honey_if_not_exists(
    honey: &ModelHoney,
    pool: &sqlx::SqlitePool,
) -> Result<(), sqlx::Error> {
    let sqlx_honey = Honey {
        id: honey.id,
        name_jp: honey.name_jp.clone(),
        name_en: honey.name_en.clone(),
        beekeeper_id: honey.beekkeeper.clone().map(|b| b.id).flatten(),
        origin_country: honey.origin_country.clone(),
        origin_region: honey.origin_region.clone(),
        harvest_year: honey.harvest_year,
        purchase_date: honey.purchase_date.clone(),
        note: honey.note.clone(),
    };
    match sqlx_honey.is_exist_by_name(pool).await {
        Ok(true) => {
            log::info!("honey {:?} は既に存在しています。", sqlx_honey.name_jp);
            Ok(())
        }
        Ok(false) => {
            log::info!("honey {:?} は、DB に書き込みます", sqlx_honey.name_jp);
            sqlx_honey.insert(pool).await
        }
        Err(e) => {
            log::error!("DB 読み込みに失敗しました");
            Err(e)
        }
    }
}

pub async fn get_all_honies(pool: &sqlx::SqlitePool) -> Vec<ModelHoney> {
    let sql_honeys: Result<Vec<Honey>, sqlx::Error> = honey::get_all(pool).await;
    let sql_beekeepers: Result<Vec<beekeeper::Beekeeper>, sqlx::Error> =
        beekeeper::Beekeeper::get_all_beekeepers(pool).await;
    match sql_honeys {
        Ok(v) => match sql_beekeepers {
            Ok(bks) => create_model_honeys(v, bks),
            Err(e) => {
                log::error!("database error! {:?}", e);
                vec![]
            }
        },
        Err(e) => {
            log::error!("database error! {:?}", e);
            vec![]
        }
    }
}

fn create_model_honeys(sql_honeyies: Vec<Honey>, sql_bk: Vec<Beekeeper>) -> Vec<ModelHoney> {
    let model_honeyies: Vec<ModelHoney> = sql_honeyies
        .iter()
        .map(|h| {
            let bk_vec: Vec<&Beekeeper> = sql_bk
                .iter()
                .filter(|b| {
                    let bk_id: Option<i32> = h.clone().beekeeper_id;
                    match bk_id {
                        Some(id) => b.id == id,
                        None => false,
                    }
                })
                .collect();
            let bk_opt = bk_vec.first().map(|b| b.clone());
            use crate::infrastructure::db::sqlx::honey as sqlx_honey_package;
            sqlx_honey_package::create_model_honey(h.clone(), bk_opt.cloned())
        })
        .collect();
    model_honeyies
}
