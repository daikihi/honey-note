use common_type::models::honey::Honey as ModelHoney;
use common_type::models::honey_detail::HoneyDetail;

use crate::infrastructure::db::sqlx::beekeeper::Beekeeper;
use crate::infrastructure::db::sqlx::honey::Honey;
use crate::infrastructure::db::sqlx::{beekeeper, honey};

use async_trait::async_trait;

#[async_trait]
pub trait HoneyRepository: Send + Sync {
    async fn insert_honey(&self, honey: HoneyDetail, user_id: i32) -> Result<i64, String>;
    async fn update_honey(&self, id: i64, honey: HoneyDetail, user_id: i32) -> Result<(), String>;
    async fn exists_honey(&self, honey: &HoneyDetail, user_id: i32) -> Result<bool, String>;
    async fn exists_honey_by_id(&self, id: i64, user_id: i32) -> Result<bool, String>;
    async fn get_all_honeys(&self, user_id: i32) -> Result<Vec<ModelHoney>, String>;
    async fn get_honey_by_id(&self, id: i64, user_id: i32) -> Result<HoneyDetail, String>;
}

pub struct HoneyRepositorySqlite {
    pub pool: sqlx::SqlitePool,
}

#[async_trait]
impl HoneyRepository for HoneyRepositorySqlite {
        async fn insert_honey(&self, honey: HoneyDetail, user_id: i32) -> Result<i64, String> {
        use crate::repository::{beekeepers as bk_repo, flowers::{FlowerRepository, FlowerRepositorySqlite}};
        use common_type::models::flowers::create_model_flower_from_name;

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        // 1) 養蜂場IDの解決（なければ作成）
        let mut beekeeper_id_opt: Option<i32> = None;
        if let Some(bk_name) = honey.basic.beekeeper_name.clone() {
            let name = bk_name.0.trim();
            if !name.is_empty() {
                beekeeper_id_opt = Some(
                    bk_repo::get_or_insert_beekeeper_id(name, user_id, &mut *tx)
                        .await
                        .map_err(|e| e.to_string())?,
                );
            }
        }

        // 2) はちみつ本体のINSERT
        let sqlx_honey = Honey {
            id: None,
            name_jp: honey.basic.name_jp.0.clone(),
            name_en: None,
            beekeeper_id: beekeeper_id_opt,
            user_id: Some(user_id),
            origin_country: honey.basic.country.map(|c| c.0),
            origin_region: honey.basic.region.map(|r| r.0),
            harvest_year: honey.basic.harvest_year,
            purchase_date: honey.basic.purchase_date.map(|d| d.to_rfc3339()),
            note: None,
        };

        let honey_id = sqlx_honey
            .insert_and_return_id(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        // 3) 蜜源（flower）との関連INSERT（なければ作成→関連付け）
        for fname in &honey.basic.flower_names {
            let name = fname.0.clone();
            if name.is_empty() { continue; }

            // Flower ID解決
            let mut flower_id_opt = {
                let row: Result<(i32,), sqlx::Error> = sqlx::query_as("SELECT id FROM flower WHERE name_jp = $1")
                    .bind(&name)
                    .fetch_one(&mut *tx)
                    .await;
                match row { Ok((id,)) => Some(id), Err(_) => None }
            };

            if flower_id_opt.is_none() {
                let model = create_model_flower_from_name(&name);
                let fl_repo_inst = FlowerRepositorySqlite { pool: self.pool.clone() };
                match fl_repo_inst.has_flower(&model, user_id, &mut *tx).await {
                    Ok(false) => {
                        fl_repo_inst.insert_flower(&model, user_id, &mut *tx).await.map_err(|e| e.to_string())?;
                    }
                    _ => {}
                }
                let row: Result<(i32,), sqlx::Error> = sqlx::query_as("SELECT id FROM flower WHERE name_jp = $1")
                    .bind(&name)
                    .fetch_one(&mut *tx)
                    .await;
                if let Ok((id,)) = row { flower_id_opt = Some(id); }
            }

            if let Some(flower_id) = flower_id_opt {
                let exists: (i64,) = sqlx::query_as("SELECT COUNT(1) FROM honey_flower WHERE honey_id = $1 AND flower_id = $2")
                    .bind(honey_id)
                    .bind(flower_id)
                    .fetch_one(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                if exists.0 == 0 {
                    sqlx::query("INSERT INTO honey_flower (honey_id, flower_id) VALUES ($1, $2)")
                        .bind(honey_id)
                        .bind(flower_id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;
                }
            }
        }

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(honey_id)
    }

    /// 所有者を確認し、蜂蜜本体と養蜂場・蜜源の関連を同じ取引で更新する。
    async fn update_honey(&self, id: i64, honey: HoneyDetail, user_id: i32) -> Result<(), String> {
        use crate::repository::beekeepers as bk_repo;
        use crate::repository::flowers::{FlowerRepository, FlowerRepositorySqlite};
        use common_type::models::flowers::create_model_flower_from_name;
        use std::collections::HashSet;

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        // 所有権チェック
        let exists = sqlx::query("SELECT 1 FROM honey WHERE id = ? AND user_id = ?")
            .bind(id as i32)
            .bind(user_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        
        if exists.is_none() {
            return Err("指定されたハチミツが見つからないか、権限がありません".to_string());
        }

        let beekeeper_name = honey
            .basic
            .beekeeper_name
            .as_ref()
            .map(|name| name.0.trim())
            .filter(|name| !name.is_empty());
        let beekeeper_id = if let Some(name) = beekeeper_name {
            Some(
                bk_repo::get_or_insert_beekeeper_id(name, user_id, &mut *tx)
                    .await
                    .map_err(|e| e.to_string())?,
            )
        } else {
            None
        };

        let mut flower_ids = Vec::new();
        let mut seen_flower_names = HashSet::new();
        for flower_name in &honey.basic.flower_names {
            let name = flower_name.0.trim();
            if name.is_empty() || !seen_flower_names.insert(name.to_string()) {
                continue;
            }

            let existing: Option<(i32,)> = sqlx::query_as(
                "SELECT id FROM flower WHERE name_jp = ? AND user_id = ? ORDER BY id LIMIT 1",
            )
            .bind(name)
            .bind(user_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

            let flower_id = match existing {
                Some((flower_id,)) => flower_id,
                None => {
                    let model = create_model_flower_from_name(name);
                    FlowerRepositorySqlite {
                        pool: self.pool.clone(),
                    }
                    .insert_flower(&model, user_id, &mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                    let (flower_id,): (i32,) = sqlx::query_as("SELECT last_insert_rowid()")
                        .fetch_one(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;
                    flower_id
                }
            };
            flower_ids.push(flower_id);
        }

        sqlx::query(
            "UPDATE honey SET name_jp = ?, beekeeper_id = ?, origin_country = ?, origin_region = ?, harvest_year = ?, purchase_date = ? WHERE id = ? AND user_id = ?",
        )
            .bind(&honey.basic.name_jp.0)
            .bind(beekeeper_id)
            .bind(honey.basic.country.map(|country| country.0))
            .bind(honey.basic.region.map(|region| region.0))
            .bind(honey.basic.harvest_year)
            .bind(honey.basic.purchase_date.map(|date| date.to_rfc3339()))
            .bind(id as i32)
            .bind(user_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query("DELETE FROM honey_flower WHERE honey_id = ?")
            .bind(id as i32)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        for flower_id in flower_ids {
            sqlx::query("INSERT INTO honey_flower (honey_id, flower_id) VALUES (?, ?)")
                .bind(id as i32)
                .bind(flower_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn exists_honey(&self, honey: &HoneyDetail, user_id: i32) -> Result<bool, String> {
        let name = &honey.basic.name_jp.0;
        let query = "SELECT EXISTS(SELECT 1 FROM honey WHERE name_jp = ? AND user_id = ?)";
        let result: (i64,) = sqlx::query_as(query)
            .bind(name)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(result.0 != 0)
    }

    async fn exists_honey_by_id(&self, id: i64, user_id: i32) -> Result<bool, String> {
        let query = "SELECT EXISTS(SELECT 1 FROM honey WHERE id = ? AND user_id = ?)";
        let result: (i64,) = sqlx::query_as(query)
            .bind(id as i32)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(result.0 != 0)
    }

    async fn get_all_honeys(&self, user_id: i32) -> Result<Vec<ModelHoney>, String> {
        let sql_honeys: Result<Vec<Honey>, sqlx::Error> = sqlx::query_as("SELECT * FROM honey WHERE user_id = ?")
            .bind(user_id)
            .fetch_all(&self.pool)
            .await;
        let sql_beekeepers: Result<Vec<beekeeper::Beekeeper>, sqlx::Error> =
            beekeeper::Beekeeper::get_all_beekeepers(&self.pool).await; // 養蜂業者は共通マスターまたは全取得で一旦妥協
        let flower_names = get_honey_flower_names(&self.pool, Some(user_id)).await;

        match (sql_honeys, sql_beekeepers, flower_names) {
            (Ok(v), Ok(bks), Ok(flowers)) => Ok(create_model_honeys(v, bks, flowers)),
            (Err(e), _, _) => Err(format!("database error (honeys): {:?}", e)),
            (_, Err(e), _) => Err(format!("database error (beekeepers): {:?}", e)),
            (_, _, Err(e)) => Err(format!("database error (honey flowers): {:?}", e)),
        }
    }

    async fn get_honey_by_id(&self, id: i64, user_id: i32) -> Result<HoneyDetail, String> {
        use chrono::{DateTime, FixedOffset};
        use common_type::models::honey_detail::HoneyDetail;
        use common_type::models::honey_detail_basic::HoneyDetailBasic;
        use common_type::models::honey_detail_dynamic::HoneyDetailDynamic;
        use common_type::models::honey_detail_types::{BeekeeperName, Country, FlowerName, HoneyNameJp, HoneyType, Region, Volume};

        // 1) honey行を取得
        let row: Result<Honey, sqlx::Error> = sqlx::query_as::<_, Honey>(
            r#"SELECT id, name_jp, name_en, beekeeper_id, user_id, origin_country, origin_region, harvest_year, purchase_date, note FROM honey WHERE id = ? AND user_id = ?"#
        )
        .bind(id as i32)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await;

        let h = row.map_err(|e| format!("NoSuchHoneyIdExistOrNoPermission: {:?}", e))?;

        // 2) 養蜂場名の解決
        let beekeeper_name_opt: Option<BeekeeperName> = match h.beekeeper_id {
            Some(bk_id) => {
                let r: Result<(String,), sqlx::Error> = sqlx::query_as("SELECT name_jp FROM beekeeper WHERE id = ?")
                    .bind(bk_id)
                    .fetch_one(&self.pool)
                    .await;
                match r { Ok((name,)) => Some(BeekeeperName(name)), Err(_) => None }
            }
            None => None,
        };

        // 3) 花名一覧の取得
        let flower_rows: Result<Vec<(String,)>, sqlx::Error> = sqlx::query_as(
            r#"SELECT f.name_jp FROM honey_flower hf JOIN flower f ON f.id = hf.flower_id WHERE hf.honey_id = ?"#
        )
        .bind(h.id.unwrap_or_default())
        .fetch_all(&self.pool)
        .await;
        let flower_names: Vec<FlowerName> = match flower_rows {
            Ok(v) => v.into_iter().map(|(n,)| FlowerName(n)).collect(),
            Err(_) => vec![],
        };

        // 4) 購入日の変換（RFC3339 → DateTime<FixedOffset>）
        let purchase_dt: Option<DateTime<FixedOffset>> = match h.purchase_date {
            Some(ref s) => chrono::DateTime::parse_from_rfc3339(s).ok(),
            None => None,
        };

        // 5) HoneyDetail 構築（dynamicは未保存のため空）
        let basic = HoneyDetailBasic {
            name_jp: HoneyNameJp(h.name_jp.clone()),
            beekeeper_name: beekeeper_name_opt,
            harvest_year: h.harvest_year,
            country: h.origin_country.clone().map(Country),
            region: h.origin_region.clone().map(Region),
            flower_names,
            honey_type: None::<HoneyType>,
            volume: None::<Volume>,
            purchase_date: purchase_dt,
        };
        let detail = HoneyDetail {
            basic,
            dynamic: Vec::<HoneyDetailDynamic>::new(),
            created_at: None,
            updated_at: None,
        };
        Ok(detail)
    }
}

pub struct HoneyRepositoryMock;

#[async_trait]
impl HoneyRepository for HoneyRepositoryMock {
    async fn insert_honey(&self, _honey: HoneyDetail, _user_id: i32) -> Result<i64, String> {
        Ok(1)
    }
    async fn update_honey(&self, _id: i64, _honey: HoneyDetail, _user_id: i32) -> Result<(), String> {
        Ok(())
    }
    async fn exists_honey(&self, _honey: &HoneyDetail, _user_id: i32) -> Result<bool, String> {
        Ok(false) // 常に新規として扱う（テスト用）
    }
    async fn exists_honey_by_id(&self, _id: i64, _user_id: i32) -> Result<bool, String> {
        Ok(true)
    }
    async fn get_all_honeys(&self, _user_id: i32) -> Result<Vec<ModelHoney>, String> {
        Ok(vec![])
    }
    async fn get_honey_by_id(&self, id: i64, _user_id: i32) -> Result<HoneyDetail, String> {
        use common_type::models::honey_detail::HoneyDetail;
        use common_type::models::honey_detail_basic::HoneyDetailBasic;
        use common_type::models::honey_detail_dynamic::HoneyDetailDynamic;
        use common_type::models::honey_detail_types::{HoneyNameJp};
        let basic = HoneyDetailBasic {
            name_jp: HoneyNameJp(format!("Mock Honey {}", id)),
            beekeeper_name: None,
            harvest_year: None,
            country: None,
            region: None,
            flower_names: vec![],
            honey_type: None,
            volume: None,
            purchase_date: None,
        };
        Ok(HoneyDetail { basic, dynamic: Vec::<HoneyDetailDynamic>::new(), created_at: None, updated_at: None })
    }
}

pub async fn insert_honey_if_not_exists<'a, E>(
    honey: &ModelHoney,
    user_id: i32,
    executor: E,
) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let sqlx_honey = Honey {
        id: honey.id,
        name_jp: honey.name_jp.clone(),
        name_en: honey.name_en.clone(),
        beekeeper_id: honey.beekkeeper.clone().map(|b| b.id).flatten(),
        user_id: Some(user_id),
        origin_country: honey.origin_country.clone(),
        origin_region: honey.origin_region.clone(),
        harvest_year: honey.harvest_year,
        purchase_date: honey.purchase_date.clone(),
        note: honey.note.clone(),
    };
    log::info!("honey {:?} は、DB に書き込みます(重複チェック付き)", sqlx_honey.name_jp);
    sqlx_honey.insert_and_return_id_with_check(executor).await.map(|_| ())
}

pub async fn get_all_honies(pool: &sqlx::SqlitePool) -> Vec<ModelHoney> {
    let sql_honeys: Result<Vec<Honey>, sqlx::Error> = honey::get_all(pool).await;
    let sql_beekeepers: Result<Vec<beekeeper::Beekeeper>, sqlx::Error> =
        beekeeper::Beekeeper::get_all_beekeepers(pool).await;
    let flower_names = get_honey_flower_names(pool, None).await;
    match (sql_honeys, sql_beekeepers, flower_names) {
        (Ok(honeys), Ok(beekeepers), Ok(flowers)) => {
            create_model_honeys(honeys, beekeepers, flowers)
        }
        (Err(e), _, _) | (_, Err(e), _) | (_, _, Err(e)) => {
            log::error!("database error! {:?}", e);
            vec![]
        }
    }
}

async fn get_honey_flower_names(
    pool: &sqlx::SqlitePool,
    user_id: Option<i32>,
) -> Result<std::collections::HashMap<i32, Vec<String>>, sqlx::Error> {
    let rows: Vec<(i32, String)> = if let Some(user_id) = user_id {
        sqlx::query_as(
            "SELECT hf.honey_id, f.name_jp FROM honey_flower hf \
             JOIN honey h ON h.id = hf.honey_id \
             JOIN flower f ON f.id = hf.flower_id \
             WHERE h.user_id = ? AND f.user_id = ? \
             ORDER BY hf.honey_id, f.name_jp",
        )
        .bind(user_id)
        .bind(user_id)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as(
            "SELECT hf.honey_id, f.name_jp FROM honey_flower hf \
             JOIN flower f ON f.id = hf.flower_id \
             ORDER BY hf.honey_id, f.name_jp",
        )
        .fetch_all(pool)
        .await?
    };

    let mut flower_names = std::collections::HashMap::new();
    for (honey_id, name) in rows {
        flower_names.entry(honey_id).or_insert_with(Vec::new).push(name);
    }
    Ok(flower_names)
}

fn create_model_honeys(
    sql_honeyies: Vec<Honey>,
    sql_bk: Vec<Beekeeper>,
    flower_names: std::collections::HashMap<i32, Vec<String>>,
) -> Vec<ModelHoney> {
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
            let honey_id = h.id.unwrap_or_default();
            let names = flower_names.get(&honey_id).cloned().unwrap_or_default();
            sqlx_honey_package::create_model_honey(h.clone(), bk_opt, names)
        })
        .collect();
    model_honeyies
}

#[cfg(test)]
mod honey_flower_list_tests {
    use super::get_honey_flower_names;

    #[tokio::test]
    async fn honey_flower_names_are_loaded_for_the_requested_user() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query("CREATE TABLE honey (id INTEGER PRIMARY KEY, user_id INTEGER NOT NULL)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE flower (id INTEGER PRIMARY KEY, name_jp TEXT NOT NULL, user_id INTEGER NOT NULL)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE honey_flower (honey_id INTEGER NOT NULL, flower_id INTEGER NOT NULL)")
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO honey (id, user_id) VALUES (10, 1), (20, 2)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO flower (id, name_jp, user_id) VALUES (100, 'アカシア', 1), (200, 'レンゲ', 2)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO honey_flower (honey_id, flower_id) VALUES (10, 100), (20, 200)")
            .execute(&pool)
            .await
            .unwrap();

        let names = get_honey_flower_names(&pool, Some(1)).await.unwrap();
        assert_eq!(names.get(&10), Some(&vec!["アカシア".to_string()]));
        assert!(!names.contains_key(&20));
    }
}
