use crate::errors::AppError;
use crate::infrastructure::db::sqlx::beekeeper::Beekeeper as SqlBeekeeper;
use crate::infrastructure::db::sqlx::beekeeper::BeekeeperForInsert;
use common_type::models::beekeeper::Beekeeper as ModelBeekeeper;
use log::error;

pub trait BeekeeperRepository: Send + Sync {
    async fn get_all_beekeepers(&self) -> Result<Vec<ModelBeekeeper>, AppError>;
    async fn get_beekeeper_id_by_name<'a, E>(&self, name: &str, executor: E) -> Option<i32>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>;
    async fn insert_beekeeper<'a, E>(&self, beekeeper: &ModelBeekeeper, executor: E) -> Result<(), AppError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>;
    async fn has_beekeeper<'a, E>(&self, beekeeper: &ModelBeekeeper, executor: E) -> bool
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>;
}

pub struct BeekeeperRepositorySqlite {
    pub pool: sqlx::SqlitePool,
}

impl BeekeeperRepository for BeekeeperRepositorySqlite {
    async fn get_all_beekeepers(&self) -> Result<Vec<ModelBeekeeper>, AppError> {
        let beekeepers = BeekeeperForInsert::get_all_beekeepers(&self.pool).await;
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

    async fn get_beekeeper_id_by_name<'a, E>(&self, name: &str, executor: E) -> Option<i32>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
    {
        SqlBeekeeper::get_beekeeper_id_by_name(name, executor).await
    }

    async fn insert_beekeeper<'a, E>(&self, beekeeper: &ModelBeekeeper, executor: E) -> Result<(), AppError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
    {
        let inserted_beekeeper = BeekeeperForInsert::new(beekeeper);
        inserted_beekeeper
            .insert_beekeeper(executor)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    async fn has_beekeeper<'a, E>(&self, beekeeper: &ModelBeekeeper, executor: E) -> bool
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
    {
        let inserted_beekeeper = BeekeeperForInsert::new(beekeeper);
        inserted_beekeeper
            .has_beekeeper(executor)
            .await
            .unwrap_or_else(|e| {
                error!("Error checking beekeeper existence: {}", e);
                false
            })
    }
}

pub async fn has_beekeeper<'a, E>(
    beekeeper: &ModelBeekeeper,
    executor: E,
) -> bool
where
    E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let inserted_beekeeper = BeekeeperForInsert::new(beekeeper);
    inserted_beekeeper
        .has_beekeeper(executor)
        .await
        .unwrap_or_else(|e| {
            error!("Error checking beekeeper existence: {}", e);
            false
        })
}

pub async fn insert_beekeeper<'a, E>(
    beekeeper: &ModelBeekeeper,
    executor: E,
) -> Result<(), AppError>
where
    E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    let inserted_beekeeper = BeekeeperForInsert::new(beekeeper);
    inserted_beekeeper
        .insert_beekeeper(executor)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

pub async fn get_beekeeper_id_by_name<'a, E>(
    name: &str,
    executor: E,
) -> Option<i32>
where
    E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
{
    SqlBeekeeper::get_beekeeper_id_by_name(name, executor).await
}

pub async fn get_all_beekeepers(pool: &sqlx::SqlitePool) -> Result<Vec<ModelBeekeeper>, AppError> {
    let repo = BeekeeperRepositorySqlite { pool: pool.clone() };
    repo.get_all_beekeepers().await
}
