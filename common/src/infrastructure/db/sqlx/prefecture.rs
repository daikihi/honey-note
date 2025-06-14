#[derive(sqlx::FromRow, sqlx::Type, Debug, Clone)]
pub struct Prefecture {
    pub id: i32,
    pub name_jp: String,
    pub name_en: String,
}

impl Prefecture {
    pub async fn has_prefecture(self: &Self, pool: &sqlx::SqlitePool) -> Result<bool, sqlx::Error> {
        let prefecture_name_jp = self.name_jp.clone();
        let query = "SELECT EXISTS(SELECT 1 FROM prefecture WHERE name_jp = $1)";
        let exists: (i64,) = sqlx::query_as(query)
            .bind(prefecture_name_jp)
            .fetch_one(pool)
            .await?;
        Ok(exists.0 != 0)
    }

    pub async fn get_all_prefectures(
        pool: &sqlx::SqlitePool,
    ) -> Result<Vec<Prefecture>, sqlx::Error> {
        println!("Fetching all prefectures from the database");
        let query = "SELECT id, name_jp, name_en FROM prefecture";
        let prefectures: Vec<Prefecture> = sqlx::query_as(query).fetch_all(pool).await?;
        Ok(prefectures)
    }

    // for writing to the database
    pub async fn insert_prefecture(&self, pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
        let query = "INSERT INTO prefecture (id, name_jp, name_en) VALUES ($1, $2, $3)";
        sqlx::query(query)
            .bind(self.id)
            .bind(&self.name_jp)
            .bind(&self.name_en)
            .execute(pool)
            .await
            .map(|_| ())
    }

    pub fn from_model(model: &crate::models::prefectures::Prefecture) -> Self {
        Prefecture {
            id: model.id,
            name_jp: model.name_jp.clone(),
            name_en: model.name_en.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    // test target model
    fn test_prefecture() -> Prefecture {
        Prefecture {
            id: 1,
            name_jp: "東京都".to_string(),
            name_en: "Tokyo".to_string(),
        }
    }

    // craete database and table for testing
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
        let _pool = setup_db().await;
        let _test_prefectue = test_prefecture();

        // before inserting, check if it does not exist
        let exists = _test_prefectue.has_prefecture(&_pool).await.unwrap();
        assert!(!exists, "Prefecture should not exist before insertion");

        // insert the prefecture
        _test_prefectue.insert_prefecture(&_pool).await.unwrap();
        // after inserting, check if it exists
        let exists = _test_prefectue.has_prefecture(&_pool).await.unwrap();
        assert!(exists, "Prefecture should exist after insertion");

        // try to insert again, it should fail due to PRIMARY KEY constraint
        let result = _test_prefectue.insert_prefecture(&_pool).await;
        assert!(
            result.is_err(),
            "Inserting duplicate prefecture should return an error"
        ); // check again, it should still exist
        let exists = _test_prefectue.has_prefecture(&_pool).await.unwrap();
        assert!(exists, "Prefecture should still exist after re-insertion");
    }

    #[tokio::test]
    async fn test_get_all_prefectures() {
        let _pool = setup_db().await;
        let _test_prefecture = test_prefecture();

        // insert the prefecture
        _test_prefecture.insert_prefecture(&_pool).await.unwrap();

        // get all prefectures
        let prefectures = Prefecture::get_all_prefectures(&_pool).await.unwrap();
        assert_eq!(
            prefectures.len(),
            1,
            "There should be one prefecture in the database"
        );
        assert_eq!(
            prefectures[0].name_jp, _test_prefecture.name_jp,
            "The prefecture name should match"
        );
    }
}
