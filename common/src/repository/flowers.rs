use log::info;

use crate::errors::AppError;
use common_type::models::flowers::Flower as ModelFlower;

pub trait FlowerRepository: Send + Sync {
    async fn get_all_flowers(&self) -> Result<Vec<ModelFlower>, AppError>;
    async fn get_flower_by_id(&self, id: i32) -> Result<ModelFlower, AppError>;
    async fn update_flower<'a, E>(
        &self,
        id: i32,
        flower: &ModelFlower,
        executor: E,
    ) -> Result<(), AppError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>;
    async fn exists_flower_by_id<'a, E>(&self, id: i32, executor: E) -> Result<bool, AppError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>;
    async fn insert_flower<'a, E>(&self, flower: &ModelFlower, executor: E) -> Result<(), AppError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>;
    async fn has_flower<'a, E>(&self, flower: &ModelFlower, executor: E) -> Result<bool, AppError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>;
    async fn get_flower_id_by_name<'a, E>(&self, name: &str, executor: E) -> Option<i32>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>;
}

pub struct FlowerRepositorySqlite {
    pub pool: sqlx::SqlitePool,
}

impl FlowerRepository for FlowerRepositorySqlite {
    async fn get_all_flowers(&self) -> Result<Vec<ModelFlower>, AppError> {
        let db_flowers = crate::infrastructure::db::sqlx::flower::get_all_flowers(&self.pool).await;
        match db_flowers {
            Ok(flowers) => Ok(flowers
                .iter()
                .map(|f| f.to_model_flower())
                .collect::<Vec<ModelFlower>>()),
            Err(e) => Err(AppError::DatabaseError(e.to_string())),
        }
    }

    async fn get_flower_by_id(&self, id: i32) -> Result<ModelFlower, AppError> {
        let result = crate::infrastructure::db::sqlx::flower::Flower::get_flower_by_id(id, &self.pool).await;
        match result {
            Ok(f) => Ok(f.to_model_flower()),
            Err(e) => Err(AppError::DatabaseError(e.to_string())),
        }
    }

    async fn update_flower<'a, E>(
        &self,
        id: i32,
        flower: &ModelFlower,
        executor: E,
    ) -> Result<(), AppError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
    {
        let sqlx_flower = crate::infrastructure::db::sqlx::flower::Flower {
            id: Some(id),
            name_jp: flower.name_jp.clone(),
            name_en: flower.name_en.clone(),
            scientific_name: flower.scientific_name.clone(),
            short_note: flower.short_note.clone(),
            flower_type: flower.flower_type.clone(),
            image_path: flower.image_path.clone(),
            note: flower.note.clone(),
        };
        sqlx_flower
            .update(executor)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    async fn exists_flower_by_id<'a, E>(&self, id: i32, executor: E) -> Result<bool, AppError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
    {
        crate::infrastructure::db::sqlx::flower::Flower::exists_flower_by_id(id, executor)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    async fn insert_flower<'a, E>(&self, flower: &ModelFlower, executor: E) -> Result<(), AppError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
    {
        info!("insert_flower: flower={:?}", flower);
        let insert_flower =
            crate::infrastructure::db::sqlx::flower::InsertFlower::from_model_flower(flower);
        insert_flower
            .insert_flower(executor)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    async fn has_flower<'a, E>(&self, flower: &ModelFlower, executor: E) -> Result<bool, AppError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
    {
        let insert_flower =
            crate::infrastructure::db::sqlx::flower::InsertFlower::from_model_flower(flower);
        insert_flower
            .has_flower(executor)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    async fn get_flower_id_by_name<'a, E>(&self, name: &str, executor: E) -> Option<i32>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
    {
        let query = "SELECT id FROM flower WHERE name_jp = $1";
        let result: Result<(i32,), sqlx::Error> =
            sqlx::query_as(query).bind(name).fetch_one(executor).await;

        match result {
            Ok((id,)) => Some(id),
            Err(_) => None,
        }
    }
}

pub async fn insert_flower<'a, E>(
    flower: &ModelFlower,
    executor: E,
) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let insert_flower =
        crate::infrastructure::db::sqlx::flower::InsertFlower::from_model_flower(flower);
    insert_flower.insert_flower(executor).await
}

pub async fn has_flower<'a, E>(
    flower: &ModelFlower,
    executor: E,
) -> Result<bool, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let insert_flower =
        crate::infrastructure::db::sqlx::flower::InsertFlower::from_model_flower(flower);
    insert_flower.has_flower(executor).await
}

pub async fn get_all_flowers(pool: &sqlx::SqlitePool) -> Result<Vec<ModelFlower>, AppError> {
    let repo = FlowerRepositorySqlite { pool: pool.clone() };
    repo.get_all_flowers().await
}

// ここから下はテストコード
#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    use super::*;
    use common_type::models::flowers::create_model_flower_from_name;

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

        assert!(
            result.is_ok(),
            "Failed to insert flower: {:?}",
            result.err()
        );

        let has_flower = has_flower(&flower, &pool).await;
        assert!(
            has_flower.is_ok(),
            "Failed to check if flower exists: {:?}",
            has_flower.err()
        );
        assert!(has_flower.unwrap(), "Flower should exist after insertion");
    }

    #[tokio::test]
    async fn test_insert_flower_duplicate() {
        let pool = setup_db().await;
        let flower = create_model_flower_from_name("Duplicate Flower");

        // Insert the flower for the first time
        let result = insert_flower(&flower, &pool).await;
        assert!(
            result.is_ok(),
            "Failed to insert flower: {:?}",
            result.err()
        );

        // Try to insert the same flower again
        let duplicate_result = insert_flower(&flower, &pool).await;
        assert!(
            duplicate_result.is_ok(),
            "Should not allow duplicate insertion"
        );

        let has_flower = has_flower(&flower, &pool).await;
        assert!(
            has_flower.is_ok(),
            "Failed to check if flower exists: {:?}",
            has_flower.err()
        );
        assert!(
            has_flower.unwrap(),
            "Flower should still exist after duplicate insertion"
        );

        // TODO: after implementing get_flower_by_name, check if the flower can be retrieved
    }
}
