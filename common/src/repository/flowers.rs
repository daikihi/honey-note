use log::info;

use crate::models::flowers::Flower;

pub async fn insert_flower(flower: &Flower, pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    info!("insert_flower: flower={:?}", flower);
    let insert_flower =
        crate::infrastructure::db::sqlx::flower::InsertFlower::from_model_flower(flower);
    insert_flower.insert_flower(pool).await.map_err(|e| {
        eprintln!("Error inserting flower: {}", e);
        e
    })
}

pub async fn has_flower(flower: &Flower, pool: &sqlx::SqlitePool) -> Result<bool, sqlx::Error> {
    // flower.has_flower(pool)
    let insert_flower =
        crate::infrastructure::db::sqlx::flower::InsertFlower::from_model_flower(flower);
    insert_flower.has_flower(pool).await.map_err(|e| {
        eprintln!("Error checking if flower exists: {}", e);
        e
    })
}
