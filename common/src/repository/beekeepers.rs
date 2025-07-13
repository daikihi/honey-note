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
