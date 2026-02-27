use common_type::models::honey::Honey as ModelHoney;
use common_type::models::honey_detail::HoneyDetail;

use crate::infrastructure::db::sqlx::beekeeper::Beekeeper;
use crate::infrastructure::db::sqlx::honey::Honey;
use crate::infrastructure::db::sqlx::{beekeeper, honey};

pub trait HoneyRepository: Send + Sync {
    async fn insert_honey(&self, honey: HoneyDetail) -> Result<i64, String>;
    async fn update_honey(&self, id: i64, honey: HoneyDetail) -> Result<(), String>;
    async fn exists_honey(&self, honey: &HoneyDetail) -> Result<bool, String>;
    async fn exists_honey_by_id(&self, id: i64) -> Result<bool, String>;
    async fn get_all_honeys(&self) -> Result<Vec<ModelHoney>, String>;
}

pub struct HoneyRepositorySqlite {
    pub pool: sqlx::SqlitePool,
}

impl HoneyRepository for HoneyRepositorySqlite {
    async fn insert_honey(&self, honey: HoneyDetail) -> Result<i64, String> {
        let sqlx_honey = Honey {
            id: None,
            name_jp: honey.basic.name_jp.0.clone(),
            name_en: None, // HoneyDetailBasicにname_enがないため
            beekeeper_id: None, // 名前からIDを引く必要があるが、一旦None
            origin_country: honey.basic.country.map(|c| c.0),
            origin_region: honey.basic.region.map(|r| r.0),
            harvest_year: honey.basic.harvest_year,
            purchase_date: honey.basic.purchase_date.map(|d| d.to_rfc3339()),
            note: None,
        };

        sqlx_honey
            .insert_and_return_id(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn update_honey(&self, id: i64, honey: HoneyDetail) -> Result<(), String> {
        let sqlx_honey = Honey {
            id: Some(id as i32),
            name_jp: honey.basic.name_jp.0.clone(),
            name_en: None,
            beekeeper_id: None,
            origin_country: honey.basic.country.map(|c| c.0),
            origin_region: honey.basic.region.map(|r| r.0),
            harvest_year: honey.basic.harvest_year,
            purchase_date: honey.basic.purchase_date.map(|d| d.to_rfc3339()),
            note: None,
        };

        sqlx_honey
            .update(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn exists_honey(&self, honey: &HoneyDetail) -> Result<bool, String> {
        Honey::is_exist_by_name_static(&honey.basic.name_jp.0, &self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn exists_honey_by_id(&self, id: i64) -> Result<bool, String> {
        Honey::is_exist_by_id_static(id as i32, &self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn get_all_honeys(&self) -> Result<Vec<ModelHoney>, String> {
        let sql_honeys: Result<Vec<Honey>, sqlx::Error> = honey::get_all(&self.pool).await;
        let sql_beekeepers: Result<Vec<beekeeper::Beekeeper>, sqlx::Error> =
            beekeeper::Beekeeper::get_all_beekeepers(&self.pool).await;

        match (sql_honeys, sql_beekeepers) {
            (Ok(v), Ok(bks)) => Ok(create_model_honeys(v, bks)),
            (Err(e), _) => Err(format!("database error (honeys): {:?}", e)),
            (_, Err(e)) => Err(format!("database error (beekeepers): {:?}", e)),
        }
    }
}

pub struct HoneyRepositoryMock;

impl HoneyRepository for HoneyRepositoryMock {
    async fn insert_honey(&self, _honey: HoneyDetail) -> Result<i64, String> {
        Ok(1)
    }
    async fn update_honey(&self, _id: i64, _honey: HoneyDetail) -> Result<(), String> {
        Ok(())
    }
    async fn exists_honey(&self, _honey: &HoneyDetail) -> Result<bool, String> {
        Ok(false) // 常に新規として扱う（テスト用）
    }
    async fn exists_honey_by_id(&self, _id: i64) -> Result<bool, String> {
        Ok(true)
    }
    async fn get_all_honeys(&self) -> Result<Vec<ModelHoney>, String> {
        Ok(vec![])
    }
}

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
    match Honey::is_exist_by_name_static(&sqlx_honey.name_jp, pool).await {
        Ok(true) => {
            log::info!("honey {:?} は既に存在しています。", sqlx_honey.name_jp);
            Ok(())
        }
        Ok(false) => {
            log::info!("honey {:?} は、DB に書き込みます", sqlx_honey.name_jp);
            sqlx_honey.insert_and_return_id(pool).await.map(|_| ())
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
            let bk_opt = bk_vec.first().map(|&b| (*b).clone());
            use crate::infrastructure::db::sqlx::honey as sqlx_honey_package;
            sqlx_honey_package::create_model_honey(h.clone(), bk_opt)
        })
        .collect();
    model_honeyies
}
