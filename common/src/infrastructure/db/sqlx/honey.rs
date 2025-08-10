use common_type::models::flowers::Flower;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, sqlx::Type, Clone)]
pub struct Honey {
    pub id: Option<i32>,                // 自動生成される主キー（nullableの場合）
    pub name_jp: String,                // NOT NULLなのでOptionなし
    pub name_en: Option<String>,        // NULL可能な場合はOptionでラップ
    pub beekeeper_id: Option<i32>,      // 外部キー（NULL可能と仮定）
    pub origin_country: Option<String>, // NULL可能なカラム
    pub origin_region: Option<String>,  // NULL可能なカラム
    pub harvest_year: Option<i32>,      // 採蜜年（NULL可能）
    pub purchase_date: Option<String>,  // 購入日（文字列として扱う場合、Date型はクレートを使う）
    pub note: Option<String>,           // 補足（NULL可能）
}

impl Honey {
    pub fn new_for_simple_insert(id: i32, name_jp: String) -> Self {
        Honey {
            id: Some(id),
            name_jp,
            name_en: None,
            beekeeper_id: None,
            origin_country: None,
            origin_region: None,
            harvest_year: None,
            purchase_date: None,
            note: None,
        }
    }

    pub async fn is_exist_by_name(&self, pool: &sqlx::SqlitePool) -> Result<bool, sqlx::Error> {
        let query = "SELECT id FROM honey WHERE name_jp = ?";
        let result = sqlx::query_as::<_, (i32,)>(query)
            .bind(&self.name_jp)
            .fetch_optional(pool)
            .await?;

        Ok(result.is_some())
    }

    pub async fn find_by_id(id: i32, name: String, pool: &sqlx::SqlitePool) -> Option<Self> {
        let query = "SELECT * FROM honey WHERE id = ? AND name_jp = ?";
        let result = sqlx::query_as::<_, Honey>(query)
            .bind(id)
            .bind(name)
            .fetch_optional(&*pool)
            .await;

        match result {
            Ok(Some(honey)) => Some(honey),
            Ok(None) => None,
            Err(_) => None, // エラー処理は適宜実装
        }
    }

    pub async fn insert(&self, pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
        let query = r#"        
            INSERT INTO honey (id, name_jp, name_en, beekeeper_id, origin_country, origin_region, harvest_year, purchase_date, note)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;

        let _ = sqlx::query(query)
            .bind(self.id)
            .bind(&self.name_jp)
            .bind(&self.name_en)
            .bind(self.beekeeper_id)
            .bind(&self.origin_country)
            .bind(&self.origin_region)
            .bind(self.harvest_year)
            .bind(&self.purchase_date)
            .bind(&self.note)
            .execute(pool).await;
        Ok(())
    }

}

pub async fn get_all(pool: &sqlx::SqlitePool) ->Result<Vec<Honey>, sqlx::Error>{
    let honeies: Result<Vec<Honey>, sqlx::Error> = sqlx::query_as::<_, Honey>(
        r#"SELECT id, name_jp, name_en, beekeeper_id, origin_country, origin_region, harvest_year, purchase_date, note
        FROM honey"#,
    ).fetch_all(pool).await;
    honeies
}

use crate::infrastructure::db::sqlx::beekeeper::Beekeeper as SqlBeekeeper;
use common_type::models::beekeeper::Beekeeper as ModelBeekeeper;
use common_type::models::honey::Honey as ModelHoney;

pub fn create_model_honey(sql_honey: Honey, beekeeper: Option<SqlBeekeeper>) -> ModelHoney {
    let _beekeeper: Option<ModelBeekeeper> = beekeeper.map(|b| ModelBeekeeper {
        id: Some(b.id),
        name_jp: b.name_jp,
        name_en: b.name_en,
        founding_year: b.founding_year,
        location_prefecture_id: b.location_prefecture_id,
        location_city: b.location_city,
        website_url: b.website_url,
        note: b.note,
    });

    ModelHoney {
        id: sql_honey.id, // デフォルト値を設定
        name_jp: sql_honey.name_jp,
        name_en: sql_honey.name_en,
        beekkeeper: _beekeeper, // 外部キーとしてBeekeeperを参照
        origin_country: sql_honey.origin_country,
        origin_region: sql_honey.origin_region,
        harvest_year: sql_honey.harvest_year,
        purchase_date: sql_honey.purchase_date,
        note: sql_honey.note,
    }
}

// TEST
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_model_honey() {
        let sql_honey = Honey {
            id: Some(1),
            name_jp: "アカシアはちみつ".to_string(),
            name_en: Some("Acacia Honey".to_string()),
            beekeeper_id: Some(2),
            origin_country: Some("日本".to_string()),
            origin_region: Some("北海道".to_string()),
            harvest_year: Some(2024),
            purchase_date: Some("2025-08-09".to_string()),
            note: Some("とても美味しい蜂蜜です".to_string()),
        };

        let sql_beekeeper = Some(SqlBeekeeper {
            id: 2,
            name_jp: "山田養蜂場".to_string(),
            name_en: Some("Yamada Bee Farm".to_string()),
            founding_year: Some(1950),
            location_prefecture_id: Some(1),
            location_city: Some("札幌市".to_string()),
            website_url: Some("https://yamadabeefarm.jp".to_string()),
            note: Some("信頼できる養蜂場".to_string()),
        });

        // 期待される出力データ
        let expected_honey: ModelHoney = ModelHoney {
            id: Some(1),
            name_jp: "アカシアはちみつ".to_string(),
            name_en: Some("Acacia Honey".to_string()),
            beekkeeper: Some(ModelBeekeeper {
                id: Some(2),
                name_jp: "山田養蜂場".to_string(),
                name_en: Some("Yamada Bee Farm".to_string()),
                founding_year: Some(1950),
                location_prefecture_id: Some(1),
                location_city: Some("札幌市".to_string()),
                website_url: Some("https://yamadabeefarm.jp".to_string()),
                note: Some("信頼できる養蜂場".to_string()),
            }),
            origin_country: Some("日本".to_string()),
            origin_region: Some("北海道".to_string()),
            harvest_year: Some(2024),
            purchase_date: Some("2025-08-09".to_string()),
            note: Some("とても美味しい蜂蜜です".to_string()),
        };

        // 実際の関数呼び出し
        let result_honey: ModelHoney = create_model_honey(sql_honey, sql_beekeeper);

        // 結果の比較
        assert_eq!(result_honey, expected_honey);
    }
}
