use crate::errors::AppError;
use crate::infrastructure::db::sqlx::beekeeper::Beekeeper as SqlBeekeeper;
use crate::infrastructure::db::sqlx::beekeeper::BeekeeperForInsert;
use common_type::models::beekeeper::Beekeeper as ModelBeekeeper;
use log::error;

pub trait BeekeeperRepository: Send + Sync {
    async fn get_all_beekeepers(&self, user_id: i32) -> Result<Vec<ModelBeekeeper>, AppError>;
    async fn get_beekeeper_by_id(&self, id: i32, user_id: i32) -> Result<ModelBeekeeper, AppError>;
    async fn update_beekeeper<'a, E>(
        &self,
        id: i32,
        beekeeper: &ModelBeekeeper,
        user_id: i32,
        executor: E,
    ) -> Result<(), AppError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>;
    async fn exists_beekeeper_by_id<'a, E>(&self, id: i32, user_id: i32, executor: E) -> Result<bool, AppError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>;
    async fn get_beekeeper_id_by_name<'a, E>(&self, name: &str, user_id: i32, executor: E) -> Option<i32>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>;
    async fn insert_beekeeper<'a, E>(&self, beekeeper: &ModelBeekeeper, user_id: i32, executor: E) -> Result<(), AppError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>;
    async fn has_beekeeper<'a, E>(&self, beekeeper: &ModelBeekeeper, user_id: i32, executor: E) -> bool
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>;
}

pub struct BeekeeperRepositorySqlite {
    pub pool: sqlx::SqlitePool,
}

impl BeekeeperRepository for BeekeeperRepositorySqlite {
    async fn get_all_beekeepers(&self, user_id: i32) -> Result<Vec<ModelBeekeeper>, AppError> {
        let result: Result<Vec<SqlBeekeeper>, sqlx::Error> = sqlx::query_as("SELECT * FROM beekeeper WHERE user_id = ?")
            .bind(user_id)
            .fetch_all(&self.pool)
            .await;
        
        match result {
            Ok(beekeepers) => Ok(beekeepers
                .into_iter()
                .map(|b| b.to_model_beekeeper())
                .collect()),
            Err(e) => {
                error!("Error fetching all beekeepers: {}", e);
                Err(AppError::DatabaseError(e.to_string()))
            }
        }
    }

    async fn get_beekeeper_id_by_name<'a, E>(&self, name: &str, user_id: i32, executor: E) -> Option<i32>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
    {
        let query = "SELECT id FROM beekeeper WHERE name_jp = ? AND user_id = ?";
        let result: Result<(i32,), sqlx::Error> =
            sqlx::query_as(query).bind(name).bind(user_id).fetch_one(executor).await;

        match result {
            Ok((id,)) => Some(id),
            Err(_) => None,
        }
    }

    async fn get_beekeeper_by_id(&self, id: i32, user_id: i32) -> Result<ModelBeekeeper, AppError> {
        let result: Result<SqlBeekeeper, sqlx::Error> = sqlx::query_as("SELECT * FROM beekeeper WHERE id = ? AND user_id = ?")
            .bind(id)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await;
        match result {
            Ok(b) => Ok(b.to_model_beekeeper()),
            Err(e) => Err(AppError::DatabaseError(e.to_string())),
        }
    }

    async fn update_beekeeper<'a, E>(
        &self,
        id: i32,
        beekeeper: &ModelBeekeeper,
        user_id: i32,
        executor: E,
    ) -> Result<(), AppError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
    {
        // 所有権チェック
        let exists: Option<(i32,)> = sqlx::query_as("SELECT 1 FROM beekeeper WHERE id = ? AND user_id = ?")
            .bind(id)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        if exists.is_none() {
            return Err(AppError::NotFound("指定された養蜂業者が見つからないか、権限がありません".to_string()));
        }

        let sqlx_bk = SqlBeekeeper {
            id,
            name_jp: beekeeper.name_jp.clone(),
            name_en: beekeeper.name_en.clone(),
            founding_year: beekeeper.founding_year,
            location_prefecture_id: beekeeper.location_prefecture_id,
            location_city: beekeeper.location_city.clone(),
            website_url: beekeeper.website_url.clone(),
            user_id: Some(user_id),
            note: beekeeper.note.clone(),
        };
        sqlx_bk
            .update(executor)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    async fn exists_beekeeper_by_id<'a, E>(&self, id: i32, user_id: i32, executor: E) -> Result<bool, AppError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
    {
        let query = "SELECT EXISTS(SELECT 1 FROM beekeeper WHERE id = ? AND user_id = ?)";
        let result: (i64,) = sqlx::query_as(query)
            .bind(id)
            .bind(user_id)
            .fetch_one(executor)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(result.0 != 0)
    }

    async fn insert_beekeeper<'a, E>(&self, beekeeper: &ModelBeekeeper, user_id: i32, executor: E) -> Result<(), AppError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
    {
        let mut inserted_beekeeper = BeekeeperForInsert::new(beekeeper);
        inserted_beekeeper.user_id = Some(user_id);
        inserted_beekeeper
            .insert_beekeeper(executor)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    async fn has_beekeeper<'a, E>(&self, beekeeper: &ModelBeekeeper, user_id: i32, executor: E) -> bool
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
    {
        let query = "SELECT EXISTS(SELECT 1 FROM beekeeper WHERE name_jp = ? AND user_id = ?)";
        let result: (i64,) = sqlx::query_as(query)
            .bind(&beekeeper.name_jp)
            .bind(user_id)
            .fetch_one(executor)
            .await
            .unwrap_or((0,));
        result.0 != 0
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
