use common_type::models::honey::Honey as ModelHoney;

use crate::infrastructure::db::sqlx::honey::Honey;

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
