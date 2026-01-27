use crate::errors::AppError;
use crate::infrastructure::db::sqlx::beekeeper::Beekeeper as SqlBeekeeper;
use crate::infrastructure::db::sqlx::beekeeper::BeekeeperForInsert;
use common_type::models::beekeeper::Beekeeper as ModelBeekeeper;
use log::error;

pub trait BeekeeperRepository: Send + Sync {
    async fn get_all_beekeepers(&self) -> Result<Vec<ModelBeekeeper>, AppError>;
    async fn get_beekeeper_id_by_name(&self, name: &str) -> Option<i32>;
    async fn insert_beekeeper(&self, beekeeper: &ModelBeekeeper) -> Result<(), AppError>;
    async fn has_beekeeper(&self, beekeeper: &ModelBeekeeper) -> bool;
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

    async fn get_beekeeper_id_by_name(&self, name: &str) -> Option<i32> {
        SqlBeekeeper::get_beekeeper_id_by_name(name, &self.pool).await
    }

    async fn insert_beekeeper(&self, beekeeper: &ModelBeekeeper) -> Result<(), AppError> {
        let inserted_beekeeper = BeekeeperForInsert::new(beekeeper);
        inserted_beekeeper
            .insert_beekeeper(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    async fn has_beekeeper(&self, beekeeper: &ModelBeekeeper) -> bool {
        let inserted_beekeeper = BeekeeperForInsert::new(beekeeper);
        inserted_beekeeper
            .has_beekeeper(&self.pool)
            .await
            .unwrap_or_else(|e| {
                error!("Error checking beekeeper existence: {}", e);
                false
            })
    }
}

pub async fn has_beekeeper(beekeeper: &ModelBeekeeper, pool: &sqlx::SqlitePool) -> bool {
    let repo = BeekeeperRepositorySqlite { pool: pool.clone() };
    repo.has_beekeeper(beekeeper).await
}

pub async fn insert_beekeeper(beekeeper: &ModelBeekeeper, pool: &sqlx::SqlitePool) {
    let repo = BeekeeperRepositorySqlite { pool: pool.clone() };
    let _ = repo.insert_beekeeper(beekeeper).await;
}

pub async fn get_beekeeper_id_by_name(name: &str, pool: &sqlx::SqlitePool) -> Option<i32> {
    let repo = BeekeeperRepositorySqlite { pool: pool.clone() };
    repo.get_beekeeper_id_by_name(name).await
}

pub async fn get_all_beekeepers(pool: &sqlx::SqlitePool) -> Result<Vec<ModelBeekeeper>, AppError> {
    let repo = BeekeeperRepositorySqlite { pool: pool.clone() };
    repo.get_all_beekeepers().await
}
