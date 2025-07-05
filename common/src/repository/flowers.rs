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


// ここから下はテストコード
#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    use super::*;
    use crate::{infrastructure::db::sqlx::get_sqlite_pool, models::flowers::create_model_flower_from_name};

    // Create database and table for testing
    async fn setup_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::query(
                "CREATE TABLE IF NOT EXISTS flower (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name_jp TEXT NOT NULL,
                name_en TEXT,
                scientific_name TEXT,
                short_note TEXT,       -- description に相当する簡単な説明
                flower_type TEXT,
                image_path TEXT,
                note TEXT
                );
            ",
        )
        .execute(&pool)
        .await
        .unwrap();
        pool
    }

    #[tokio::test]
    async fn test_insert_flower() {
        let pool = setup_db().await;
        let flower = create_model_flower_from_name("Test Flower");
        let result = insert_flower(&flower, &pool).await;

        assert!(result.is_ok(), "Failed to insert flower: {:?}", result.err());


        let has_flower = has_flower(&flower, &pool).await;
        assert!(has_flower.is_ok(), "Failed to check if flower exists: {:?}", has_flower.err());
        assert!(has_flower.unwrap(), "Flower should exist after insertion");
    }

    #[tokio::test]
    async fn test_insert_flower_duplicate() {
        let pool = setup_db().await;
        let flower = create_model_flower_from_name("Duplicate Flower");

        // Insert the flower for the first time
        let result = insert_flower(&flower, &pool).await;
        assert!(result.is_ok(), "Failed to insert flower: {:?}", result.err());

        // Try to insert the same flower again
        let duplicate_result = insert_flower(&flower, &pool).await;
        assert!(duplicate_result.is_ok(), "Should not allow duplicate insertion");

        let has_flower = has_flower(&flower, &pool).await;
        assert!(has_flower.is_ok(), "Failed to check if flower exists: {:?}", has_flower.err());
        assert!(has_flower.unwrap(), "Flower should still exist after duplicate insertion");

        // TODO: after implementing get_flower_by_name, check if the flower can be retrieved
    }
}