use crate::errors::AppError;
use crate::infrastructure::db::sqlx::prefecture;
use common_type::models::prefectures::Prefecture as PrefectureModel;
use log::info;

pub trait PrefectureRepository: Send + Sync {
    async fn get_all_prefectures(&self) -> Result<Vec<PrefectureModel>, AppError>;
    async fn get_prefecture_by_name(&self, name: &str) -> Result<PrefectureModel, AppError>;
    async fn insert_prefecture(&self, model: &PrefectureModel) -> Result<(), AppError>;
    async fn has_prefecture(&self, model: &PrefectureModel) -> Result<bool, AppError>;
}

pub struct PrefectureRepositorySqlite {
    pub pool: sqlx::SqlitePool,
}

impl PrefectureRepository for PrefectureRepositorySqlite {
    async fn get_all_prefectures(&self) -> Result<Vec<PrefectureModel>, AppError> {
        let db_prefectures = prefecture::Prefecture::get_all_prefectures(&self.pool).await;
        match db_prefectures {
            Ok(prefectures) => {
                let model_prefectures: Vec<PrefectureModel> = prefectures
                    .into_iter()
                    .map(|db_pref| PrefectureModel {
                        id: db_pref.id,
                        name_jp: db_pref.name_jp,
                        name_en: db_pref.name_en,
                    })
                    .collect();
                Ok(model_prefectures)
            }
            Err(e) => Err(AppError::DatabaseError(e.to_string())),
        }
    }

    async fn get_prefecture_by_name(&self, name: &str) -> Result<PrefectureModel, AppError> {
        let db_prefecture = prefecture::Prefecture::get_prefecture_by_name(name, &self.pool).await;
        match db_prefecture {
            Ok(pref) => Ok(pref.to_model()),
            Err(e) => {
                info!("Error getting prefecture ID: {}", e);
                Err(AppError::DatabaseError(e.to_string()))
            }
        }
    }

    async fn insert_prefecture(&self, model: &PrefectureModel) -> Result<(), AppError> {
        let db_prefecture: prefecture::Prefecture = prefecture::Prefecture::from_model(model.clone());
        db_prefecture
            .insert_prefecture(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    async fn has_prefecture(&self, model: &PrefectureModel) -> Result<bool, AppError> {
        let db_prefecture: prefecture::Prefecture = prefecture::Prefecture::from_model(PrefectureModel {
            id: 0,
            name_jp: model.name_jp.clone(),
            name_en: model.name_en.clone(),
        });
        db_prefecture
            .has_prefecture(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }
}

pub async fn has_prefecture(
    _model_prefecture: &PrefectureModel,
    pool: &sqlx::SqlitePool,
) -> Result<bool, AppError> {
    let repo = PrefectureRepositorySqlite { pool: pool.clone() };
    repo.has_prefecture(_model_prefecture).await
}

pub async fn get_all_prefectures(
    pool: &sqlx::SqlitePool,
) -> Result<Vec<PrefectureModel>, AppError> {
    let repo = PrefectureRepositorySqlite { pool: pool.clone() };
    repo.get_all_prefectures().await
}

pub async fn insert_prefecture(_model_prefecture: &PrefectureModel, pool: &sqlx::SqlitePool) {
    let repo = PrefectureRepositorySqlite { pool: pool.clone() };
    let _ = repo.insert_prefecture(_model_prefecture).await;
}

pub async fn get_prefecture_by_name(
    prefecture_name: &str,
    pool: &sqlx::SqlitePool,
) -> Result<PrefectureModel, AppError> {
    let repo = PrefectureRepositorySqlite { pool: pool.clone() };
    repo.get_prefecture_by_name(prefecture_name).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    // Test target model
    fn test_prefecture() -> PrefectureModel {
        PrefectureModel {
            id: 1,
            name_jp: "東京都".to_string(),
            name_en: "Tokyo".to_string(),
        }
    }

    // Create database and table for testing
    async fn setup_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::query(
            "CREATE TABLE prefecture (
                id INTEGER PRIMARY KEY,
                name_jp TEXT NOT NULL,
                name_en TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();
        pool
    }

    #[tokio::test]
    async fn test_insert_and_has_prefecture() {
        // 都道府県情報の登録と、登録した情報の存在確認が正しく行えることをテストする
        // Test that prefecture information can be inserted and its existence can be correctly verified.
        let pool = setup_db().await;
        let test_prefecture = test_prefecture();

        // Before inserting, check if it does not exist
        let exists = has_prefecture(&test_prefecture, &pool).await.unwrap();
        assert!(!exists, "Prefecture should not exist before insertion");

        // Insert the prefecture
        insert_prefecture(&test_prefecture, &pool).await;

        // After inserting, check if it exists
        let exists = has_prefecture(&test_prefecture, &pool).await.unwrap();
        assert!(exists, "Prefecture should exist after insertion");
    }
}
