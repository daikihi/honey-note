use common_type::models::flowers::Flower as ModelFlower;

#[derive(sqlx::FromRow, sqlx::Type, Debug, Clone)]
pub struct InsertFlower {
    pub name_jp: String,
    pub name_en: Option<String>,
    pub scientific_name: Option<String>,
    pub short_note: Option<String>, // description に相当する簡単な説明
    pub flower_type: Option<String>,
    pub image_path: Option<String>,
    pub note: Option<String>,
}

impl InsertFlower {
    pub async fn has_flower<'a, E>(self: &Self, executor: E) -> Result<bool, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
    {
        let flower_name_jp = self.name_jp.clone();
        let query = "SELECT EXISTS(SELECT 1 FROM flower WHERE name_jp = $1)";
        let exists: (i64,) = sqlx::query_as(query)
            .bind(flower_name_jp)
            .fetch_one(executor)
            .await?;
        Ok(exists.0 != 0)
    }

    pub async fn insert_flower<'a, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
    {
        let query = "INSERT INTO flower (name_jp, name_en, scientific_name, short_note, flower_type, image_path, note) VALUES ($1, $2, $3, $4, $5, $6, $7)";
        sqlx::query(query)
            .bind(&self.name_jp)
            .bind(&self.name_en)
            .bind(&self.scientific_name)
            .bind(&self.short_note)
            .bind(&self.flower_type)
            .bind(&self.image_path)
            .bind(&self.note)
            .execute(executor)
            .await
            .map(|_| ())
    }

    pub fn from_model_flower(model_flower: &ModelFlower) -> Self {
        InsertFlower {
            name_jp: model_flower.name_jp.clone(),
            name_en: model_flower.name_en.clone(),
            scientific_name: model_flower.scientific_name.clone(),
            short_note: model_flower.short_note.clone(),
            flower_type: model_flower.flower_type.clone(),
            image_path: model_flower.image_path.clone(),
            note: model_flower.note.clone(),
        }
    }
}

#[derive(sqlx::FromRow, sqlx::Type, Debug, Clone)]
pub struct Flower {
    pub id: Option<i32>,
    pub name_jp: String,
    pub name_en: Option<String>,
    pub scientific_name: Option<String>,
    pub short_note: Option<String>,
    pub flower_type: Option<String>,
    pub image_path: Option<String>,
    pub note: Option<String>,
}

impl Flower {
    pub fn to_model_flower(&self) -> ModelFlower {
        ModelFlower {
            id: self.id.map(|l| l as f64),
            name_jp: self.name_jp.clone(),
            name_en: self.name_en.clone(),
            scientific_name: self.scientific_name.clone(),
            short_note: self.short_note.clone(),
            flower_type: self.flower_type.clone(),
            image_path: self.image_path.clone(),
            note: self.note.clone(),
        }
    }

    pub async fn get_flower_by_id(
        id: i32,
        pool: &sqlx::SqlitePool,
    ) -> Result<Flower, sqlx::Error> {
        let query = "SELECT id, name_jp, name_en, scientific_name, short_note, flower_type, image_path, note FROM flower WHERE id = $1";
        sqlx::query_as::<_, Flower>(query)
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn exists_flower_by_id<'a, E>(id: i32, executor: E) -> Result<bool, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
    {
        let query = "SELECT EXISTS(SELECT 1 FROM flower WHERE id = $1)";
        let exists: (i64,) = sqlx::query_as(query).bind(id).fetch_one(executor).await?;
        Ok(exists.0 != 0)
    }

    pub async fn update<'a, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
    {
        let query = r#"
            UPDATE flower
            SET name_jp = ?, name_en = ?, scientific_name = ?, short_note = ?, flower_type = ?, image_path = ?, note = ?
            WHERE id = ?
        "#;
        sqlx::query(query)
            .bind(&self.name_jp)
            .bind(&self.name_en)
            .bind(&self.scientific_name)
            .bind(&self.short_note)
            .bind(&self.flower_type)
            .bind(&self.image_path)
            .bind(&self.note)
            .bind(self.id)
            .execute(executor)
            .await?;
        Ok(())
    }
}

pub async fn get_all_flowers(pool: &sqlx::SqlitePool) -> Result<Vec<Flower>, sqlx::Error> {
    // get all flowers from sqlite db
    let flowers: Result<Vec<Flower>, sqlx::Error> = sqlx::query_as::<_, Flower>(
        r#"
        SELECT id, name_jp, name_en, scientific_name , short_note, flower_type,
        image_path, note
        FROM flower
        "#,
    )
    .fetch_all(pool)
    .await;
    flowers
}
