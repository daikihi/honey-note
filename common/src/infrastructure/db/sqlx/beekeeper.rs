use common_type::models::beekeeper::Beekeeper as ModelBeekeeper;

// for select and update
#[derive(Debug, sqlx::FromRow, Clone)]
pub struct Beekeeper {
    pub id: i32,
    pub name_jp: String,
    pub name_en: Option<String>,
    pub founding_year: Option<i32>, // 創業・設立された西暦年
    pub location_prefecture_id: Option<i32>,
    pub location_city: Option<String>,
    pub website_url: Option<String>,
    pub note: Option<String>,
}

impl Beekeeper {
    pub fn to_model_beekeeper(&self) -> ModelBeekeeper {
        ModelBeekeeper {
            id: Some(self.id),
            name_jp: self.name_jp.clone(),
            name_en: self.name_en.clone(),
            founding_year: self.founding_year,
            location_prefecture_id: self.location_prefecture_id,
            location_city: self.location_city.clone(),
            website_url: self.website_url.clone(),
            note: self.note.clone(),
        }
    }

    pub async fn get_beekeeper_id_by_name(name: &str, pool: &sqlx::SqlitePool) -> Option<i32> {
        let query = "SELECT id FROM beekeeper WHERE name_jp = $1";
        let result: Result<(i32,), sqlx::Error> =
            sqlx::query_as(query).bind(name).fetch_one(pool).await;

        match result {
            Ok((id,)) => Some(id),
            Err(_) => None,
        }
    }

    pub async fn get_all_beekeepers(
        pool: &sqlx::SqlitePool,
    ) -> Result<Vec<Beekeeper>, sqlx::Error> {
        let query = "SELECT id, name_jp, name_en, founding_year, location_prefecture_id, location_city, website_url, note FROM beekeeper";
        let beekeepers: Result<Vec<Beekeeper>, sqlx::Error> =
            sqlx::query_as::<_, Beekeeper>(query).fetch_all(pool).await;
        beekeepers
    }
}

#[derive(Debug, sqlx::FromRow)]
// for new insert
pub struct BeekeeperForInsert {
    pub name_jp: String,
    pub name_en: Option<String>,
    pub founding_year: Option<i32>, // 創業・設立された西暦年
    pub location_prefecture_id: Option<i32>,
    pub location_city: Option<String>,
    pub website_url: Option<String>,
    pub note: Option<String>,
}

impl BeekeeperForInsert {
    pub fn new(model_beekeeper: &ModelBeekeeper) -> Self {
        BeekeeperForInsert {
            name_jp: model_beekeeper.name_jp.clone(),
            name_en: model_beekeeper.name_en.clone(),
            founding_year: model_beekeeper.founding_year,
            location_prefecture_id: model_beekeeper.location_prefecture_id,
            location_city: model_beekeeper.location_city.clone(),
            website_url: model_beekeeper.website_url.clone(),
            note: model_beekeeper.note.clone(),
        }
    }

    pub async fn has_beekeeper(&self, pool: &sqlx::SqlitePool) -> Result<bool, sqlx::Error> {
        let query = "SELECT EXISTS(SELECT 1 FROM beekeeper WHERE name_jp = $1)";
        let exists: (i64,) = sqlx::query_as(query)
            .bind(&self.name_jp)
            .fetch_one(pool)
            .await?;
        Ok(exists.0 != 0)
    }

    pub async fn insert_beekeeper(&self, pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
        let query = "INSERT INTO beekeeper (name_jp, name_en, founding_year, location_prefecture_id, location_city, website_url, note) VALUES ($1, $2, $3, $4, $5, $6, $7)";
        sqlx::query(query)
            .bind(&self.name_jp)
            .bind(&self.name_en)
            .bind(self.founding_year)
            .bind(self.location_prefecture_id)
            .bind(&self.location_city)
            .bind(&self.website_url)
            .bind(&self.note)
            .execute(pool)
            .await
            .map(|_| ())
    }

    // should be implemented in Beekeeper not in BeekeeperForInsert
    pub async fn get_all_beekeepers(
        pool: &sqlx::SqlitePool,
    ) -> Result<Vec<Beekeeper>, sqlx::Error> {
        let query = "SELECT id, name_jp, name_en, founding_year, location_prefecture_id, location_city, website_url, note FROM beekeeper";
        let beekeepers: Result<Vec<Beekeeper>, sqlx::Error> =
            sqlx::query_as::<_, Beekeeper>(query).fetch_all(pool).await;
        beekeepers
    }
}
