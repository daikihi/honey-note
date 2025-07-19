use crate::errors::AppError;
use crate::infrastructure::db::sqlx::beekeeper::BeekeeperForInsert;
use crate::models::beekeeper::Beekeeper as ModelBeekeeper;
use log::error;

pub async fn has_beekeeper(beekeeper: &ModelBeekeeper, pool: &sqlx::SqlitePool) -> bool {
    let inserted_beekeeper = BeekeeperForInsert::new(beekeeper);
    inserted_beekeeper
        .has_beekeeper(pool)
        .await
        .unwrap_or_else(|e| {
            error!("Error checking beekeeper existence: {}", e);
            false
        })
}

pub async fn insert_beekeeper(beekeeper: &ModelBeekeeper, pool: &sqlx::SqlitePool) {
    let inserted_beekeeper = BeekeeperForInsert::new(beekeeper);
    if let Err(e) = inserted_beekeeper.insert_beekeeper(pool).await {
        error!("Error inserting beekeeper: {}", e);
    }
}

pub async fn get_all_beekeepers(pool: &sqlx::SqlitePool) -> Result<Vec<ModelBeekeeper>, AppError> {
    let beekeepers = BeekeeperForInsert::get_all_beekeepers(pool).await;
    match beekeepers {
        Ok(beekeepers) => Ok(beekeepers
            .into_iter()
            .map(|b| ModelBeekeeper {
                id: Some(b.id),
                name_jp: b.name_jp,
                name_en: b.name_en,
                founding_year: b.founding_year,
                location_prefecture_id: b.location_prefecture_id,
                location_city: b.location_city,
                website_url: b.website_url,
                note: b.note,
            })
            .collect()),
        Err(e) => {
            error!("Error fetching all beekeepers: {}", e);
            Err(AppError::DatabaseError(e.to_string()))
        }
    }
}
