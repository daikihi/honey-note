use crate::infrastructure::db::sqlx::prefecture;
use crate::models::prefectures::Prefecture as PrefectureModel;
use log::info;

pub async fn has_prefecture(
    _model_prefecture: &PrefectureModel,
    pool: &sqlx::SqlitePool,
) -> Result<bool, sqlx::Error> {
    let db_prefecture: prefecture::Prefecture =
        prefecture::Prefecture::from_model(&PrefectureModel {
            id: 0, // ID is not used in this check
            name_jp: _model_prefecture.name_jp.clone(),
            name_en: _model_prefecture.name_en.clone(), // English name is not used in this check
        });

    for_logging(&db_prefecture, "Checking if prefecture exists in database");

    let result = db_prefecture.has_prefecture(pool).await;
    // @todo should not return sqlx::Error to application layer
    result
}

pub async fn get_all_prefectures(
    pool: &sqlx::SqlitePool,
) -> Result<Vec<PrefectureModel>, sqlx::Error> {
    let db_prefectures = prefecture::Prefecture::get_all_prefectures(pool).await;
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
        Err(e) => Err(e),
    }
}

pub async fn insert_prefecture(_model_prefecture: &PrefectureModel, pool: &sqlx::SqlitePool) {
    let db_prefecture: prefecture::Prefecture =
        prefecture::Prefecture::from_model(_model_prefecture);
    let _cloned = db_prefecture.clone();
    let msg: String = format!(
        "Inserting prefecture into database: id={}, name_jp={}, name_en={}",
        _cloned.id, _cloned.name_jp, _cloned.name_en
    );
    for_logging(&db_prefecture, msg.as_str());

    db_prefecture
        .insert_prefecture(pool)
        .await
        .expect("Failed to insert prefecture ");
}

fn for_logging(db_prefecture: &prefecture::Prefecture, msg: &str) {
    info!("{}, {:?}", msg, db_prefecture);
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
