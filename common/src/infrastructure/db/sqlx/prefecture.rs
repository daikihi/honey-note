

#[derive(sqlx::FromRow, sqlx::Type, Debug, Clone)]
pub struct Prefecture {
    pub id: i32,
    pub name_jp: String,
    pub name_en: String,
}

impl Prefecture {
    pub async fn has_prefecture(
        self: &Self,
        pool: &sqlx::SqlitePool
    ) -> Result<bool, sqlx::Error> {
        let prefecture_name_jp = self.name_jp.clone();
        let query = "SELECT EXISTS(SELECT 1 FROM prefecture WHERE name_jp = $1)";
        let exists: (i64,) = sqlx::query_as(query)
            .bind(prefecture_name_jp)
            .fetch_one(pool)
            .await?;
        Ok(exists.0 != 0)
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
